use chrono;

#[derive(Debug, PartialEq)]
pub struct Wallet {
    holder: String,
    balance: f64,
    is_active: bool,
    minimun_transaction: f32,
    number_of_transactions: u32,
}

pub fn initialize_wallet() -> Wallet {
    println!("💰 My wallet");
    let wallet_holder = "Alberto Basalo";
    println!("Wallet holder: {}", wallet_holder);
    let wallet_balance = 0.0;
    println!("Wallet balance: {}", wallet_balance);
    #[derive(Debug)]
    enum CurrencyTypes {
        USD,
        EUR,
        Bitcoin,
        Ether,
    }
    let wallet_currency = CurrencyTypes::Ether;
    println!("Wallet currency: {:?}", wallet_currency);
    let number_of_transactions: u32 = 0;
    println!("Number of transactions: {}", number_of_transactions);
    const MINIMUM_TRANSACTION: f32 = 0.01;
    println!("Minimum Transaction : {}", MINIMUM_TRANSACTION);
    let is_active: bool = true;
    println!("Is active: {}", is_active);
    let creation_date = chrono::Local::now();
    println!("Creation date: {}", creation_date);
    let expiration_date = creation_date + (chrono::Duration::days(365 * 2));
    println!("Expiration date: {}", expiration_date);
    #[derive(Debug)]
    enum TransactionType {
        Deposit,
        Withdrawal,
        Transfer,
    }
    #[derive(Debug)]
    struct Transaction {
        date: chrono::DateTime<chrono::Local>,
        amount: f32,
        transaction_type: TransactionType,
    }
    let new_transaction = Transaction {
        date: chrono::Local::now(),
        amount: 0.01,
        transaction_type: TransactionType::Deposit,
    };
    println!("New transaction: {:?}", new_transaction);
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
    // test the name is snake_case of the scenario
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
        assert_eq!(actual, expected);
    }
}
