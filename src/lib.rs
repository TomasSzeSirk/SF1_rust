use std::iter::Sum;

pub mod receptive_account;
pub mod transactions;

#[cfg(test)]
mod receptive_account_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pesos{
    value: i32
}

impl Pesos {
    pub fn new(value: i32) -> Self {
        Pesos { value }
    }
}

impl Sum for Pesos {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let total = iter.fold(0, |acc, pesos| acc + pesos.value);
        Pesos::new(total)
    }
}
