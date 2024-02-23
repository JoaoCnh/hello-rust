pub fn move_semantics2() {
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

pub fn move_semantics3() {
    let mut vec0 = Vec::new();

    let vec1 = fill_vec_2(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec_2(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

pub fn move_semantics4() {
    let mut vec1 = fill_vec_3(); 

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec_3() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}