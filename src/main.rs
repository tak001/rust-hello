fn main() {
    println!("Hello, world!");
    tuple();
    array();
    person();
    foo();
    hoge();
    pbox();
    fuga();
    ihu();
    ihu2();
    ru_pu();
    wairu();
    fo();
    label();
    mat1();
    mat2();
    let x = add(1, 2);
    println!("x: {}", x);
    let s = abs(1);
    println!("s; {}", s);

    let p = Person2 {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name();
    p.say_age();

    let p2 = Person3 {
        name: String::from("Taro"),
        age: 20,
    };
    p2.say_name().say_age();

    let p3 = Person3::new("Taro", 20);
    p3.say_name().say_age();
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

// struct = interface的な
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

// 構造体を一発でだすために#[derive(Debug)]と{:?}
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

// Vec
// ベクタ型で、配列とは違い内部に収められる要素の数を増減させることができる。
// 初期値を便利に実施するためのvec![]マクロが用意されており、これを使うことで予め要素をつめた状態を作ることができる

fn hoge() {
    let v1 = vec![1, 2, 3, 4, 5]; // 1~5の数を入れて初期化
    let v2 = vec![0; 5]; // 0を5つ埋めて初期化
                         // {:?} 配列を一発で出す
    println!("{:?} | {:?}", v1, v2);

    // for
    for element in &v1 {
        println!("{}", element);
    }
}

// Box
// Boxを使うと、値はヒープ領域に確保される
// コンパイル時にサイズがわからない型を収納すること
// 大きなサイズの型の値を渡す際に、データの中身をコピーせず、ポインタで渡すこと
// 共通のトレイとを実装した様々な型を画一的にポインタで扱うこと
fn pbox() {
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'l', b'd', b'!'];
    print(Box::new(byte_array));
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

// 変数宣言
/*
* let variable = 再代入不可
* let mut variable = 再代入可能
*/
// 型推論
fn fuga() {
    let immut_val = 10;
    let mut mut_val = 20;
    mut_val += immut_val;
    println!("mut_val: {:?}", mut_val);

    // 型の明示
    let v1: u64 = 10;

    // 数値限定で、値に直接型名をくっつける
    let v2 = 10u64;
}

// 定数
/*
* const static
* const = 常に値の変更不可
* static = 変更可能
* グローバルスコープで定義したstaticの値は、どこからでも変更が可能な危険な変数になる
* そのため、この値を操作するときにはunsafeブロック内に入れる必要がある
*/

// if
fn ihu() {
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }
}

// Rustにおけるifは式なので、if式で評価した値を、変数に束縛することや、関数の引数にすることができる
fn ihu2() {
    let number = 1;
    let result = if 0 <= number { number } else { -number };
    println!("{:?}", result);
}

// loop
fn ru_pu() {
    let mut count = 0;

    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    println!("loop result: {:?}", result);
    // count: 0~9 loop result: 10
}

fn wairu() {
    let mut count = 0;

    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }
    // count: 0~9
}

// 繰り返したい回数や、処理したい要素の集合が決まっている場合に有効
fn fo() {
    let count: i32;

    for count in 0..10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for element in &array {
        println!("element: {}", element);
    }
}

/*
whileもforも繰り返しの中でbreakを使うことができる
またloop/while/forにはラベルをつけることができ、breakするときに、そのラベルを指定して繰り返すを抜けることができる
ラベル名はその先頭に'をつける必要がある
*/
fn label() {
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end"); // 表示されない
        }
        println!("main loop end"); // 表示されない
    }
}

// match(switch)
/*
型の構造に一致しているか確認できる
数値/文字列/列挙型/タプル型/構造体など
値の範囲やワイルドカードを使って広範囲に比較させることもできる
*/

fn mat1() {
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"), // アンダースコアは、あらゆる値にマッチする
    }
}

enum Color {
    Red,
    Blue,
    Green,
}

fn mat2() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }
}

fn add(a: i32, b: i32) -> i32 {
    // 関数ないの最後に、セミころんなしで記述された値を戻り値として扱うルールがある
    a + b
}

fn abs(number: i32) -> i32 {
    if number < 0 {
        // return を入れたパターン
        return -number;
    }
    number
}

//impl
/*
構造体にメソッドを加えることができる
*/

struct Person2 {
    name: String,
    age: u32,
}

impl Person2 {
    fn say_name(&self) {
        println!("I am {}", self.name);
    }

    fn say_age(&self) {
        println!("I am {} year(s) old", self.age);
    }
}

/*
メソッドの戻り値に自分自身の型を指定することで、メソッドチェーンを使うこともできる
*/
struct Person3 {
    name: String,
    age: u32,
}

impl Person3 {
    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old", self.age);
        self
    }
}

/*
引数で&selfだと、構造体のメンバ変数を変更することができない。
変更可能にするには&mut selfにする
*/

impl Person3 {
    fn new(name: &str, age: u32) -> Person3 {
        Person3 {
            name: String::from(name),
            age: age,
        }
    }
}
