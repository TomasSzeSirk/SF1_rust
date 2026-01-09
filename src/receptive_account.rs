use std::rc::Rc;
use crate::{Pesos, transactions::Transaction};

pub struct ReceptiveAccount {
    transactions: Vec<Rc<dyn Transaction>>,
}

impl ReceptiveAccount {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }

    pub fn register(&mut self, transaction: Rc<dyn Transaction>) {
        self.transactions.push(transaction);
    }

    pub fn balance(&self) -> Pesos {
        self.transactions
            .iter()
            .map(|t| t.value())
            .sum()
    }

    pub fn has_registered(&self, a_transaction: &Rc<dyn Transaction>) -> bool {
        self.transactions
            .iter()
            .any(|transaction| Rc::ptr_eq(transaction, a_transaction))
    }

    pub fn transactions(&self) -> Vec<Rc<dyn Transaction>> {
        self.transactions.clone()
    }
}