//! Authoritative instruction boundaries, exported from Ghidra.
//!
//! A function body is not a contiguous run of instructions: it can contain
//! embedded data (jump tables) and bytes Ghidra never disassembled. Ghidra knows
//! exactly where the instructions are, so we use its map rather than a blind
//! linear sweep.
//!
//! `generated/boundaries.tsv` stores, per function, the inclusive instruction
//! runs (end exclusive), e.g.
//!
//! ```text
//! 0042d470  0042d470-0042d584,0042d588-0042d5ec,0042d5f0-0042d864,0042d868-0042dad8
//! ```

use std::sync::LazyLock;

/// A half-open range of instruction addresses `[start, end)`.
pub type Run = (u32, u32);

/// Per-function instruction boundary map.
pub struct Boundaries {
    entries: Vec<(u32, Vec<Run>)>,
}

static TSV: &str = include_str!("../../../generated/boundaries.tsv");
static BOUNDARIES: LazyLock<Boundaries> = LazyLock::new(|| Boundaries::parse(TSV));

/// The embedded boundary map.
pub fn boundaries() -> &'static Boundaries {
    &BOUNDARIES
}

impl Boundaries {
    pub fn parse(tsv: &'static str) -> Self {
        let mut entries = Vec::new();
        for line in tsv.lines() {
            if line.is_empty() {
                continue;
            }
            let (addr, spec) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("malformed boundary line: {line:?}"));
            let address =
                u32::from_str_radix(addr, 16).unwrap_or_else(|_| panic!("bad address {addr:?}"));
            let mut runs = Vec::new();
            for run in spec.split(',') {
                let (lo, hi) = run
                    .split_once('-')
                    .unwrap_or_else(|| panic!("bad run {run:?}"));
                let lo = u32::from_str_radix(lo, 16).unwrap();
                let hi = u32::from_str_radix(hi, 16).unwrap();
                assert!(hi > lo, "empty run {run:?}");
                runs.push((lo, hi));
            }
            entries.push((address, runs));
        }
        entries.sort_by_key(|(a, _)| *a);
        Self { entries }
    }

    /// Instruction runs for the function whose entry is `address`.
    pub fn runs(&self, address: u32) -> Option<&[Run]> {
        self.entries
            .binary_search_by_key(&address, |(a, _)| *a)
            .ok()
            .map(|i| self.entries[i].1.as_slice())
    }

    /// Number of functions with a boundary entry.
    pub fn function_count(&self) -> usize {
        self.entries.len()
    }

    /// Total instructions across every function.
    pub fn instruction_count(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|(_, runs)| runs)
            .map(|(lo, hi)| ((hi - lo) / 4) as usize)
            .sum()
    }

    /// Functions whose body has more than one run (i.e. holes or embedded data).
    pub fn holed_count(&self) -> usize {
        self.entries.iter().filter(|(_, r)| r.len() > 1).count()
    }

    /// Iterate `(address, run)` for every instruction address.
    pub fn iter_instructions(&self) -> impl Iterator<Item = u32> + '_ {
        self.entries.iter().flat_map(|(_, runs)| {
            runs.iter().flat_map(|(lo, hi)| (*lo..*hi).step_by(4))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_runs() {
        let b = Boundaries::parse(
            "00100008\t00100008-00100010\n0042d470\t0042d470-0042d478,0042d480-0042d488\n",
        );
        assert_eq!(b.function_count(), 2);
        assert_eq!(b.runs(0x0010_0008), Some(&[(0x0010_0008, 0x0010_0010)][..]));
        assert_eq!(
            b.runs(0x0042_d470),
            Some(&[(0x0042_d470, 0x0042_d478), (0x0042_d480, 0x0042_d488)][..])
        );
        assert_eq!(b.instruction_count(), 2 + 4);
        assert_eq!(b.holed_count(), 1);
    }

    #[test]
    fn unknown_function_is_none() {
        let b = Boundaries::parse("00100008\t00100008-00100010\n");
        assert!(b.runs(0xdead_beef).is_none());
    }
}
