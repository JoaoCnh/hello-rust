use ferris_says::say;
use std::io::{stdout, BufWriter};
mod rustlings;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // variables exercises
    rustlings::variables::variables1();
    rustlings::variables::variables2();
    rustlings::variables::variables3();
    rustlings::variables::variables4();
}
