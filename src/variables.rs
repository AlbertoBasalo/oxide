use chrono;

// structs for complex types
#[derive(Debug, PartialEq)]
pub struct Wallet {
    holder: String,
    balance: f64,
    is_active: bool,
    minimun_transaction: f32,
    number_of_transactions: u32,
}
// A kind of string enum
#[derive(Debug)]
pub enum CurrencyTypes {
    USD,
    EUR,
    Bitcoin,
    Ether,
}
#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}
#[derive(Debug)]
pub struct Transaction {
    pub date: chrono::DateTime<chrono::Local>,
    pub amount: f32,
    pub transaction_type: TransactionType,
}

pub fn initialize_wallet() -> Wallet {
    println!("💰 My wallet");
    // standard way to declare, with implicit type, and assign value to a variable
    let wallet_holder = "Alberto Basalo";
    println!("Wallet holder: {}", wallet_holder);
    // attention for different kinds of numbers
    let wallet_balance: f64 = 0.0;
    println!("Wallet balance: {}", wallet_balance);
    let wallet_currency = CurrencyTypes::Ether;
    println!("Wallet currency: {:?}", wallet_currency);
    let number_of_transactions: u32 = 0;
    println!("Number of transactions: {}", number_of_transactions);
    const MINIMUM_TRANSACTION: f32 = 0.01;
    println!("Minimum Transaction : {}", MINIMUM_TRANSACTION);
    let is_active: bool = true;
    println!("Is active: {}", is_active);
    // date and time came on their own crates
    let creation_date = chrono::Local::now();
    println!("Creation date: {}", creation_date);
    let expiration_date = creation_date + (chrono::Duration::days(365 * 2));
    println!("Expiration date: {}", expiration_date);
    let new_transaction = Transaction {
        date: chrono::Local::now(),
        amount: 0.01,
        transaction_type: TransactionType::Deposit,
    };
    println!("New transaction: {:?}", new_transaction);
    // Mutable arrays are called Vectors
    let mut transactions: Vec<Transaction> = Vec::new();
    transactions.push(new_transaction);
    println!("Transactions: {:?}", transactions);
    let my_wallet = Wallet {
        holder: wallet_holder.to_string(),
        balance: wallet_balance,
        is_active: is_active,
        minimun_transaction: MINIMUM_TRANSACTION,
        number_of_transactions: number_of_transactions,
    };
    return my_wallet;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_start_line() {
        // arrange
        // act
        let actual = initialize_wallet();
        // assert
        let expected = Wallet {
            holder: "Alberto Basalo".to_string(),
            balance: 0.0,
            is_active: true,
            minimun_transaction: 0.01,
            number_of_transactions: 0,
        };
        // requires the trait PartialEq
        assert_eq!(actual, expected);
    }
}
