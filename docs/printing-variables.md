# Printing variables

## {}

```rust
println!("{}", var);
```

- Primitive 타입의 데이터를 출력 가능
- `std::fmt::Display`를 호출

## {:?}, {:#?}

```rust
println!("{:?}", arr);
```

- 배열이나 튜플 등 복잡한 타입이나 Struct와 같은 Custom 타입을 출력 가능
- `std::fmt::Debug`를 이미 구현하고 있기 때문에 출력 가능
- Struct는 Custom 타입으로 `std::fmt::Debug`가 구현되어 있지 않지만, `#[derive(Debug)]` Attribute를 지정하여 사용 가능
- `{:?}`: 한 라인으로 모두 출력
- `{:#?}`: 여러 라인으로 정렬하여 출력

## dbg! 매크로

```rust
dbg!(&var);
```

- `#[derive(Debug)]`를 붙인 Struct에 사용 가능
- 소스 파일명과 라인 수, 데이터 값이 함께 출력됨

```bash
[src/main.rs:16] &var = Point {
  width: 100,
  height: 50,
}
```