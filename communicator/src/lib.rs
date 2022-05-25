// mod network {
//     fn connect() {

//     }
// }

// mod client {
//     fn connect() {

//     }
// }

// communicator
//  ├── network
//  └── client


// #![allow(unused)]
// fn main() {
// mod network {
//     fn connect() {
//     }

//     mod client {
//         fn connect() {
//         }
//     }
// }
// }

// communicator
//  └── network
//      └── client


// #![allow(unused)]
// fn main() {
// mod client {
//     fn connect() {
//     }
// }

// mod network {
//     fn connect() {
//     }

//     mod server {
//         fn connect() {
//         }
//     }
// }
// }

// communicator
// ├── client
// └── network
//     └── server

// 만일 위 모듈들이 여러 개의 함수를 갖고 있고 이 함수들이 길어지고 있다면 우리가 작업하고자 하는
// 코드를 찾으려고 이 파일을 스크롤 하기가 까다로워질 것이다.

mod client;
mod network;


// 위에서는 여전이 client 모듈을 선언하고 있지만, 코드 블록을 세미콜론으로 대체함으로써 
// 우리는 러스트에게 client 모듈의 스코프 내에 정의된 코드를 다른 위치에서 찾으라고 말하는 것이다.
// 숨참고 럽 다이브

// mod client {
//     // contents of client.rs
// }



// network::connect 함수와 client::connect 함수
// 이들은 완전히 다른 기능을 갖고 있을 수 있고, 서로 다른 모듈에 정의되어 있기 때문에
// 함수 이름이 서로 부딪힐 일은 없다.

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}

        secret_function()
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
