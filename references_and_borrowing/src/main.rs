// // 1
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// 2
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// } 

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
// // error!!! `some_string` is a `&` reference, so the data it refers to 
// cannot be borrowed as mutable

// 3 
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// } 

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }
// // it works!!

// 4
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
// expected named lifetime parameter
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from

