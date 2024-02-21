pub fn functions1() {
    call_me();
}

fn call_me() {
    println!("call me fn");
}

pub fn functions2() {
    call_me_2(3);
}

fn call_me_2(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn functions3() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn functions4() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}