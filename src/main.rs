fn main() {
    println!("Hello, world!");
    tuple();
    array();
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
