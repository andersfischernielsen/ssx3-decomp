use super::*;

const NOW: Tod = [0, 1, 2, 3, 4, 5, 0xea, 0x07];

fn formatted() -> Fs {
    let mut fs = Fs::new(Card::erased());
    assert!(!fs.formatted());
    assert_eq!(fs.format(NOW), OK);
    fs
}

/// Every allocated cluster is reachable from exactly one entry (or is a
/// directory's), and every chain ends.
fn check(fs: &Fs) {
    let mut owned = std::collections::HashSet::new();
    fn walk(fs: &Fs, dir: u32, owned: &mut std::collections::HashSet<u32>) {
        for c in fs.chain(dir) {
            assert!(owned.insert(c), "cluster {c} shared");
        }
        for i in 2..fs.dir_len(dir) {
            let e = fs.entry(Loc { dir, index: i }).unwrap();
            if !e.exists() {
                continue;
            }
            if e.is_dir() {
                walk(fs, e.cluster(), owned);
            } else if e.len() > 0 {
                let ch = fs.chain(e.cluster());
                assert_eq!(
                    ch.len(),
                    (e.len() as usize).div_ceil(CLUSTER),
                    "{:?}",
                    String::from_utf8_lossy(e.name())
                );
                for c in ch {
                    assert!(owned.insert(c), "cluster {c} shared");
                }
            }
        }
    }
    walk(fs, fs.sb().root, &mut owned);
    let allocated: std::collections::HashSet<u32> = (0..=fs.sb().limit())
        .filter(|&c| fs.fat(c) & ALLOCATED != 0)
        .collect();
    assert_eq!(allocated, owned);
}

#[test]
fn a_fresh_card_has_8000_kb_free() {
    let mut fs = formatted();
    assert_eq!(fs.get_info(), (-1, 2, 8000, 1));
    assert_eq!(fs.get_info().0, OK);
    check(&fs);
}

#[test]
fn an_erased_card_is_unformatted() {
    let mut fs = Fs::new(Card::erased());
    assert_eq!(fs.get_info(), (NO_FORMAT, 2, 0, 0));
    assert_eq!(fs.get_info().0, OK);
    assert_eq!(fs.open("/x", O_RDONLY, NOW), NO_FORMAT);
}

#[test]
fn save_round_trip() {
    let mut fs = formatted();
    assert_eq!(fs.open("/BASLUS-20772-GAM0001", O_MKDIR, NOW), OK);
    assert_eq!(fs.open("/BASLUS-20772-GAM0001", O_MKDIR, NOW), NO_ENTRY);
    let data: Vec<u8> = (0..39777u32).map(|i| (i * 7 + i / 300) as u8).collect();
    let fd = fs.open("/BASLUS-20772-GAM0001/save", O_WRONLY | O_CREAT, NOW);
    assert!(fd >= 0);
    // Written in odd pieces, like libmc's aligned/unaligned split.
    let mut at = 0;
    for n in [5usize, 1019, 1024, 3000, 100000] {
        let k = n.min(data.len() - at);
        assert_eq!(fs.write(fd, &data[at..at + k]), k as i32);
        at += k;
    }
    assert_eq!(fs.close(fd, NOW), OK);
    check(&fs);
    // The root grows a cluster for its third entry; the directory takes two
    // (".", "..", "save"), the file 39.
    assert_eq!(fs.free_clusters(), 8000 - 1 - 2 - 39);

    fs.chdir("/BASLUS-20772-GAM0001").unwrap();
    let fd = fs.open("save", O_RDONLY, NOW);
    assert!(fd >= 0);
    assert_eq!(fs.seek(fd, 1000, 0), 1000);
    assert_eq!(fs.read(fd, 50000).unwrap(), &data[1000..]);
    assert_eq!(fs.read(fd, 10).unwrap(), b"");
    fs.close(fd, NOW);

    let recs = fs.get_dir("/BASLUS-20772-GAM0001/*", false, 10).unwrap();
    let names: Vec<&[u8]> = recs
        .iter()
        .map(|r| &r[32..32 + r[32..].iter().position(|&c| c == 0).unwrap()])
        .collect();
    assert_eq!(names, [&b"."[..], b"..", b"save"]);
    assert_eq!(
        u32::from_le_bytes(recs[2][16..20].try_into().unwrap()),
        39777
    );
    assert_eq!(u16::from_le_bytes([recs[2][20], recs[2][21]]), 0x8497);

    // Not empty, then empty.
    assert_eq!(fs.delete("/BASLUS-20772-GAM0001", NOW), NOT_EMPTY);
    assert_eq!(fs.delete("/BASLUS-20772-GAM0001/save", NOW), OK);
    assert_eq!(fs.delete("/BASLUS-20772-GAM0001", NOW), OK);
    check(&fs);
    // (The root keeps its second cluster.)
    assert_eq!(fs.free_clusters(), 7999);
    assert!(fs.get_dir("/*", false, 10).unwrap().len() == 2);
}

