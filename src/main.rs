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
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;

        // return the updated balance
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;

        // return the updated balance
        self.balance
    }

    fn account_summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn bank_summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.account_summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    // create new bank instance
    let mut bank = Bank::new();

    // create new account instance (in immutable way of borrowing)
    let mut account = Account::new(1, String::from("Ilia"));

    // deposit money to the account
    account.deposit(750);

    // withdraw money from the account
    account.withdraw(300);

    // show an account summary as a string
    account.account_summary();

    // add account to the Bank
    bank.add_account(account);

    // display bank summary
    println!("{:#?}", bank.bank_summary());

    // display total balance
    println!("{}", bank.total_balance());
}
