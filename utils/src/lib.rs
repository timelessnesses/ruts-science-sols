use std::io::{stdin, stdout, Write};
pub fn input(question: Option<&str>) -> String {
    if let Some(q) = question {
        print!("{q}");
        stdout().flush().unwrap();
    };

    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    out.trim();
    out
}