#[test]
fn directories_grow_and_reuse_slots() {
    let mut fs = formatted();
    for i in 0..9 {
        let fd = fs.open(&format!("/f{i}"), O_WRONLY | O_CREAT, NOW);
        assert_eq!(fs.write(fd, &[i as u8; 10]), 10);
        fs.close(fd, NOW);
    }
    assert_eq!(fs.dir_len(0), 11);
    check(&fs);
    assert_eq!(fs.delete("/f3", NOW), OK);
    let fd = fs.open("/g", O_WRONLY | O_CREAT, NOW);
    fs.close(fd, NOW);
    assert_eq!(fs.dir_len(0), 11);
    let recs = fs.get_dir("/f*", false, 3).unwrap();
    assert_eq!(recs.len(), 3);
    let more = fs.get_dir("", true, 100).unwrap();
    assert_eq!(more.len(), 5);
    check(&fs);
}

#[test]
fn wildcards() {
    assert!(matches(b"BASLUS-20772-GAM*", b"BASLUS-20772-GAM0001"));
    assert!(!matches(b"BASLUS-20772-SET*", b"BASLUS-20772-GAM0001"));
    assert!(matches(b"*", b"."));
    assert!(matches(b"a?c", b"abc"));
    assert!(!matches(b"a?c", b"ac"));
}

/// The card MCMAN wrote (`saves/ssx3-mc0.ps2`, when present): what the
/// recompiled MCSERV answered the game with (SSX3_TRACE_SIF, 2026-10-17).
#[test]
fn reads_like_mcman() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../saves/ssx3-mc0.ps2");
    let Ok(img) = std::fs::read(path) else {
        eprintln!("no {path}; skipped");
        return;
    };
    let mut fs = Fs::new(Card::from_image(img.clone()));
    assert!(fs.formatted());
    check(&fs);
    let (r, ty, free, fmt) = fs.get_info();
    if free != 7884 {
        eprintln!("the save card changed; skipped");
        return;
    }
    assert_eq!((r, ty, free, fmt), (-1, 2, 7884, 1));
    let recs = fs.get_dir("BASLUS-20772-GAM*", false, 6).unwrap();
    assert_eq!(recs.len(), 1);
    let words: Vec<u32> = recs[0]
        .chunks(4)
        .take(8)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    assert_eq!(
        words,
        [0x06361100, 0x07e7051f, 0x06361200, 0x07e7051f, 0, 0x8427, 0, 0]
    );
    let recs = fs
        .get_dir("/BASLUS-20772-GAM0001/BASLUS-20772-GAM0001", false, 1)
        .unwrap();
    let words: Vec<u32> = recs[0]
        .chunks(4)
        .take(6)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    assert_eq!(
        words,
        [0x06361200, 0x07e7051f, 0x06361200, 0x07e7051f, 0x9b61, 0x8497]
    );
    assert!(fs
        .get_dir("BASLUS-20772-SET*", false, 6)
        .unwrap()
        .is_empty());
    let fd = fs.open("/BASLUS-20772-GAM0001/icon.sys", O_RDONLY, NOW);
    assert_eq!(fd, 0);
    let d = fs.read(fd, 0x3c4).unwrap();
    assert_eq!(d.len(), 0x3c4);
    assert_eq!(&d[..4], b"PS2D");
    // Our format writes MCMAN's superblock.
    let mut f = Fs::new(Card::erased());
    f.format(NOW);
    assert_eq!(f.card.page(0), &img[..PAGE]);
}

/// A new directory's "." is stamped now, ".." with the parent's creation
/// time, even after the parent was modified (MCMAN, overwriting a save).
#[test]
fn new_directories_are_stamped_like_mcman() {
    let mut fs = formatted();
    let later: Tod = [0, 9, 9, 9, 9, 9, 0xea, 0x07];
    let fd = fs.open("/x", O_WRONLY | O_CREAT, later);
    fs.close(fd, later);
    assert_eq!(fs.delete("/x", later), OK);
    assert_eq!(fs.open("/d", O_MKDIR, later), OK);
    let d = fs.entry(fs.lookup(0, b"d").unwrap()).unwrap().cluster();
    let dot = fs.entry(Loc { dir: d, index: 0 }).unwrap();
    let dotdot = fs.entry(Loc { dir: d, index: 1 }).unwrap();
    assert_eq!((dot.created(), dot.modified()), (later, later));
    assert_eq!((dotdot.created(), dotdot.modified()), (NOW, NOW));
    assert_eq!((dot.cluster(), dot.dir_entry()), (0, 2));
}
