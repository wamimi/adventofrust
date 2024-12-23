struct BankAccount{
    account_number: String,
    owner: String,
    balance: f64,
    is_active: bool,
}
impl BankAccount {
    // Correct deposit method - use the passed amount, not hardcoded 100.0
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited {}. New balance is {}", amount, self.balance);
    }


    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient funds".to_string())
        } else {
            self.balance -= amount;
            println!("Withdrawn {}. New balance is {}", amount, self.balance);
            Ok(())
        }
    }

    fn get_balance(&self) -> f64 {
        println!("Current balance is {}!", self.balance);
        self.balance
    }
}

impl BankAccount {
    fn create_account(account_number: String, owner: String) -> BankAccount {
        BankAccount {
            account_number,
            owner,
            balance: 0.0,
            is_active: true,
        }
    }

    // Modify transfer to actually transfer between two accounts
    fn transfer(from: &mut BankAccount, to: &mut BankAccount, amount: f64) -> Result<(), String> {
        if from.withdraw(amount).is_ok() {
            to.deposit(amount);
            Ok(())
        } else {
            Err("Transfer failed".to_string())
        }
    }
} 
fn main() {
    let mut account1 = BankAccount::create_account("123".to_string(), "John".to_string());
    let mut account2 = BankAccount::create_account("456".to_string(), "Jane".to_string());

    account1.deposit(500.0);
    let balance = account1.get_balance(); // 500.0

    match BankAccount::transfer(&mut account1, &mut account2, 200.0) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }
}
// This was my initial code 
/*
 struct BankAccount{
    account_number: String,
    owner: String,
    balance: f64,
    is_active: bool,
}
impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += 100.0;
        println!("{} is", self.balance)

    }
    fn withdraw(&mut self, amount: f64) {
        self.balance -= 100.0;
        println!("{} is", self.balance)
        }
    fn get_balance(&self) -> f64 {
            self.balance;
            println!("current balance is {}!", self.balance)
            }


}

impl BankAccount{
fn create_account(account_number: String, owner: String) -> BankAccount{
    BankAccount{
        account_number: account_number.to_string(),
        owner: owner.to_string(),
        balance: 0.0,
        is_active: true,
        }
}
}
impl BankAccount{
    fn transfer(account_number: String, amount: f64) -> BankAccount{
        BankAccount{
            account_number: account_number.to_string(),
            owner: "Nelly".to_string(),
            balance: 100.0,
            is_active: true,
            }
}
}
    

 */