// /// 우리는 맞춤 운동 계획을 생성하는 앱을 만드는 스타트업에서 일한다.
// /// 백엔드는 러스트로 작성되어 있고, 운동 계획을 생성하는 알고리즘은 앱 사용자의 나이, 체질량 지수, 선호도, /// 최근 운동들과 그들이 지정한 강도 숫자와 같은 많은 다른 요소들을 고려한다.
// /// 이 예제에 사용되는 실제 알고리즘은 중요하지 않다.
// /// 중요한 것은 이 알고리즘에 몇 초가 걸린다는 것이다. 이 알고리즘을 우리가 필요할 때 한 번만 호출하기를
// /// 원하고, 그래서 사용자가 필요 이상으로 기다리지 않게 만들고 싶다.

// use std::thread;
// use std::time::Duration;

// // fn simulated_expensive_calculation(intensity: u32) -> u32 {
// //     println!("calculating slowly...");
// //     thread::sleep(Duration::from_secs(2));
// //     intensity
// // }

// fn main() {

//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number
//     );
// }

// // fn generate_workout(intensity: u32, random_number: u32) {
// //     if intensity < 25 {
// //         println!(
// //             "Today, do {} pushups!",
// //             simulated_expensive_calculation(intensity)
// //         );
// //         println!(
// //             "Next, do {} situps!",
// //             simulated_expensive_calculation(intensity)
// //         )
// //     } else {
// //         if random_number == 3 {
// //             println!("Take a break today! Remember to stay hydrated!");
// //         } else {
// //             println!(
// //                 "Today, run for {} minutes!",
// //                 simulated_expensive_calculation(intensity)
// //             )
// //         }
// //     }
// // }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_closure(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }


/// caculation에 클로저를 담고, 선택적인 결과를 value에 담는 Cacher 구조체 정의하기


fn main() {
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
}



// Cacher 구조체는 제너릭 타입 T의 calculation 필드를 갖는다. T에 대한 트레잇 바운드는 Fn 트레잇을 
// 사용하여 그것이 클로저라는 것을 기술한다. caculation 필드에 저장하고자 하는 클로저는 하나의 u32 타입 //파라미터 (Fn 다음에 괄호안에 명시됨)을 갖고 u32 (-> 다음에 명시됨) 타입의 값을 반환해야 한다.
