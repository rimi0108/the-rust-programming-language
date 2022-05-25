// extern crate communicator

// fn main() {
//     communicator::client::connect();
// }

// communicator 라이브러리 크레이트를 가져오기 위해 extern crate 명령을 사용한다.
// 카고는 src/main.rs를 바이너리 크레이트의 루트 파일로 취급하는데,
// 이 바이너리 크레이트는 src/lib.rs가 루트 파일인 이미 있던 라이브러리 크레이트와는 별개이다.
// 대부분의 기능은 라이브러리 크레이트 안에 있고, 바이너리 크레이트는
// 이 라이브러리 크레이트를 이용한다.

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {

            }
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}