#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 1000;
}

fn main() {
    // 1 - immutable way of borrowing
    let account = Account::new(1, String::from("me"));

    // read-only reference - uses `&` operator:
    print_account(&account);

    println!("{:#?}", &account);
    
    // 2 - mutable borrowing
    let mut new_account = Account::new(2, String::from("you"));

    // mutable reference uses `mut` keyword along with `&` operator
    // NOTE: you can't create a mutable ref in case there'are immutable refs currently in use
    change_account(&mut new_account);
    
    println!("{:#?}", &new_account);    
}
