// 사용자 계정정보를 저장하는 `User` 구조체 정의
// #[derive(Debug)] -> Debug 를 위해 구조체 위에 붙여 줘야 함. 
// 붙여줘야 구조체 print 할 수 있음!
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// fn main() {
//     // 구조체 `User`의 인스턴스 생성하기
//     let mut user1 = User {
//         email: String::from("hahaha@haha.com"),
//         username: String::from("rimi"),
//         active: true,
//         sign_in_count: 1,
//     };
//     // `User` 인스턴스의 `email` 필드 변경하기
//     user1.email = String::from("anotherhahaha@hahaha.com");
// }

fn main() {
    let email = String::from("huhuhu@hfuhdu.com");
    let username = String::from("rimi");

    println!("{:?}", build_user(email, username));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}