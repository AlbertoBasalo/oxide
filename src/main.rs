mod variables;

// the main function, called on startup
fn main() {
    let program: String; // declaring a variable of type String
    program = "oxide".to_string(); // assign a value (transformed) to the variable
    let line = get_start_line(program); // initialize and assign a value
    println!("{}", line); // print the value of the variable

    variables::initialize_wallet(); // call the function
}
/*
    Function that returns a formated string
    needs input and output types to be declared
*/
fn get_start_line(program: String) -> String {
    return format!("{}{}{}", "🚀 ", program, " started");
}

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
