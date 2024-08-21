use std::io::{stdin, stdout, Write};
pub fn input(question: Option<&str>) -> String {
    print!("{}", question.unwrap_or(""));
    stdout().flush().unwrap();

    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    out.truncate(out.len() - 1); // \n
    out.truncate(out.len() - 1); // \r
    out
}