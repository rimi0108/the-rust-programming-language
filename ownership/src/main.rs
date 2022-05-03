// // 변수의 스코프

// // 변수 s는 스트링 리터럴을 나타내는데, 스트링 리터럴의 값은 우리의 프로그램이 텍스트 내에
// // 하드 코딩 되어있다. 변수는 선언된 시점부터 현재의 스코프가 끝날 때까지 유효하다 .
// fn main() {          // 변수 s는 유효하지 않다. 아직 선언되지 않았으니까!
//     let s = "hello"; // s는 이 지점부터 유효하다.

//     // s를 가지고 무언가 한다.
// } // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않다.

// // 1. 스코프 안에서 s가 등장하면, 유효하다.
// // 2. 이 유효기간은 스코프 밖으로 벗어날 때까지 지속된다.


// // String 타입
// // 위에서 본 스트링 리터럴 값은 프로그램 안에 하드코딩 되어있다.
// // 문자열 값은 편리하지만, 텍스트를 필요로 하는 모든 경우에 항상 적절하지는 않다.
// // 그 중 한가지 이유로, 문자열 값은 불변이다. (immutable)
// // 또다른 이유는 모든 문자열이 우리가 프로그래밍 하는 시점에서 다 알 수 있는 것이 아니란 점이다.
// // 예를 들면, 사용자의 입력을 받아 저장하고 싶을 경우이다.
// // 이 경우들에 대하여, 러스트는 두 번째 문자열 타입인 String을 제공한다.
// // 이 타입은 힙에 할당되고 컴파일 타임에는 우리가 알 수 없는 양의 텍스트를 저장할 수 있다.
// fn main() {
//     let mut s = String::from("hello"); // 더블 콜론은 우리가 string_from 과 같은 이름을 쓰기 보다는 String 타입 아래의
//     // from 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자이다. 이러한 종류의 문자열이 변경이 가능하다.
//     s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여준다.
//     println!("{}", s); // "hello, world!"를 출력한다.
// }
// // 왜 String은 변할 수 있는데(mutable) 스트링 리터럴은 불변일까? (immutable)
// // 차이점은 두 타입이 메모리를 쓰는 방식에 있다.


// // 변수와 데이터가 상호작용하는 방법: 이동(move)
// // 여러 개의 변수들은 러스트에서 서로 다른 방식으로 같은 데이터에 대해 상호작용을 할 수 있다.
// fn main() {
//     // // 변수 x의 정수값을 y에 대입하기
//     // let x = 5;
//     // let y = x;
//     // // 아마도 다른 언어들에서의 경험을 토대로 어떤 일이 벌어지는 지 추측할 수 있다. "정수값 5를 x에 묶어놓고 x의 값의 복사본
//     // // 을 만들어 y에 묶는다." 우리는 이제 x와 y 두 개의 변수를 갖게 되었고, 둘 다 5와 같다.
//     // // 정수값이 결정되어 있는 고정된 크기의 단순한 값이고, 5라는 값들이 스택에 푸쉬된다.

//     // String 버전
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // 이 코드는 위 코드와 매우 유사해 보여서, 동작하는 방식도 동일할 것이라고 가정할 지도 모른다.
//     // 즉, 두 번째 줄이 s1의 복사본을 만들어서 s2에 묶어놓는 식으로 말이다. 하지만 그렇지 않다.
//     // README에서 설명 계속..
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }

// // 소유권과 함수
// fn main() {
//     let s = String::from("hello"); // s가 스코프 안으로 들어왔다.
//     takes_ownership(s); // s의 값이 함수 안으로 이동했다.
//                         // 그리고 이제 더이상 유효하지 않다.
//     let x = 5;  // x가 스코프 안으로 들어왔다.

//     makes_copy(x);  // x가 함수 안으로 이동했지만, i32는 Copy가 
//                     // 되므로, 이후에 계속 사용해도 된다.
// } // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나간다. 하지만 s는 이미
//   // 이동되었으므로, 별다른 일이 발생하지 않는다.

// fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔다.
//     println!("{}", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출된다. 메모리는 해제되었다.

// fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어왔다.
//     println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어났다. 별다른 일은 발생하지 않는다.

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership은 반환값을 s1에게 이동시킨다.
    
//     let s2 = String::from("hello"); // s2가 스코프 안에 들어왔다.

//     let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로 이동되었고, 이 함수가 반환값을 s3으로도 이동시켰다.
// } // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출된다. s2는 스코프 밖으로 벗어났지만 이동되었으므로 아무 일도 일어나지 않는다. 
//   // s1은 스코프 밖으로 벗어나서 drop이 호출된다.

// fn gives_ownership() -> String {    // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킨다.
//     let some_string = String::from("hello");    // some_string이 스코프 안으로 들어왔다.

//     some_string // some_string이 반환되고, 호출한 쪽의 함수로 이동한다.
// }

// fn takes_and_gives_back(a_string: String) -> String {   // a_string이 스코프 안으로 들어왔다.
//     a_string    // a_string은 반환되고, 호출한 쪽의 함수로 이동된다.
// }

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
// 위 코드는 너무 나간 절차이고 일반적인 개념으로는 과한 작업이 된다. 운좋게도, 러스트는
// 이를 위한 기능을 갖고 있으며, 참조자(references)라고 부른다.