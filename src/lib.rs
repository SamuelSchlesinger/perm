#![allow(dead_code)]

use std::{collections::VecDeque, ops::Mul};

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

impl<const N: usize> From<Cycles<N>> for Table<N> {
    fn from(_cycles: Cycles<N>) -> Self {
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cycles<const N: usize> {
    cycles: Vec<VecDeque<usize>>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CycleType<const N: usize> {
    cycle_type: [usize; N],
}

impl<const N: usize> Cycles<N> {
    fn cycle_type(&self) -> CycleType<N> {
        let mut cycle_type = [0; N];
        for cycle in self.cycles.iter() {
            cycle_type[cycle.len()] += 1;
        }
        CycleType { cycle_type }
    }
}

impl<const N: usize> From<Table<N>> for Cycles<N> {
    fn from(table: Table<N>) -> Cycles<N> {
        let mut included = [false; N];
        let mut cycles = Vec::new();
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
                        cycles.push(cycle);
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
        Cycles { cycles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycles_from_table() {
        let table: Table<3> = Table { table: [1, 2, 0] };
        let cycles: Cycles<3> = Cycles {
            cycles: vec![VecDeque::from_iter(vec![0, 1, 2].into_iter())],
        };
        assert_eq!(Cycles::from(table), cycles);

        let table: Table<3> = Table { table: [0, 1, 2] };
        let cycles: Cycles<3> = Cycles {
            cycles: vec![
                VecDeque::from_iter(vec![0].into_iter()),
                VecDeque::from_iter(vec![1].into_iter()),
                VecDeque::from_iter(vec![2].into_iter()),
            ],
        };
        assert_eq!(Cycles::from(table), cycles);
    }
}
