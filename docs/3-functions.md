# 3 - Functions

## 3.1 Simple


```rust
// simple function
// receiving references, also called borrowing
pub fn process_transactions(transactions: &Vec<variables::Transaction>) {
	println!("Do stuff...")
}
process_transactions(&transactions);

```


## 3.2 Anonymous

```rust
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
```

## 3.3 Multiple return values


### 3.3.1 Returning multiple values

```rust
// function with multiple return values
// current is a value not a pointer (no effect on the original value)
// function with multiple return values
fn get_new_balance(current: f32, transaction: &variables::Transaction) -> Result<f32, io::Error> {
    if transaction.amount < 0.0 {
        // return errors on guard failure
        return Err(io::Error::new(io::ErrorKind::InvalidInput,"Negative amount"));
    }
    var something float32 = 42
    // return values on happy path
    Ok(something)
}
```

### 3.3.2 Calling and saving the results


```rust
// the result is a complex type with a value or an error
let new_balance = get_new_balance(balance, transaction);
// need to switch on the result
match new_balance {
	Ok(new_balance) => balance = new_balance,
	Err(error) => println!("💥 Not processed: {}", error),
}
```


## 3.4 Values, not references

```rust
let mut balance = 0.0
// current is a value not a pointer (no effect on the original value)
// function with multiple return values
fn get_new_balance(current: f32, transaction: &variables::Transaction) -> Result<f32, io::Error> {
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
// sends a copy of the value of variable balance
let new_balance = get_new_balance(balance, transaction);
// but balance is not mutated inside the function
println!("current balance: {}", balance);
println!("new balance: {}", new_balance);
balance = new_balance;

```

### [Back to index](https://github.com/AtomicBuilders/gluon/blob/main/docs/index.md)