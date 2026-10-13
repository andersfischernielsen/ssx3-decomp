//! The function inventory of `SLUS_207.72`, exported from Ghidra.
//!
//! [`FunctionIndex`] is a sorted, deduplicated table of every function Ghidra
//! recovered, including its exact extents. Addresses are ascending, so lookups
//! are a binary search.

use std::sync::LazyLock;

/// A single function as Ghidra recovered it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    /// Entry address in the EE address space.
    pub address: u32,
    /// Exclusive end of the function body (Ghidra's `body_end` + 1).
    pub end: u32,
    /// Symbol name (`entry`, `FUN_001001c8`, `SetGsCrt`, ...).
    pub name: &'static str,
}

/// Prefixes Ghidra uses for names it generated itself, as opposed to names
/// recovered from symbols or applied by a human.
const AUTO_PREFIXES: &[&str] = &[
    "FUN_", "thunk_FUN_", "LAB_", "DAT_", "UNK_", "SUB_", "switchD_", "caseD_",
];

impl Function {
    /// Body length in bytes.
    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.address)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// True when `addr` lies within this function body.
    pub fn contains(&self, addr: u32) -> bool {
        (self.address..self.end).contains(&addr)
    }

    /// True when the name looks like an auto-generated placeholder rather than
    /// a recovered/human name.
    pub fn is_auto_named(&self) -> bool {
        AUTO_PREFIXES.iter().any(|p| self.name.starts_with(p))
    }

    /// True when the name was recovered from symbols or applied by a human.
    pub fn is_named(&self) -> bool {
        !self.is_auto_named()
    }
}

/// `generated/functions.tsv`: `<hex-address>\t<hex-end>\t<name>` per line.
static TSV: &str = include_str!("../../../generated/functions.tsv");

static INDEX: LazyLock<FunctionIndex> = LazyLock::new(|| FunctionIndex::parse(TSV));

/// The embedded inventory.
pub fn index() -> &'static FunctionIndex {
    &INDEX
}

/// A sorted table of [`Function`]s.
pub struct FunctionIndex {
    functions: Vec<Function>,
}

impl FunctionIndex {
    /// Parse an `<hex-addr>\t<hex-end>\t<name>` table. Lines must already be
    /// address-ascending; this does not sort (that would hide export bugs).
    pub fn parse(tsv: &'static str) -> Self {
        let mut functions = Vec::new();
        for line in tsv.lines() {
            if line.is_empty() {
                continue;
            }
            let mut cols = line.split('\t');
            let (addr, end, name) = match (cols.next(), cols.next(), cols.next()) {
                (Some(a), Some(e), Some(n)) => (a, e, n),
                _ => panic!("malformed inventory line: {line:?}"),
            };
            let address =
                u32::from_str_radix(addr, 16).unwrap_or_else(|_| panic!("bad address {addr:?}"));
            let end = u32::from_str_radix(end, 16).unwrap_or_else(|_| panic!("bad end {end:?}"));
            functions.push(Function { address, end, name });
        }
        Self { functions }
    }

    /// Every function, ascending by address.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn len(&self) -> usize {
        self.functions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Look up a function by its exact entry address.
    pub fn by_address(&self, address: u32) -> Option<&Function> {
        self.functions
            .binary_search_by_key(&address, |f| f.address)
            .ok()
            .map(|i| &self.functions[i])
    }

    /// The function containing `address` (entry inclusive, end exclusive).
    pub fn containing(&self, address: u32) -> Option<&Function> {
        let i = match self.functions.binary_search_by_key(&address, |f| f.address) {
            Ok(i) => i,
            Err(0) => return None,
            Err(i) => i - 1,
        };
        let f = &self.functions[i];
        f.contains(address).then_some(f)
    }

    /// All functions whose name contains `needle` (case-insensitive).
    pub fn search(&self, needle: &str) -> impl Iterator<Item = &Function> {
        let needle = needle.to_lowercase();
        self.functions
            .iter()
            .filter(move |f| f.name.to_lowercase().contains(&needle))
    }

    /// Functions with recovered/human names (not auto-generated placeholders).
    pub fn named(&self) -> impl Iterator<Item = &Function> {
        self.functions.iter().filter(|f| f.is_named())
    }

    /// Count of functions with recovered/human names.
    pub fn named_count(&self) -> usize {
        self.named().count()
    }

    /// Lowest and highest entry addresses.
    pub fn address_range(&self) -> Option<(u32, u32)> {
        match (self.functions.first(), self.functions.last()) {
            (Some(a), Some(b)) => Some((a.address, b.address)),
            _ => None,
        }
    }

    /// True when addresses are strictly ascending.
    pub fn is_sorted_unique(&self) -> bool {
        self.functions.windows(2).all(|w| w[0].address < w[1].address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_looks_up() {
        let idx = FunctionIndex::parse(
            "00100008\t001001c8\tentry\n001001c8\t001001d0\tFUN_001001c8\n",
        );
        assert_eq!(idx.len(), 2);
        assert_eq!(idx.by_address(0x0010_0008).unwrap().name, "entry");
        assert_eq!(idx.by_address(0x0010_0008).unwrap().end, 0x0010_01c8);
        assert_eq!(idx.by_address(0x0010_01c8).unwrap().len(), 8);
        assert!(idx.by_address(0x0010_0009).is_none());
        assert!(idx.is_sorted_unique());
    }

    #[test]
    fn containing_respects_the_end() {
        let idx = FunctionIndex::parse(
            "00100008\t001001c8\tentry\n001001c8\t001001d0\tFUN_001001c8\n",
        );
        assert_eq!(idx.containing(0x0010_0100).unwrap().name, "entry");
        // 0x001001d0 is past the end of FUN_001001c8.
        assert!(idx.containing(0x0010_01d0).is_none());
    }

    #[test]
    fn auto_named_classification() {
        let idx = FunctionIndex::parse(
            "00100008\t001001c8\tentry\n001001c8\t001001d0\tFUN_001001c8\n",
        );
        assert!(idx.by_address(0x0010_0008).unwrap().is_named());
        assert!(idx.by_address(0x0010_01c8).unwrap().is_auto_named());
    }
}
