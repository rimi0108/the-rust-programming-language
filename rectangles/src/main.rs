// fn main() {
//     let length1 = 50;
//     let width1 = 30;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1, width1)
//     );
// }

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// 튜플을 이용한 리팩터링

// fn main() {
//     // let length1 = 50;
//     // let width1 = 30;
//     let rect1 = (50, 30);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 위에서는 `length`가 튜플 인덱스 `0`이고 `width`가 튜플 인덱스 `1`이라는 점을 꼭 기억해야 한다.
// 만일 다른 누군가가 이 코드를 이용해서 작업한다면, 그들 또한 이 사실을 알아내어 기억해야 한다.
// 우리의 코드 내에 데이터의 의미를 전달하지 않았기 때문에, 이 값들을 잊어먹거나 혼동하여 에러를 발생시키는 일이 쉽게 발생할 것이다.

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}