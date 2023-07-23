enum WebEvent {
    PageLoad,                 // 페이로드가 없는 유형
    KeyPress(char),           // 튜플 구조체 유형
    Click { x: i64, y: i64 }, // 완전한 구조체 유형
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad                  => println!("page loaded"),
        WebEvent::KeyPress(c)         => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}