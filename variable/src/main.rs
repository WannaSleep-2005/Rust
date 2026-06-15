fn main() {
    let s1 = String::from("hello");
    //let (s2, len) = cal_len(s1);
    let len = cal_len2(&s1);
    println!("Length of {s1} = {len}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    let mut r = String::from("abcd");
    let r1 = &r;
    let r2 = &r;
    println!("{0}, {1}", r1, r2);

    let r3 = &mut r;
    println!("{r3}");
}

/*fn cal_len(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}*/

fn cal_len2(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}