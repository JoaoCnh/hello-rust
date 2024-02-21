pub fn variables1() {
    let x = 5;
    println!("x has the value {}", x);
}

pub fn variables2() {
    let x = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}

pub fn variables3() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);
}

pub fn variables4() {
    let x: i32 = 12;
    println!("Number {}", x);
}