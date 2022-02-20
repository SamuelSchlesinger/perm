use std::{collections::VecDeque, ops::Mul};

/// Represents a permutation as a table, where a permutation
/// is a bijective function from \[n\] to \[n\], where \[n\] = {0, 1, ... n}.
///
/// ```
/// 0 1 2 3
/// | | | |
/// v v v v
/// 1 3 2 0
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Table<const N: usize> {
    table: [usize; N],
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

impl<const N: usize> From<CycleDecomposition<N>> for Table<N> {
    fn from(_cycles: CycleDecomposition<N>) -> Self {
        todo!()
    }
}

/// Represents a permutation as a cycle decomposition, where a permutation
/// is a bijective function from \[n\] to \[n\], where \[n\] = {0, 1, ... n}.
///
/// If the table for a permutation looks like:
///
/// ```
/// 0 1 2 3
/// | | | |
/// v v v v
/// 1 3 2 0
/// ```
///
/// then the cycle decomposition looks like:
///
/// ```
/// (0 1 3) (2)
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CycleDecomposition<const N: usize> {
    cycles: Box<[Box<[usize]>]>,
}

impl<const N: usize> CycleDecomposition<N> {
    pub fn cycle_type(&self) -> CycleType<N> {
        let mut cycle_type = [0; N];
        for cycle in self.cycles.iter() {
            cycle_type[cycle.len()] += 1;
        }
        CycleType { cycle_type }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CycleType<const N: usize> {
    cycle_type: [usize; N],
}

impl<const N: usize> CycleType<N> {
    pub fn as_slice(&self) -> &[usize; N] {
        &self.cycle_type
    }
}

impl<const N: usize> From<Table<N>> for CycleDecomposition<N> {
    fn from(table: Table<N>) -> CycleDecomposition<N> {
        let mut included = [false; N];
        let mut tmp_cycles = Vec::new();
        loop {
            if let Some(i) = included
                .iter()
                .enumerate()
                .fold(None, |acc, (i, b)| match acc {
                    Some(_) => acc,
                    None => {
                        if !*b {
                            Some(i)
                        } else {
                            None
                        }
                    }
                })
            {
                let mut cycle = VecDeque::new();
                let mut j = table.table[i];
                included[i] = true;
                cycle.push_back(i);
                loop {
                    if j == i {
                        tmp_cycles.push(cycle);
                        break;
                    } else {
                        cycle.push_back(j);
                        included[j] = true;
                        j = table.table[j];
                    }
                }
            } else {
                break;
            }
        }
        fn normalize(v: &VecDeque<usize>) -> Box<[usize]> {
            if let Some((highest, _)) = v
                .iter()
                .enumerate()
                .max_by_key(|(_highest_index, highest_value)| *highest_value)
            {
                let mut b = vec![0; v.len()];
                let n = v.len();
                let mut i: usize = highest;
                loop {
                    b[(n + i - highest) % n] = v[i % n];
                    i = (i + 1) % n;
                    if i == highest {
                        break;
                    }
                }
                b.into_boxed_slice()
            } else {
                panic!("no no no no no");
            }
        }
        let n_cycles = tmp_cycles.len();
        let cycles = (0..n_cycles)
            .map(|i| normalize(&tmp_cycles[i]))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        CycleDecomposition { cycles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_from_table() {
        let table: Table<3> = Table { table: [1, 2, 0] };
        let cycles: CycleDecomposition<3> = CycleDecomposition {
            cycles: vec![vec![2, 0, 1].into_boxed_slice()].into_boxed_slice(),
        };
        assert_eq!(CycleDecomposition::from(table), cycles);

        let table: Table<3> = Table { table: [0, 1, 2] };
        let cycles: CycleDecomposition<3> = CycleDecomposition {
            cycles: vec![
                vec![0].into_boxed_slice(),
                vec![1].into_boxed_slice(),
                vec![2].into_boxed_slice(),
            ]
            .into_boxed_slice(),
        };
        assert_eq!(CycleDecomposition::from(table), cycles);
    }
}
