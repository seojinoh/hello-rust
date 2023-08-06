/// Lifetimes
///
/// Rust는 컴파일 시, Rust 컴파일러의 탑재된 Borrow checker로 유효성을 검사한다.
///
/// Rust 컴파일러는 코드를 분석하여, 대부분의 경우 수명을 자동으로 추론하며,
/// 이를 'Lifetime Elision'이라 한다.
///
/// Rust 컴파일러의 수명 추론 주요 규칙
/// 1. 모든 참조는 수명을 갖음
/// 2. 파라미터의 참조가 1개라면, 해당 참조의 수명이 모든 리턴에 적용됨
/// 3. 파라미터로 self 또는 &mut self가 있다면, 해당 self의 수명이 모든 리턴에 적용됨
///
/// Rust 컴파일러가 수명을 추론할 수 없는 경우, 수명 애노테이션이 필요하며,
/// 없을 경우 컴파일 시 에러가 발생한다.
///
/// 수명 애노테이션이 참조의 유효 기간에 영향을 미치지는 않으며,
/// 함수의 제네릭 파라미터로 어떠한 타입의 값도 전달할 수 있는 것처럼,
/// 제네릭 수명 파라미터로 어떠한 수명의 참조도 전달할 수 있다.
///
/// 변수에 사용 시
/// - '(작은 따옴표)로 시작하며, 대부분 소문자 알파벳을 사용하며, 주로 'a를 사용
/// - &'a i32
/// - &'a mut i32
///
/// 함수에 사용 시
/// - 제네릭 파라미터처럼 함수 이름과 파라미터 사이에 <>(꺽쇠 괄호)에 제네릭 수명 파라미터를 선언
///
fn main() {
    // Dangling Reference
    // test_dangling_reference();

    // Generic Lifetimes Of Function 1
    // test_generic_lifetimes_of_function_1();

    // Generic Lifetimes Of Function 2
    // test_generic_lifetimes_of_function_2();
}

///
/// Test Dangling Reference
///
/// 1. Bad case!
///    - 변수 r은 Block 내부에서 수명이 유효한 변수 x를 참조하며,
///      변수 r을 Block 외부에서 출력을 시도하기 때문에 컴파일 시 에러가 발생한다.
///
/// 2. Good case!
///    - 변수 r을 변수 x의 수명이 유효한 Block 내부에서 출력을 시도한다.
///
fn test_dangling_reference() {
    // Bad case!
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r);

    // Good case!
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    //
    //     println!("r: {}", r);
    // }
}

///
/// Test Generic Lifetimes Of Function 1
///
/// 1. Bad case!
///    - longest 함수는 두 개의 파라미터는 문자열 슬라이스를 참조하는 참조자이며,
///      어떤 파라미터를 리턴하게 될지 알 수 없기 때문에 컴파일 시 에러가 발생한다.
///
/// 2. Good case!
///    - longest 함수에 수명 애노테이션으로 두 개의 파라미터 문자열 슬라이스와 리턴의 수명이 동일하다고 명시적으로 선언하여,
///      Rust 컴파일러는 수명의 유효성을 판단할 수 있다.
///
fn test_generic_lifetimes_of_function_1() {
    let str_1 = "Tom";
    let str_2 = "Jerry";

    // Bad case!
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    //
    // println!("longest: {}", longest(str_1, str_2));

    // Good case!
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    //
    // println!("longest: {}", longest(str_1, str_2));
}

///
/// Test Generic Lifetimes Of Function 2
///
/// 1. Bad case!
///    - longest 함수는 두 개의 파라미터 문자열 슬라이스와 리턴의 수명이 동일하다고 명시적으로 선언되어,
///      'a의 수명은 str_1, str_2 인자 중 수명이 짧은 str_2 인자의 수명으로 결정되어,
///      str_2 인자의 수명이 유효하지 않은 Block 외부에서는 리턴의 값을 사용할 수 없다.
///
/// 2. Good case!
///    - str_2 인자의 수명이 유효한 Block 내부에서 리턴의 값을 사용한다.
///
fn test_generic_lifetimes_of_function_2() {
    let str_1 = "very very very long string";

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Bad case!
    // let result: &str;
    //
    // {
    //     let str_2 = "short string";
    //     let result = longest(str_1, str_2);
    // }
    //
    // println!("longest: {}", result);


    // Good case!
    // let result: &str;
    //
    // {
    //     let str_2 = "short string";
    //     let result = longest(str_1, str_2);
    //
    //     println!("longest: {}", result);
    // }
}