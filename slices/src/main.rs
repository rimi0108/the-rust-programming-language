// 1
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 5를 갖게 될 것이다.

    s.clear(); // 이 코드는 String을 비워서 ""로 만들게 된다.

    // word는 여기서 여전히 5를 갖고 있지만, 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없다.
    // word는 이제 완전 유효하지 않다.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}