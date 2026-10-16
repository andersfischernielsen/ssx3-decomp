//! The PS2 memory card filesystem, natively: what the IOP's MCMAN and
//! MCSERV do for libmc, on the raw card image (`0x4000` pages of 512 data +
//! 16 spare bytes, the layout of PCSX2's `.ps2` files and of
//! `ssx3_iop::memcard`).
//!
//! Layout of a formatted 8 MB card (as MCMAN writes it):
//! - page 0: the superblock ("Sony PS2 Memory Card Format 1.2.0.0", page
//!   and cluster geometry, `alloc_offset`, `alloc_end`, the root cluster,
//!   the backup blocks, the indirect FAT cluster list);
//! - clusters (2 pages, 1 KB) from `alloc_offset` on hold files and
//!   directories; their numbers are relative to `alloc_offset`;
//! - the FAT: `ifc_list[i]` → indirect clusters → FAT clusters → one `u32`
//!   per cluster: bit 31 set = allocated, low 31 bits = next cluster,
//!   `0xffff_ffff` = end of chain, `0x7fff_ffff` = free;
//! - a directory is a chain of 512-byte entries ("." and ".." first). Its
//!   entry count is in the entry that describes it: the root's own "."
//!   entry, a subdirectory's entry in its parent (found through the
//!   subdirectory's "." entry: cluster = the parent's, `dir_entry` = the
//!   index there). Deleted entries keep their slot (exists bit clear).
//!
//! Results use libmc's codes (`sceMcRes*`).
//!
//! Checked against the recompiled MCMAN/MCSERV: the same save made through
//! both gives the same filesystem (entries, allocation, contents, free
//! space). The one difference is MCMAN's write journal: it copies what it
//! is about to change into backup block 1 (block 1023) and lists it in
//! backup block 2 (1022), which it erases when the operation is complete.
//! A finished operation leaves block 1022 erased either way, so MCMAN never
//! replays block 1023; native writes are not journaled.

pub mod ecc;

pub const PAGE: usize = 512;
pub const RAW: usize = 528;
pub const PAGES: usize = 0x4000;
pub const CARD_BYTES: usize = RAW * PAGES;
pub const CLUSTER: usize = 1024;
const ENTRY: usize = 512;

pub const FAT_FREE: u32 = 0x7fff_ffff;
pub const FAT_EOC: u32 = 0xffff_ffff;
const ALLOCATED: u32 = 0x8000_0000;

// Entry mode bits.
pub const M_READ: u16 = 0x0001;
pub const M_WRITE: u16 = 0x0002;
pub const M_EXEC: u16 = 0x0004;
pub const M_FILE: u16 = 0x0010;
pub const M_DIR: u16 = 0x0020;
pub const M_CLOSED: u16 = 0x0080;
pub const M_0400: u16 = 0x0400;
pub const M_EXISTS: u16 = 0x8000;

// libmc results.
pub const OK: i32 = 0;
pub const NO_FORMAT: i32 = -2;
pub const FULL: i32 = -3;
pub const NO_ENTRY: i32 = -4;
pub const DENIED: i32 = -5;
pub const NOT_EMPTY: i32 = -6;
pub const UP_LIMIT_HANDLE: i32 = -7;

// Open flags.
pub const O_RDONLY: u32 = 0x0001;
pub const O_WRONLY: u32 = 0x0002;
pub const O_MKDIR: u32 = 0x0040;
pub const O_CREAT: u32 = 0x0200;
pub const O_TRUNC: u32 = 0x0400;

const MAX_FILES: usize = 16;

/// A card time: `[0, sec, min, hour, day, month, year lo, year hi]` (JST).
pub type Tod = [u8; 8];

pub fn tod(sec: u8, min: u8, hour: u8, day: u8, month: u8, year: u16) -> Tod {
    let y = year.to_le_bytes();
    [0, sec, min, hour, day, month, y[0], y[1]]
}

/// The raw card: pages and their ECC, with the pages changed since the last
/// [`Card::take_dirty`].
pub struct Card {
    pub img: Vec<u8>,
    dirty: Vec<usize>,
}

impl Card {
    /// An erased (unformatted) card.
    pub fn erased() -> Card {
        Card {
            img: vec![0xff; CARD_BYTES],
            dirty: Vec::new(),
        }
    }

