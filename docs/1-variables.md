# 1 - Variables

## 1.1 Declaring variables


```rust
// Expressive way (& is a reference )
let wallet_holder : &str = "Alberto Basalo";
// Implicit way
let wallet_holder = "Alberto Basalo";
// Undefined (not recommeded)
let wallet_holder : &str;
wallet_holder = "Alberto Basalo";
```


## 1.2 Primitive Types

```rust
let wallet_holder = "Alberto Basalo";
// Attention to different kinds of numbers
let wallet_balance : f64 = 0.0;
let number_of_transactions: u32 = 42;
let is_active: bool = true;
let creation_date = chrono::Local::now();
```

## 1.3 Custom Types


### 1.3.1 Enums

```rust
#[derive(Debug)] // trait to print the value
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}
```

### 1.3.2 Structs


```rust
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
```

### 1.4 Arrays

> Actually they are called *Vector* when mutable

```rust
 // Mutable arrays are called Vectors
let mut transactions: Vec<Transaction> = Vec::new();
transactions.push(new_transaction);
```

### 1.5 Tests

```rust
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
```


### [Back to index](https://github.com/AtomicBuilders/oxide/blob/main/docs/index.md)