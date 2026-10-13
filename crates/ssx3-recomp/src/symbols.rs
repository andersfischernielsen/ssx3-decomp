//! The function inventory of `SLUS_207.72`, exported from Ghidra.
//!
//! [`FunctionIndex`] is a sorted, deduplicated table of every function Ghidra
//! recovered. Addresses are ascending, so lookups are a binary search.

use std::sync::LazyLock;

/// A single function as Ghidra names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Function {
    /// Entry address in the EE address space.
    pub address: u32,
    /// Symbol name (`entry`, `FUN_001001c8`, `SetGsCrt`, ...).
    pub name: &'static str,
}

/// Prefixes Ghidra uses for names it generated itself, as opposed to names
/// recovered from symbols or applied by a human.
const AUTO_PREFIXES: &[&str] = &[
    "FUN_", "thunk_FUN_", "LAB_", "DAT_", "UNK_", "SUB_", "switchD_", "caseD_",
];

impl Function {
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

/// `generated/functions.tsv`: `<hex-address>\t<name>` per line, ascending.
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
    /// Parse a `<hex>\t<name>` table. Lines must already be address-ascending;
    /// this does not sort (that would hide export bugs).
    pub fn parse(tsv: &'static str) -> Self {
        let mut functions = Vec::new();
        for line in tsv.lines() {
            if line.is_empty() {
                continue;
            }
            let (addr, name) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed inventory line: {line:?}"));
            let address = u32::from_str_radix(addr, 16)
                .unwrap_or_else(|_| panic!("bad address {addr:?}"));
            functions.push(Function { address, name });
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

    /// The function at or containing `address`, given a parallel list of
    /// function end addresses is not tracked here — this returns the nearest
    /// function at or below `address`.
    pub fn containing(&self, address: u32) -> Option<&Function> {
        let i = match self.functions.binary_search_by_key(&address, |f| f.address) {
            Ok(i) => i,
            Err(0) => return None,
            Err(i) => i - 1,
        };
        Some(&self.functions[i])
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
        let idx = FunctionIndex::parse("00100008\tentry\n001001c8\tFUN_001001c8\n");
        assert_eq!(idx.len(), 2);
        assert_eq!(idx.by_address(0x0010_0008).unwrap().name, "entry");
        assert_eq!(idx.by_address(0x0010_01c8).unwrap().name, "FUN_001001c8");
        assert!(idx.by_address(0x0010_0009).is_none());
        assert!(idx.is_sorted_unique());
    }

    #[test]
    fn containing_finds_nearest_below() {
        let idx = FunctionIndex::parse("00100008\tentry\n001001c8\tFUN_001001c8\n");
        assert_eq!(idx.containing(0x0010_0100).unwrap().name, "entry");
        assert_eq!(idx.containing(0x0010_01ff).unwrap().name, "FUN_001001c8");
        assert!(idx.containing(0x0000_0000).is_none());
    }

    #[test]
    fn auto_named_classification() {
        let idx = FunctionIndex::parse("00100008\tentry\n001001c8\tFUN_001001c8\n");
        assert!(idx.by_address(0x0010_0008).unwrap().is_named());
        assert!(idx.by_address(0x0010_01c8).unwrap().is_auto_named());
    }
}
