// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];

//     let mut largest = numbers[0];

//     for number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];

//     let mut largest = numbers[0];

//     for number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest = numbers[0];

//     for number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];

//     let result = largest(&numbers);
//     println!("The largest number is {}", result);

//     let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&numbers);
//     println!("The largest number is {}", result);
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let numbers = vec![34, 50, 25, 100, 65];

//     let result = largest(&numbers);
//     println!("The largest number is {}", result);

//     let chars = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&chars);
//     println!("The largest char is {}", result);
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
