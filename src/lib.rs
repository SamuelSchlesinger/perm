use std::ops::Mul;

use smallvec::SmallVec;

/// A trait to overload the notion of a group acting on a type, akin
/// to a group acting on a set in group theory.
pub trait Action<Set: ?Sized> {
    fn act(&self, element: &mut Set);
}

/// Represents a permutation as a table, where a permutation
/// is a bijective function from \[n\] to \[n\], where \[n\] = {0, 1, ... n}.
///
/// ```text
/// 0 1 2 3
/// | | | |
/// v v v v
/// 1 3 2 0
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Table<const N: usize> {
    table: [usize; N],
}

impl<const N: usize> Action<usize> for Table<N> {
    fn act(&self, element: &mut usize) {
        *element = self.table[*element % N];
    }
}

impl<const N: usize, T> Action<[T]> for Table<N>
where
    Table<N>: Action<T>,
{
    fn act(&self, element: &mut [T]) {
        for i in element.iter_mut() {
            self.act(i);
        }
    }
}

#[cfg(feature = "random")]
impl<const N: usize> rand::distributions::Distribution<Table<N>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Table<N> {
        let mut table = Table { table: [0; N] };
        for i in 0..N {
            table.table[i] = i;
        }

        use rand::seq::SliceRandom;

        (&mut table.table).shuffle(rng);
        table
    }
}

impl<const N: usize> Table<N> {
    pub fn identity() -> Self {
        let mut table = Table { table: [0; N] };
        for i in 0..N {
            table.table[i] = i;
        }
        table
    }

    pub fn cycle() -> Self {
        let mut table = Table { table: [0; N] };
        for i in 0..N {
            table.table[i] = (i + 1 + N) % N;
        }
        table
    }
}

impl<const N: usize> Mul for Table<N> {
    type Output = Table<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut table = [0; N];
        for i in 0..N {
            table[i] = self.table[rhs.table[i]];
        }
        Table { table }
    }
}

impl<const N: usize> From<&CycleDecomposition<N>> for Table<N> {
    fn from(cycles: &CycleDecomposition<N>) -> Self {
        let mut table = Table { table: [0; N] };
        for cycle in cycles {
            for (i, j) in cycle
                .cycle_slice
                .iter()
                .copied()
                .zip(cycle.cycle_slice.iter().copied().cycle().skip(1))
            {
                table.table[i] = j;
            }
        }
        table
    }
}

/// Represents a permutation as a cycle decomposition, where a permutation
/// is a bijective function from \[n\] to \[n\], where \[n\] = {0, 1, ... n}.
///
/// If the table for a permutation looks like:
///
/// ```text
/// 0 1 2 3
/// | | | |
/// v v v v
/// 1 3 2 0
/// ```
///
/// then the cycle decomposition looks like:
///
/// ```text
/// (0 1 3) (2)
/// ```
///
/// Beware that the equality instance here will not check for equality
/// between group elements, but equality between their representations.
/// As such, one must normalize a decomposition before checking equality
/// in order to make sure the two refer to the same group element.
#[derive(Debug, PartialEq, Eq)]
pub struct CycleDecomposition<const N: usize> {
    enumeration: [usize; N],
    starts: SmallVec<[usize; 5]>,
}

impl<const N: usize> CycleDecomposition<N> {
    /// Because cycle decompositions are not structurally unique, it isn't
    /// useful to check PartialEq or Eq on them randomly. Instead, one should
    /// normalize them first and then check if they're equal.
    pub fn normalize(&mut self) {
        // TODO First make every cycle begin with their highest element
        // and then sort the cycles by the size of their highest element.
        todo!()
    }
}

/// A view into a particular cycle of a [`CycleDecomposition`].
pub struct Cycle<'a, const N: usize> {
    cycle_slice: &'a [usize],
}

/// An owned version of a [`Cycle`].
pub struct OwnedCycle<const N: usize> {
    cycle: Vec<usize>,
}

impl<'a, const N: usize> From<&'a OwnedCycle<N>> for Cycle<'a, N> {
    fn from(owned_cycle: &'a OwnedCycle<N>) -> Self {
        Cycle {
            cycle_slice: owned_cycle.cycle.as_slice(),
        }
    }
}

impl<'a, const N: usize> From<Cycle<'a, N>> for OwnedCycle<N> {
    fn from(cycle: Cycle<'a, N>) -> Self {
        OwnedCycle {
            cycle: cycle.cycle_slice.to_vec(),
        }
    }
}