    pub fn from_image(mut img: Vec<u8>) -> Card {
        img.resize(CARD_BYTES, 0xff);
        Card {
            img,
            dirty: Vec::new(),
        }
    }

    pub fn page(&self, n: usize) -> &[u8] {
        &self.img[n * RAW..n * RAW + PAGE]
    }

    pub fn write_page(&mut self, n: usize, data: &[u8]) {
        let at = n * RAW;
        self.img[at..at + PAGE].copy_from_slice(&data[..PAGE]);
        self.img[at + PAGE..at + RAW].copy_from_slice(&ecc::spare(&data[..PAGE]));
        self.dirty.push(n);
    }

    pub fn erase_page(&mut self, n: usize) {
        self.img[n * RAW..(n + 1) * RAW].fill(0xff);
        self.dirty.push(n);
    }

    pub fn cluster(&self, abs: u32) -> Vec<u8> {
        let p = abs as usize * 2;
        [self.page(p), self.page(p + 1)].concat()
    }

    pub fn write_cluster(&mut self, abs: u32, data: &[u8]) {
        let p = abs as usize * 2;
        self.write_page(p, &data[..PAGE]);
        self.write_page(p + 1, &data[PAGE..CLUSTER]);
    }

    /// Pages changed since the last call (sorted, unique).
    pub fn take_dirty(&mut self) -> Vec<usize> {
        let mut d = std::mem::take(&mut self.dirty);
        d.sort_unstable();
        d.dedup();
        d
    }
}

