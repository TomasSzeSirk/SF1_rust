use crate::Pesos;
use crate::receptive_account::ReceptiveAccount;
use crate::transactions::{Deposit, Withdraw};


#[test]
fn test01_receptive_account_have_zero_as_balance_when_created() {
    let account = ReceptiveAccount::new();
    assert_eq!(Pesos::new(0), account.balance());
}

#[test]
fn test02_deposit_increases_balance_on_transaction_value() {
    let mut account = ReceptiveAccount::new();

    Deposit::register(Pesos::new(100), &mut account);

    assert_eq!(Pesos::new(100), account.balance());
}

#[test]
fn test03_withdraw_decreases_balance_on_transaction_value() {
    let mut account = ReceptiveAccount::new();

    Deposit::register(Pesos::new(100), &mut account);
    Withdraw::register(Pesos::new(50), &mut account);

    assert_eq!(Pesos::new(50), account.balance());
}

#[test]
fn test04_withdraw_value_must_be_positive() {
    let mut account = ReceptiveAccount::new();
    let withdraw_value = Pesos::new(50);

    let withdraw = Withdraw::register(withdraw_value, &mut account);

    assert_eq!(withdraw_value, withdraw.value());
}

#[test]
fn test05_receptive_account_knows_registered_transactions() {
    let mut account = ReceptiveAccount::new();

    let deposit = Deposit::register(Pesos::new(100), &mut account);
    let withdraw = Withdraw::register(Pesos::new(50), &mut account);

    assert!(account.has_registered(&deposit));
    assert!(account.has_registered(&withdraw));
}

#[test]
fn test06_receptive_account_do_not_know_not_registered_transactions() {
    let account = ReceptiveAccount::new();

    let deposit = Deposit::for_amount(Pesos::new(100));
    let withdraw = Withdraw::for_amount(Pesos::new(50));

    assert!(!account.has_registered(&deposit));
    assert!(!account.has_registered(&withdraw));
}

#[test]
fn test07_account_knows_its_transactions() {
    let mut account = ReceptiveAccount::new();

    let deposit = Deposit::register(Pesos::new(50), &mut account);

    assert_eq!(1, account.transactions().len());
    assert!(account.has_registered(&deposit));
}
