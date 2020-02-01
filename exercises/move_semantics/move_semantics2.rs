// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    //let mut vec0 = Vec::new(); // method 1 
    let vec0 = Vec::new(); // method 2 & 3

    //let mut vec1 = fill_vec(&mut vec0); // method 1
    //let mut vec1 = fill_vec(& vec0); // method 2
    let mut vec1 = fill_vec(vec0.to_vec()); // method 3

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> { // method 3
//fn fill_vec(vecc: & Vec<i32>) -> Vec<i32> { // method 2
//fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> { //method 1
    
    // method 1 does not need mut vec at all.
    // let mut vec = vecc.to_vec(); // method 2
    let mut vec = vec; // method 3

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec.to_vec() // method 1
    vec // method 2
}
