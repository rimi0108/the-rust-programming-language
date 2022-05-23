fn connect() {

}

// 이미 src/lib.rs 안에 client 모듈을 mod를 이용하여 선언했기 때문에
// 이 파일 안에는 mod 선언이 필요없다.__rust_force_expr!
// 이 파일은 단지 client 모듈의 내용물만 제공할 뿐이다.
// 만일 mod client를 여기 또 집어넣는다면, 이는 client 모듈 내에 서브모듈 client를 만들게 된다.
// 러스트는 기본적으로 src/lib.rs 만 찾아볼줄 안다/
