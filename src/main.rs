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

    // functions exercises
    rustlings::functions::functions1();
    rustlings::functions::functions2();
    rustlings::functions::functions3();
    rustlings::functions::functions4();

    // primitive types exercises
    rustlings::primitive_types::primitive_types1();
    rustlings::primitive_types::primitive_types2();
    rustlings::primitive_types::primitive_types3();
    rustlings::primitive_types::primitive_types4();
    rustlings::primitive_types::primitive_types5();
    rustlings::primitive_types::primitive_types6();
}
