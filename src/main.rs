 fn main() {
    println!("Hello, world!");
    tuple();
    array();
    person();
    foo();
}

fn tuple() {
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{},{}", t.0, t.1);
}

fn array() {
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);
}

struct Person {
    name: String,
    age: u32,
}

fn person() {
    let p = Person {
        name: String::from("Jhon"),
        age: 8,
    };
    println!("{} | {}", p.name, p.age);
}

#[derive(Debug)]
enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 },
}

fn foo() {
    let e1 = Event::Quit;
    let e2 = Event::KeyDown(1);
    let e3 = Event::MouseDown { x: 10, y: 10 };
    println!("{:?} | {:?} | {:?}", e1, e2, e3);
}

// 標準ライブラリ
// データが存在しない場合はNone、存在する場合はその型をTとした時
pub enum Option<T> {
    None,
    Some(T),
}

// Resultは処理の結果が成功か、エラーかを表現できる列挙型
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn res() {
    let result: Result<i32, String> = Result::Ok(200);

    match result {
        Result::Ok(code) => println!("code: {}", code),
        Result::Err(err) => println!("Err: {}", err),
    }
}

fn res2() {
    // Okの場合はOkの中身を展開、Errの場合はunwrap_or(-1)の値を展開
    // let result: Result<i32, String> = Result::Ok(200);
    // println!("code: {}", result.unwrap(-1)); // "code: 200"
    // let result: Result<i32, String> = Result::Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1)); // "code: -1"
}

// and_then はOkだった場合にだけ、指定した関数を実行することができる
fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Result::Ok(100)
}

// fn func2() {
    // let result: Result<i32, String> = Result::Ok(200);
    // let next_result = result.and_then
// }
