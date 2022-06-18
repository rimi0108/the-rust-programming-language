// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     let third: Option<&i32> = v.get(2);
// }
// // 인덱스 문법 혹은 get 메소드를 사용하여 벡터 내의 아이템에 접근

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     // let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);
//     // 첫번째 변수는 panic!을 일으키는데, 이는 존재하지 않는 요소를 참조하기 때문이다.
//     // get 함수에 벡터 범위를 벗어난 인덱스가 주어졌을 때는 패닉 없이 None이 반환된다.
// }

#![allow(unused)]
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
