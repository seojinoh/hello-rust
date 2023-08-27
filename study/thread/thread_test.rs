use std::{thread, time};
use std::time::Duration;

// 3초 간 1초에 한 번 메시지를 표시하는 함수
fn sleep_print(word: &str) {
    for i in 1..=3 {
        println!("{}: i={}", word, i);
        thread::sleep(time::Duration::from_millis(1_000));
    }
}

fn main() {
    // 스레드를 이용하지 않는 경우
    println!("--- 스레드 없음 ---");

    sleep_print("스레드 없음");

    // 스레드를 이용하는 경우
    println!("--- 스레드 이용 ---");

    // 스레드 1
    thread::spawn(|| {
        sleep_print("토마토");
    });

    // 스레드 2
    thread::spawn(|| {
        sleep_print("스위스");
    });

    // 스레드 3
    thread::spawn(|| {
        sleep_print("별똥별");
    });

    // 스레드 4
    // - 클로저를 이용하는 경우, 클로저를 사용하는 함수 안에서 유효한 변수를 이용할 수 있으며, 환경 캡처라 부름
    // - 캡처한 변수에서 소유권 이동이 필요한 경우, 클로저에 move를 지정해 명시적으로 소유권을 이동해야 함
    let str = "우영우";

    thread::spawn(move || {
        sleep_print(str);
    });

    sleep_print("기러기");
}