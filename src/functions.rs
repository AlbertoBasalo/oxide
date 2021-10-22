use crate::variables;
use std::io;

// simple function
// receiving references, also called borrowing
pub fn process_transactions(transactions: &Vec<variables::Transaction>) {
    let balance = calculate_balance(&transactions);
    // anonymous function saved in a variable
    let print_wealth = |wealth| match wealth {
        x if x < 0.0 => println!("😖 You have no money"),
        x if x < 100.0 => println!("😃 You have money"),
        _ => println!("🤑 You have a lot of money"),
    };
    // calls the anonymous function, by its variable name
    print_wealth(balance);
}

fn calculate_balance(transactions: &Vec<variables::Transaction>) -> f32 {
    let mut balance = 0.0;
    for transaction in transactions {
        // the result is a complex type with a value or an error
        let new_balance = get_new_balance(balance, transaction);
        // need to switch on the result
        match new_balance {
            Ok(new_balance) => balance = new_balance,
            Err(error) => println!("💥 Not processed: {}", error),
        }
    }
    // implicit return
    balance
}

// current is a value not a pointer (no effect on the original value)
// function with multiple return values
fn get_new_balance(current: f32, transaction: &variables::Transaction) -> Result<f32, io::Error> {
    if transaction.amount < 0.0 {
        // return errors on guard failure
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Negative amount",
        ));
    }
    // changes to local variable named current
    // initialized with a copy of the caller value
    let mut balance = current;
    match transaction.transaction_type {
        variables::TransactionType::Deposit => balance += transaction.amount,
        variables::TransactionType::Withdrawal => balance -= transaction.amount,
        _ => {
            balance -= transaction.amount;
        }
    }
    // return values on happy path
    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_process_transactions() {
        let transactions = vec![
            variables::Transaction {
                date: chrono::Local::now(),
                transaction_type: variables::TransactionType::Deposit,
                amount: 0.03,
            },
            variables::Transaction {
                date: chrono::Local::now(),
                transaction_type: variables::TransactionType::Withdrawal,
                amount: 0.01,
            },
            variables::Transaction {
                date: chrono::Local::now(),
                transaction_type: variables::TransactionType::Withdrawal,
                amount: -0.01,
            },
        ];
        process_transactions(&transactions);
        // act
        let actual = calculate_balance(&transactions);
        // assert
        let expected = 0.02;
        assert_eq!(actual, expected);
    }
}
