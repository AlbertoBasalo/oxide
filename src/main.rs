fn main() {
    let program = "oxide".to_string();
    let line = get_start_line(&program);
    println!("{}", line);
}

fn get_start_line(program: &String) -> String {
    return format!("{}{}{}", "🚀 ", program, " started");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_get_start_line() {
        let input = "test".to_string();
        let actual = get_start_line(&input);
        let expected = "🚀 test started";
        assert_eq!(actual, expected);
    }
}
