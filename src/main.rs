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

    // tests exercises
    rustlings::tests::is_even(2);
    rustlings::tests::times_two(2);

    // ifs exercises
    rustlings::ifs::bigger(2, 4);

    // strings exercises
    rustlings::strings::strings1();
    rustlings::strings::strings2();
    rustlings::strings::strings3();

    // move semantics exercises
    rustlings::move_semantics::move_semantics2();
    rustlings::move_semantics::move_semantics3();
    rustlings::move_semantics::move_semantics4();
}
