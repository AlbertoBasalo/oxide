fn main() {
    let program = "oxide".to_string();
    let line = get_start_line(&program);
    println!("{}", line);
}

fn get_start_line(program: &String) -> String {
    return format!("{}{}{}", "🚀 ", program, " started");
}
