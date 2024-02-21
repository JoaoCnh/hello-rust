pub fn primitive_types1() {
    // Booleans (`bool`)
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true;
    if is_evening {
        println!("Good evening!");
    }
}

pub fn primitive_types2() {
    // Characters (`char`)
    check('A');
    check('2');
    check('📦');
}

fn check(character: char) {
    if character.is_alphabetic() {
        println!("Alphabetical!");
    } else if character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

pub fn primitive_types3() {
    let a = ['a';150];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}

pub fn primitive_types4() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    if nice_slice == [2, 3, 4] {
        println!("Nice slice!");
    } else {
        println!("Not quite what I was expecting... I see: {:?}", nice_slice);
    }
}

pub fn primitive_types5() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

pub fn primitive_types6() {
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);
}