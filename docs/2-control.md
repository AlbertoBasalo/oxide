# 2 - Control

## 2.1 For loop


```rust
// Classic for loop with index
for i in 0..transactions.len() {
    let transaction = &mut transactions[i];
    println!("Transaction: {:?}", transaction);
}
// For loop to iterate through a vector
for transaction in transactions.iter_mut() {
    println!("Transaction: {:?}", transaction);
}
```


## 2.2 Conditionals

```rust
// simple conditionals
if transaction.transaction_type == variables::TransactionType::Deposit {
    balance += transaction.amount;
} else {
    balance -= transaction.amount;
}
// nested if conditionals
if balance < 0.0 {
    println!("😖 You have no money")
} else if balance < 100.0 {
    println!("😃 You have money")
} else {
    println!("🤑 You have a lot of money")
}

```

## 2.3 Switch

### Really match

```rust
// match, not switch
match transaction.transaction_type {
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
```




### [Back to index](https://github.com/AtomicBuilders/oxide/blob/main/docs/index.md)