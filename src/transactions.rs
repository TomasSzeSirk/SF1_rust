use std::any::Any;
use std::rc::Rc;

use crate::Pesos;
use crate::receptive_account::ReceptiveAccount;

pub trait Transaction {
    fn value(&self) -> Pesos;
    // Necesario para hasRegistered (identidad)
    fn as_any(&self) -> &dyn Any;
}

/* ---------- Deposit ---------- */

#[derive(Debug)]
pub struct Deposit {
    amount: Pesos,
}

impl Deposit {
    pub fn register(amount: Pesos, account: &mut ReceptiveAccount) -> Rc<dyn Transaction> {
        let transaction = Rc::new(Deposit { amount });
        account.register(transaction.clone());
        transaction
    }

    pub fn for_amount(amount: Pesos) -> Rc<dyn Transaction> {
        Rc::new(Deposit { amount })
    }
}

impl Transaction for Deposit {
    fn value(&self) -> Pesos {
        self.amount
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/* ---------- Withdraw ---------- */

#[derive(Debug)]
pub struct Withdraw {
    amount: Pesos,
}

impl Withdraw {
    pub fn register(amount: Pesos, account: &mut ReceptiveAccount) -> Rc<dyn Transaction> {
        let transaction = Rc::new(Withdraw { amount });
        account.register(transaction.clone());
        transaction
    }

    pub fn for_amount(amount: Pesos) -> Rc<dyn Transaction> {
        Rc::new(Withdraw { amount })
    }
}

impl Transaction for Withdraw {
    fn value(&self) -> Pesos {
        self.amount
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}