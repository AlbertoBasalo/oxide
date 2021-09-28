use chrono;

pub fn initialize_wallet() {
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
    let number_of_transactions = 0;
    println!("Number of transactions: {}", number_of_transactions);
    const MINIMUM_TRANSACTION_AMOUNT: f32 = 0.01;
    println!("Minimum Transaction Amount: {}", MINIMUM_TRANSACTION_AMOUNT);
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
}