impl<'a, const N: usize> Cycle<'a, N> {
    pub fn len(&self) -> usize {
        self.cycle_slice.len()
    }
}

pub struct CycleIter<'a, const N: usize> {
    decomposition: &'a CycleDecomposition<N>,
    cycle: usize,
}

impl<'a, const N: usize> Iterator for CycleIter<'a, N> {
    type Item = Cycle<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.decomposition.starts.len();

        if self.cycle >= n {
            return None;
        }

        self.cycle += 1;

        let right_endpoint = if self.cycle >= self.decomposition.starts.len() {
            N
        } else {
            self.decomposition.starts[self.cycle]
        };

        let next_cycle = Some(Cycle {
            cycle_slice: &self.decomposition.enumeration
                [self.decomposition.starts[self.cycle - 1]..right_endpoint],
        });

        next_cycle
    }
}

impl<'a, const N: usize> IntoIterator for &'a CycleDecomposition<N> {
    type Item = Cycle<'a, N>;

    type IntoIter = CycleIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        CycleIter {
            decomposition: &self,
            cycle: 0,
        }
    }
}

impl<const N: usize> CycleDecomposition<N> {
    pub fn cycle_type(&self) -> CycleType<N> {
        let mut cycle_type = [0; N];
        for cycle in self {
            cycle_type[cycle.len() - 1] += 1;
        }
        CycleType { cycle_type }
    }
}

/// Determines the cycle type, a representation of the conjugacy class of the
/// permutation in the symmetric group.
///
/// If the table for a permutation looks like:
///
/// ```text
/// 0 1 2 3
/// | | | |
/// v v v v
/// 1 3 2 0
/// ```
///
/// then the cycle decomposition looks like:
///
/// ```text
/// (0 1 3) (2)
/// ```
///
/// and the cycle type is:
///
/// ```text
/// [ 1, 0, 1, 0 ]
/// ```
///
/// because there is one cycle of length one, zero cycles of length
/// two, one cycle of length three, and zero cycles of length four.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CycleType<const N: usize> {
    cycle_type: [usize; N],
}

impl<const N: usize> CycleType<N> {
    pub fn as_slice(&self) -> &[usize; N] {
        &self.cycle_type
    }
}

impl<const N: usize> From<&Table<N>> for CycleDecomposition<N> {
    fn from(table: &Table<N>) -> CycleDecomposition<N> {
        let mut i = 0;
        let mut enumeration = [0; N];
        // NB: It's probably cheaper to first figure out how many cycles there
        // are and then preallocate the SmallVec with_capacity. Measure before
        // making this change.
        let mut starts: SmallVec<[usize; 5]> = SmallVec::new();
        // TODO Replace with bitvec. Tried once but I had problems using N because
        // of it being parameterized in the impl head.
        let mut used = [false; N];
        loop {
            if let Some((next_unused_index, _)) = used.iter().enumerate().find(|(_i, e)| !*e) {
                let mut j = next_unused_index;
                starts.extend(std::iter::once(i));
                loop {
                    used[j] = true;
                    enumeration[i] = table.table[j];
                    i += 1;
                    j = table.table[j];
                    if j == next_unused_index {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        CycleDecomposition {
            enumeration,
            starts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: usize = 100;

    #[test]
    fn cycle_cycles() {
        let table: Table<N> = Table::cycle();
        for i in 0..N {
            let mut j = i;
            table.act(&mut j);
            assert_eq!(j, (i + 1) % N);
        }
    }

    #[test]
    fn identity_identifies() {
        let table: Table<N> = Table::identity();
        for i in 0..N {
            let mut j = i;
            table.act(&mut j);
            assert_eq!(j, i);
        }
    }

    #[test]
    fn back_n_forth() {
        let table: Table<N> = Table::cycle();
        let cycle_decomposition_from_table: CycleDecomposition<N> = (&table).into();
        let table_from_cycle_decomposition_from_table = (&cycle_decomposition_from_table).into();

        assert_eq!(table, table_from_cycle_decomposition_from_table);
    }

    #[cfg(feature = "random")]
    #[test]
    fn random_round_trips() {
        use rand::prelude::*;

        let mut rng = thread_rng();

        for _ in 0..100 {
            let table: Table<N> = rng.gen();
            let cycle_decomposition_from_table: CycleDecomposition<N> = (&table).into();
            let table_from_cycle_decomposition_from_table =
                (&cycle_decomposition_from_table).into();

            assert_eq!(table, table_from_cycle_decomposition_from_table);
        }
    }
}