fn u16_at(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn u32_at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

fn put32(b: &mut [u8], o: usize, v: u32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

#[derive(Clone, Debug)]
pub struct Super {
    pub clusters_per_card: u32,
    pub alloc_offset: u32,
    pub alloc_end: u32,
    pub root: u32,
    pub ifc: [u32; 32],
}

const MAGIC: &[u8] = b"Sony PS2 Memory Card Format ";

impl Super {
    fn parse(p: &[u8]) -> Option<Super> {
        if &p[..MAGIC.len()] != MAGIC || u16_at(p, 0x28) as usize != PAGE || u16_at(p, 0x2a) != 2 {
            return None;
        }
        let mut ifc = [0; 32];
        for (i, v) in ifc.iter_mut().enumerate() {
            *v = u32_at(p, 0x50 + 4 * i);
        }
        Some(Super {
            clusters_per_card: u32_at(p, 0x30),
            alloc_offset: u32_at(p, 0x34),
            alloc_end: u32_at(p, 0x38),
            root: u32_at(p, 0x3c),
            ifc,
        })
    }

    /// Clusters MCMAN allocates from (and counts as free space): FAT
    /// indices `0..=limit`. On an 8 MB card 8000 (+1), which is the
    /// "8,000 KB" a fresh card shows (the root takes one). Checked against
    /// MCMAN's free count on a used card (7884).
    fn limit(&self) -> u32 {
        (self.clusters_per_card / 1024 * 1000).min(self.alloc_end - self.alloc_offset - 1)
    }
}

/// A directory entry (512 bytes on the card).
#[derive(Clone)]
pub struct Entry(pub Vec<u8>);

impl Entry {
    fn blank() -> Entry {
        Entry(vec![0; ENTRY])
    }
    pub fn mode(&self) -> u16 {
        u16_at(&self.0, 0)
    }
    pub fn set_mode(&mut self, m: u16) {
        self.0[0..2].copy_from_slice(&m.to_le_bytes());
    }
    pub fn len(&self) -> u32 {
        u32_at(&self.0, 4)
    }
    pub fn set_len(&mut self, v: u32) {
        put32(&mut self.0, 4, v)
    }
    pub fn created(&self) -> Tod {
        self.0[8..16].try_into().unwrap()
    }
    pub fn set_created(&mut self, t: Tod) {
        self.0[8..16].copy_from_slice(&t)
    }
    pub fn cluster(&self) -> u32 {
        u32_at(&self.0, 0x10)
    }
    pub fn set_cluster(&mut self, v: u32) {
        put32(&mut self.0, 0x10, v)
    }
    pub fn dir_entry(&self) -> u32 {
        u32_at(&self.0, 0x14)
    }
    pub fn set_dir_entry(&mut self, v: u32) {
        put32(&mut self.0, 0x14, v)
    }
    pub fn modified(&self) -> Tod {
        self.0[0x18..0x20].try_into().unwrap()
    }
    pub fn set_modified(&mut self, t: Tod) {
        self.0[0x18..0x20].copy_from_slice(&t)
    }
    pub fn attr(&self) -> u32 {
        u32_at(&self.0, 0x20)
    }
    pub fn name(&self) -> &[u8] {
        let n = &self.0[0x40..0x60];
        &n[..n.iter().position(|&c| c == 0).unwrap_or(32)]
    }
    pub fn set_name(&mut self, name: &[u8]) {
        self.0[0x40..0x60].fill(0);
        let n = name.len().min(31);
        self.0[0x40..0x40 + n].copy_from_slice(&name[..n]);
    }
    pub fn exists(&self) -> bool {
        self.mode() & M_EXISTS != 0
    }
    pub fn is_dir(&self) -> bool {
        self.mode() & M_DIR != 0
    }
}

/// Where an entry lives: the first cluster of its directory and its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Loc {
    pub dir: u32,
    pub index: u32,
}

#[derive(Clone, Copy, Debug)]
struct OpenFile {
    loc: Loc,
    pos: u32,
    flags: u32,
    written: bool,
}

/// One libmc `sceMcTblGetDir` record (64 bytes).
pub fn dir_record(e: &Entry) -> [u8; 64] {
    let mut r = [0u8; 64];
    r[0..8].copy_from_slice(&e.created());
    r[8..16].copy_from_slice(&e.modified());
    let size = if e.is_dir() { 0 } else { e.len() };
    put32(&mut r, 16, size);
    r[20..22].copy_from_slice(&e.mode().to_le_bytes());
    put32(&mut r, 28, e.attr());
    let n = e.name();
    r[32..32 + n.len()].copy_from_slice(n);
    r
}

/// libmc's name patterns: `*` any run, `?` any one character.
pub fn matches(pat: &[u8], name: &[u8]) -> bool {
    match (pat.first(), name.first()) {
        (None, None) => true,
        (Some(b'*'), _) => {
            matches(&pat[1..], name) || (!name.is_empty() && matches(pat, &name[1..]))
        }
        (Some(b'?'), Some(_)) => matches(&pat[1..], &name[1..]),
        (Some(a), Some(b)) if a == b => matches(&pat[1..], &name[1..]),
        _ => false,
    }
}

/// The filesystem on one card.
pub struct Fs {
    pub card: Card,
    sb: Option<Super>,
    files: [Option<OpenFile>; MAX_FILES],
    cwd: String,
    /// GetInfo has not been asked since the card was inserted.
    fresh: bool,
    /// A GetDir listing in progress: (directory, pattern, next index).
    listing: Option<(u32, Vec<u8>, u32)>,
}

impl Fs {
    pub fn new(card: Card) -> Fs {
        let sb = Super::parse(card.page(0));
        Fs {
            card,
            sb,
            files: [None; MAX_FILES],
            cwd: "/".into(),
            fresh: true,
            listing: None,
        }
    }

    pub fn formatted(&self) -> bool {
        self.sb.is_some()
    }

    fn sb(&self) -> &Super {
        self.sb.as_ref().expect("formatted")
    }

    // ------------------------------------------------------------- FAT

    /// (FAT cluster, word index) of FAT entry `c`.
    fn fat_slot(&self, c: u32) -> (u32, usize) {
        let sb = self.sb();
        let per = (CLUSTER / 4) as u32;
        let ind = c / per;
        let indirect = sb.ifc[(ind / per) as usize];
        let fat = u32_at(&self.card.cluster(indirect), (ind % per) as usize * 4);
        (fat, (c % per) as usize)
    }

    pub fn fat(&self, c: u32) -> u32 {
        let (fc, i) = self.fat_slot(c);
        u32_at(&self.card.cluster(fc), i * 4)
    }

    fn set_fat(&mut self, c: u32, v: u32) {
        let (fc, i) = self.fat_slot(c);
        let mut cl = self.card.cluster(fc);
        put32(&mut cl, i * 4, v);
        self.card.write_cluster(fc, &cl);
    }

    /// The clusters of a chain (relative numbers).
    pub fn chain(&self, first: u32) -> Vec<u32> {
        let mut out = Vec::new();
        let mut c = first;
        let lim = self.sb().limit();
        while c <= lim && out.len() <= lim as usize {
            out.push(c);
            let e = self.fat(c);
            if e == FAT_EOC || e & ALLOCATED == 0 {
                break;
            }
            c = e & !ALLOCATED;
        }
        out
    }

    pub fn free_clusters(&self) -> u32 {
        (0..=self.sb().limit())
            .filter(|&c| self.fat(c) & ALLOCATED == 0)
            .count() as u32
    }

    /// Allocate a cluster (zeroed), linked after `prev` if given.
    fn alloc(&mut self, prev: Option<u32>) -> Option<u32> {
        let c = (0..=self.sb().limit()).find(|&c| self.fat(c) & ALLOCATED == 0)?;
        self.set_fat(c, FAT_EOC);
        if let Some(p) = prev {
            self.set_fat(p, c | ALLOCATED);
        }
        let abs = c + self.sb().alloc_offset;
        self.card.write_cluster(abs, &[0; CLUSTER]);
        Some(c)
    }

    fn free_chain(&mut self, first: u32) {
        for c in self.chain(first) {
            self.set_fat(c, FAT_FREE);
        }
    }

    fn read_rel(&self, c: u32) -> Vec<u8> {
        self.card.cluster(c + self.sb().alloc_offset)
    }

    fn write_rel(&mut self, c: u32, data: &[u8]) {
        let a = self.sb().alloc_offset;
        self.card.write_cluster(c + a, data);
    }

    // ------------------------------------------------------- directories

    pub fn entry(&self, loc: Loc) -> Option<Entry> {
        let ch = self.chain(loc.dir);
        let c = *ch.get(loc.index as usize / 2)?;
        let o = (loc.index as usize % 2) * ENTRY;
        Some(Entry(self.read_rel(c)[o..o + ENTRY].to_vec()))
    }

    fn put_entry(&mut self, loc: Loc, e: &Entry) {
        let ch = self.chain(loc.dir);
        let c = ch[loc.index as usize / 2];
        let mut cl = self.read_rel(c);
        let o = (loc.index as usize % 2) * ENTRY;
        cl[o..o + ENTRY].copy_from_slice(&e.0);
        self.write_rel(c, &cl);
    }

    /// The entry holding a directory's entry count: the root's ".", a
    /// subdirectory's entry in its parent.
    fn count_loc(&self, dir: u32) -> Loc {
        if dir == self.sb().root {
            return Loc { dir, index: 0 };
        }
        let dot = self.entry(Loc { dir, index: 0 }).expect("dir .");
        Loc {
            dir: dot.cluster(),
            index: dot.dir_entry(),
        }
    }

    pub fn dir_len(&self, dir: u32) -> u32 {
        self.entry(self.count_loc(dir)).map_or(0, |e| e.len())
    }

    fn parent(&self, dir: u32) -> u32 {
        if dir == self.sb().root {
            return dir;
        }
        self.entry(Loc { dir, index: 0 })
            .map_or(self.sb().root, |e| e.cluster())
    }

    fn lookup(&self, dir: u32, name: &[u8]) -> Option<Loc> {
        (0..self.dir_len(dir))
            .map(|index| Loc { dir, index })
            .find(|&l| {
                self.entry(l)
                    .is_some_and(|e| e.exists() && e.name() == name)
            })
    }

    /// Resolve `path` to (its directory, the final name). The directories
    /// on the way must exist.
    fn split(&self, path: &str) -> Result<(u32, Vec<u8>), i32> {
        let full = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("{}/{}", self.cwd.trim_end_matches('/'), path)
        };
        let parts: Vec<&str> = full.split('/').filter(|p| !p.is_empty()).collect();
        let mut dir = self.sb().root;
        let Some((last, dirs)) = parts.split_last() else {
            return Ok((dir, b".".to_vec()));
        };
        for p in dirs {
            dir = match *p {
                "." => dir,
                ".." => self.parent(dir),
                _ => {
                    let l = self.lookup(dir, p.as_bytes()).ok_or(NO_ENTRY)?;
                    let e = self.entry(l).ok_or(NO_ENTRY)?;
                    if !e.is_dir() {
                        return Err(NO_ENTRY);
                    }
                    e.cluster()
                }
            };
        }
        Ok((dir, last.as_bytes().to_vec()))
    }

    /// A directory's first cluster from its path.
    fn dir_of(&self, path: &str) -> Result<u32, i32> {
        let (dir, name) = self.split(path)?;
        match name.as_slice() {
            b"." => Ok(dir),
            b".." => Ok(self.parent(dir)),
            _ => {
                let e = self
                    .lookup(dir, &name)
                    .and_then(|l| self.entry(l))
                    .ok_or(NO_ENTRY)?;
                if e.is_dir() {
                    Ok(e.cluster())
                } else {
                    Err(NO_ENTRY)
                }
            }
        }
    }

    /// A slot for a new entry in `dir`: a deleted one, or a new one at the
    /// end (growing the chain when the last cluster is full).
    fn new_slot(&mut self, dir: u32) -> Result<Loc, i32> {
        let n = self.dir_len(dir);
        if let Some(index) = (2..n).find(|&i| {
            self.entry(Loc { dir, index: i })
                .is_some_and(|e| !e.exists())
        }) {
            return Ok(Loc { dir, index });
        }
        if n % 2 == 0 {
            let last = *self.chain(dir).last().unwrap();
            self.alloc(Some(last)).ok_or(FULL)?;
        }
        let cl = self.count_loc(dir);
        let mut c = self.entry(cl).unwrap();
        c.set_len(n + 1);
        self.put_entry(cl, &c);
        Ok(Loc { dir, index: n })
    }

    fn touch_dir(&mut self, dir: u32, now: Tod) {
        let cl = self.count_loc(dir);
        if let Some(mut e) = self.entry(cl) {
            e.set_modified(now);
            self.put_entry(cl, &e);
        }
    }

    // ------------------------------------------------------------ libmc

    /// sceMcGetInfo: (result, type, free clusters, formatted). The first
    /// call after insertion reports a new card (-1 formatted, -2 not).
    pub fn get_info(&mut self) -> (i32, u32, u32, u32) {
        let fresh = std::mem::replace(&mut self.fresh, false);
        if self.sb.is_none() {
            return (if fresh { NO_FORMAT } else { OK }, 2, 0, 0);
        }
        (if fresh { -1 } else { OK }, 2, self.free_clusters(), 1)
    }

    /// sceMcOpen / sceMcMkdir (`O_MKDIR`): a descriptor or an error.
    pub fn open(&mut self, path: &str, flags: u32, now: Tod) -> i32 {
        if self.sb.is_none() {
            return NO_FORMAT;
        }
        let (dir, name) = match self.split(path) {
            Ok(v) => v,
            Err(e) => return e,
        };
        if name.is_empty() || name == b"." || name == b".." {
            return NO_ENTRY;
        }
        let found = self.lookup(dir, &name);
        if flags & O_MKDIR != 0 {
            if found.is_some() {
                return NO_ENTRY;
            }
            return match self.mkdir(dir, &name, now) {
                Ok(()) => OK,
                Err(e) => e,
            };
        }
        let loc = match found {
            Some(l) => {
                let mut e = self.entry(l).unwrap();
                if e.is_dir() {
                    return DENIED;
                }
                if flags & O_TRUNC != 0 && flags & O_WRONLY != 0 && e.len() > 0 {
                    self.free_chain(e.cluster());
                    e.set_cluster(FAT_EOC);
                    e.set_len(0);
                    self.put_entry(l, &e);
                }
                l
            }
            None if flags & O_CREAT != 0 => {
                let l = match self.new_slot(dir) {
                    Ok(l) => l,
                    Err(e) => return e,
                };
                let mut e = Entry::blank();
                e.set_mode(M_EXISTS | M_0400 | M_FILE | M_READ | M_WRITE | M_EXEC);
                e.set_cluster(FAT_EOC);
                e.set_created(now);
                e.set_modified(now);
                e.set_name(&name);
                self.put_entry(l, &e);
                self.touch_dir(dir, now);
                l
            }
            None => return NO_ENTRY,
        };
        let Some(fd) = self.files.iter().position(|f| f.is_none()) else {
            return UP_LIMIT_HANDLE;
        };
        self.files[fd] = Some(OpenFile {
            loc,
            pos: 0,
            flags,
            written: false,
        });
        fd as i32
    }

    fn mkdir(&mut self, dir: u32, name: &[u8], now: Tod) -> Result<(), i32> {
        let l = self.new_slot(dir)?;
        let c = self.alloc(None).ok_or(FULL)?;
        // "." is stamped now, ".." with the parent's creation time (both
        // fields; as MCMAN writes them).
        let pdot = self.entry(Loc { dir, index: 0 }).unwrap();
        let mut dot = Entry::blank();
        dot.set_mode(M_EXISTS | M_0400 | M_DIR | M_READ | M_WRITE | M_EXEC);
        dot.set_created(now);
        dot.set_modified(now);
        dot.set_cluster(dir);
        dot.set_dir_entry(l.index);
        dot.set_name(b".");
        let mut dotdot = dot.clone();
        dotdot.set_created(pdot.created());
        dotdot.set_modified(pdot.created());
        dotdot.set_cluster(0);
        dotdot.set_dir_entry(0);
        dotdot.set_name(b"..");
        let mut cl = dot.0.clone();
        cl.extend_from_slice(&dotdot.0);
        self.write_rel(c, &cl);
        let mut e = Entry::blank();
        e.set_mode(M_EXISTS | M_0400 | M_DIR | M_READ | M_WRITE | M_EXEC);
        e.set_len(2);
        e.set_created(now);
        e.set_modified(now);
        e.set_cluster(c);
        e.set_name(name);
        self.put_entry(l, &e);
        self.touch_dir(dir, now);
        Ok(())
    }

    fn file(&self, fd: i32) -> Option<OpenFile> {
        self.files.get(fd as usize).copied().flatten()
    }

    pub fn close(&mut self, fd: i32, now: Tod) -> i32 {
        let Some(f) = self.file(fd) else {
            return NO_ENTRY;
        };
        self.files[fd as usize] = None;
        if f.written {
            if let Some(mut e) = self.entry(f.loc) {
                e.set_mode(e.mode() | M_CLOSED);
                e.set_modified(now);
                self.put_entry(f.loc, &e);
            }
        }
        OK
    }

    pub fn seek(&mut self, fd: i32, offset: i32, origin: u32) -> i32 {
        let Some(f) = self.file(fd) else {
            return NO_ENTRY;
        };
        let len = self.entry(f.loc).map_or(0, |e| e.len()) as i64;
        let base = match origin {
            0 => 0,
            1 => f.pos as i64,
            _ => len,
        };
        let pos = (base + offset as i64).max(0) as u32;
        self.files[fd as usize].as_mut().unwrap().pos = pos;
        pos as i32
    }

    pub fn read(&mut self, fd: i32, size: u32) -> Result<Vec<u8>, i32> {
        let f = self.file(fd).ok_or(NO_ENTRY)?;
        if f.flags & O_RDONLY == 0 {
            return Err(DENIED);
        }
        let e = self.entry(f.loc).ok_or(NO_ENTRY)?;
        let n = size.min(e.len().saturating_sub(f.pos));
        let mut out = Vec::with_capacity(n as usize);
        if n > 0 {
            let ch = self.chain(e.cluster());
            let mut p = f.pos;
            while (out.len() as u32) < n {
                let Some(&c) = ch.get(p as usize / CLUSTER) else {
                    break;
                };
                let cl = self.read_rel(c);
                let o = p as usize % CLUSTER;
                let k = (CLUSTER - o).min((n as usize) - out.len());
                out.extend_from_slice(&cl[o..o + k]);
                p += k as u32;
            }
        }
        self.files[fd as usize].as_mut().unwrap().pos += out.len() as u32;
        Ok(out)
    }

    pub fn write(&mut self, fd: i32, data: &[u8]) -> i32 {
        let Some(f) = self.file(fd) else {
            return NO_ENTRY;
        };
        if f.flags & O_WRONLY == 0 {
            return DENIED;
        }
        let Some(mut e) = self.entry(f.loc) else {
            return NO_ENTRY;
        };
        // (An empty file has no chain to follow, whatever its cluster says.)
        let mut ch = if e.cluster() == FAT_EOC || e.len() == 0 {
            Vec::new()
        } else {
            self.chain(e.cluster())
        };
        let end = f.pos as usize + data.len();
        while ch.len() * CLUSTER < end {
            let Some(c) = self.alloc(ch.last().copied()) else {
                break;
            };
            if ch.is_empty() {
                e.set_cluster(c);
            }
            ch.push(c);
        }
        let mut done = 0usize;
        let mut p = f.pos as usize;
        while done < data.len() {
            let Some(&c) = ch.get(p / CLUSTER) else { break };
            let mut cl = self.read_rel(c);
            let o = p % CLUSTER;
            let k = (CLUSTER - o).min(data.len() - done);
            cl[o..o + k].copy_from_slice(&data[done..done + k]);
            self.write_rel(c, &cl);
            done += k;
            p += k;
        }
        if p as u32 > e.len() {
            e.set_len(p as u32);
        }
        self.put_entry(f.loc, &e);
        let of = self.files[fd as usize].as_mut().unwrap();
        of.pos = p as u32;
        of.written = true;
        if done < data.len() && done == 0 {
            return FULL;
        }
        done as i32
    }

    /// sceMcGetDir: up to `max` records for `pattern` (a path whose last
    /// component may hold wildcards); `cont` continues the last listing.
    pub fn get_dir(&mut self, pattern: &str, cont: bool, max: u32) -> Result<Vec<[u8; 64]>, i32> {
        if self.sb.is_none() {
            return Err(NO_FORMAT);
        }
        if !cont || self.listing.is_none() {
            let (dir, pat) = self.split(pattern)?;
            self.listing = Some((dir, pat, 0));
        }
        let (dir, pat, mut next) = self.listing.take().unwrap();
        let n = self.dir_len(dir);
        let mut out = Vec::new();
        while next < n && (out.len() as u32) < max {
            if let Some(e) = self.entry(Loc { dir, index: next }) {
                if e.exists() && matches(&pat, e.name()) {
                    out.push(dir_record(&e));
                }
            }
            next += 1;
        }
        self.listing = Some((dir, pat, next));
        Ok(out)
    }

    /// sceMcChdir: the previous directory, or an error.
    pub fn chdir(&mut self, path: &str) -> Result<String, i32> {
        let dir = self.dir_of(path)?;
        let _ = dir;
        let full = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("{}/{}", self.cwd.trim_end_matches('/'), path)
        };
        // Normalise "." / "..".
        let mut parts: Vec<&str> = Vec::new();
        for p in full.split('/').filter(|p| !p.is_empty()) {
            match p {
                "." => {}
                ".." => {
                    parts.pop();
                }
                _ => parts.push(p),
            }
        }
        let new = format!("/{}", parts.join("/"));
        Ok(std::mem::replace(&mut self.cwd, new))
    }

    /// sceMcDelete: a file, or an empty directory.
    pub fn delete(&mut self, path: &str, now: Tod) -> i32 {
        if self.sb.is_none() {
            return NO_FORMAT;
        }
        let (dir, name) = match self.split(path) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let Some(l) = self.lookup(dir, &name) else {
            return NO_ENTRY;
        };
        if self.files.iter().flatten().any(|f| f.loc == l) {
            return DENIED;
        }
        let mut e = self.entry(l).unwrap();
        if e.is_dir() {
            let d = e.cluster();
            if (2..self.dir_len(d)).any(|i| {
                self.entry(Loc { dir: d, index: i })
                    .is_some_and(|x| x.exists())
            }) {
                return NOT_EMPTY;
            }
            self.free_chain(d);
        } else if e.cluster() != FAT_EOC && e.len() > 0 {
            self.free_chain(e.cluster());
        }
        e.set_mode(e.mode() & !M_EXISTS);
        self.put_entry(l, &e);
        self.touch_dir(dir, now);
        OK
    }

    /// sceMcSetFileInfo / sceMcRename: `flags` 1 = creation time, 2 =
    /// modification time, 4 = attributes, 0x10 = name; from a 64-byte
    /// `sceMcTblGetDir` record.
    pub fn set_file_info(&mut self, path: &str, flags: u32, rec: &[u8]) -> i32 {
        if self.sb.is_none() {
            return NO_FORMAT;
        }
        let (dir, name) = match self.split(path) {
            Ok(v) => v,
            Err(e) => return e,
        };
        let Some(l) = self.lookup(dir, &name) else {
            return NO_ENTRY;
        };
        let mut e = self.entry(l).unwrap();
        if flags & 0x10 != 0 {
            let n = &rec[32..64];
            let new = &n[..n.iter().position(|&c| c == 0).unwrap_or(32)];
            if self.lookup(dir, new).is_some() {
                return NO_ENTRY;
            }
            e.set_name(new);
        }
        if flags & 1 != 0 {
            e.set_created(rec[0..8].try_into().unwrap());
        }
        if flags & 2 != 0 {
            e.set_modified(rec[8..16].try_into().unwrap());
        }
        if flags & 4 != 0 {
            let keep = M_EXISTS | M_DIR | M_FILE | M_0400 | M_CLOSED;
            e.set_mode((e.mode() & keep) | (u16_at(rec, 20) & !keep));
        }
        self.put_entry(l, &e);
        OK
    }

    /// sceMcGetEntSpace: entries that fit in `path`'s directory without a
    /// new cluster.
    pub fn ent_space(&self, path: &str) -> i32 {
        match self.dir_of(path) {
            Ok(d) => (self.dir_len(d) % 2) as i32,
            Err(e) => e,
        }
    }

    /// sceMcFormat: an empty filesystem, laid out like MCMAN's.
    pub fn format(&mut self, now: Tod) -> i32 {
        const CPC: u32 = 8192;
        const ALLOC_OFFSET: u32 = 41;
        const ALLOC_END: u32 = 8135;
        let mut sb = vec![0u8; PAGE];
        sb[..36].copy_from_slice(b"Sony PS2 Memory Card Format 1.2.0.0\0"[..36].as_ref());
        sb[0x28..0x2a].copy_from_slice(&(PAGE as u16).to_le_bytes());
        sb[0x2a..0x2c].copy_from_slice(&2u16.to_le_bytes());
        sb[0x2c..0x2e].copy_from_slice(&16u16.to_le_bytes());
        sb[0x2e..0x30].copy_from_slice(&0xff00u16.to_le_bytes());
        put32(&mut sb, 0x30, CPC);
        put32(&mut sb, 0x34, ALLOC_OFFSET);
        put32(&mut sb, 0x38, ALLOC_END);
        put32(&mut sb, 0x3c, 0);
        put32(&mut sb, 0x40, 1023);
        put32(&mut sb, 0x44, 1022);
        put32(&mut sb, 0x50, 8);
        for i in 0..32 {
            put32(&mut sb, 0xd0 + 4 * i, 0xffff_ffff);
        }
        sb[0x150] = 2;
        sb[0x151] = 0x2b;
        // Everything zeroed (with ECC) except the second backup block,
        // which stays erased.
        let zero = [0u8; PAGE];
        for p in 0..PAGES {
            if (1022 * 16..1023 * 16).contains(&p) {
                self.card.erase_page(p);
            } else {
                self.card.write_page(p, &zero);
            }
        }
        self.card.write_page(0, &sb);
        // Indirect FAT cluster 8 → FAT clusters 9..=40, all free.
        let mut ind = vec![0u8; CLUSTER];
        for i in 0..32 {
            put32(&mut ind, 4 * i, 9 + i as u32);
        }
        self.card.write_cluster(8, &ind);
        let mut fat = vec![0u8; CLUSTER];
        for i in 0..256 {
            put32(&mut fat, 4 * i, FAT_FREE);
        }
        for c in 9..=40 {
            self.card.write_cluster(c, &fat);
        }
        self.sb = Super::parse(self.card.page(0));
        // Root directory in cluster 0.
        self.set_fat(0, FAT_EOC);
        let mut dot = Entry::blank();
        dot.set_mode(M_EXISTS | M_0400 | M_DIR | M_READ | M_WRITE | M_EXEC);
        dot.set_len(2);
        dot.set_created(now);
        dot.set_modified(now);
        dot.set_name(b".");
        let mut dotdot = dot.clone();
        dotdot.set_mode(M_EXISTS | 0x2000 | M_0400 | M_DIR | M_WRITE | M_EXEC);
        dotdot.set_len(0);
        dotdot.set_name(b"..");
        let mut cl = dot.0.clone();
        cl.extend_from_slice(&dotdot.0);
        self.write_rel(0, &cl);
        self.files = [None; MAX_FILES];
        self.cwd = "/".into();
        OK
    }

    /// sceMcUnformat: erase the card.
    pub fn unformat(&mut self) -> i32 {
        for p in 0..PAGES {
            self.card.erase_page(p);
        }
        self.sb = None;
        self.files = [None; MAX_FILES];
        OK
    }
}

#[cfg(test)]
mod tests;
