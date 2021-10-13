use crate::variables;

pub fn calculate_balance_1(mut transactions: Vec<variables::Transaction>) -> f32 {
    let mut balance: f32 = 0.0;
    // Classic for loop with index
    for i in 0..transactions.len() {
        let transaction = &mut transactions[i];
        // simple conditionals
        if transaction.transaction_type == variables::TransactionType::Deposit {
            balance += transaction.amount;
        } else {
            balance -= transaction.amount;
        }
    }
    // nested if conditionals
    if balance < 0.0 {
        println!("😖 You have no money")
    } else if balance < 100.0 {
        println!("😃 You have money")
    } else {
        println!("🤑 You have a lot of money")
    }
    balance
}

pub fn calculate_balance_2(mut transactions: Vec<variables::Transaction>) -> f32 {
    let mut balance: f32 = 0.0;
    // for loop to iterate through a vector
    for transaction in transactions.iter_mut() {
        // match statement , not switch
        match transaction.transaction_type {
            variables::TransactionType::Deposit => balance += transaction.amount,
            variables::TransactionType::Withdrawal => balance -= transaction.amount,
            _ => {
                // default case
                balance -= transaction.amount;
            }
        }
    }
    // match statement , not switch
    match balance {
        x if x < 0.0 => println!("😖 You have no money"),
        x if x < 100.0 => println!("😃 You have money"),
        _ => println!("🤑 You have a lot of money"),
    }
    balance
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_calculate_balance() {
        // arrange
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
        ];
        // act
        let actual = calculate_balance_1(transactions);
        // assert
        let expected = 0.02;
        assert_eq!(actual, expected);
    }
}
