# 0 - Hello world

## 0.1 Install tools

- [Official download site](https://www.rust-lang.org/tools/install)

- [Windows ONLY Visual C++ build tools](https://visualstudio.microsoft.com/es/visual-cpp-build-tools/)

- [VS Code extensions](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)


## 0.2 Hello world

```bash
 # generate a folder for your program
cargo new oxide
cd cargo new oxide
code .
```

### Show me the code

On `src/main.rs` file [View code](https://github.com/AtomicBuilders/oxide/blob/main/src/main.rs)

```rust
// the main function, called on startup
fn main() {
    let program: String; // declaring a variable of type String
    program = "oxide".to_string(); // assign a value (transformed) to the variable
    let line = get_start_line(program); // initialize and assign a value
    println!("{}", line); // print the value of the variable
}
/*
    Function that returns a formated string
    needs input and output types to be declared
*/
fn get_start_line(program: String) -> String {
    return format!("{}{}{}", "🚀 ", program, " started");
}
```


### Build and run

```bash
cargo run  # builds the executable file and runs it
cargo build # generates an executable
./target/debug/oxide.exe  # runs it on your platform
```

## 0.3 Testing

### Tests goes on the same file

```rust
/*
    Test functions are on the same file than the main function
    They have to be decorated with #[cfg(test)]
*/
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // test the name is snake_case of the scenario
    fn it_should_get_start_line() {
        // arrange
        let input = "test".to_string();
        // act
        let actual = get_start_line(input);
        // assert
        let expected = "🚀 test started";
        assert_eq!(actual, expected);
    }
}
```

### Run tests

```bash
cargo test # builds and runs on memory
```

### [Back to index](https://github.com/AtomicBuilders/oxide/blob/main/docs/index.md)