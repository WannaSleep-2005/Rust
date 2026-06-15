fn main() {
    let s1 = call1();
    let s2 = String::from("hello");
    let s3 = call2(s2);

    println!("{s1}, {s3}");
}

fn call1() -> String {
    let s = String::from("world");
    s
}

fn call2(s: String) -> String {
    s
}