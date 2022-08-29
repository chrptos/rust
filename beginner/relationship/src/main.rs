fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 pointer: {:?}", v1.as_ptr());
    println!("v1 len: {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());
    println!("v1 [0]: {:?}", &v1[0]);

    let s1: String = String::from("Hello");
    let s2: String = String::from("Rust");
    let s = concat(&s1, &s2);
    println!("{}", s);
    // 借用しているためs1, s2が使える
    println!("{}", s1);
    println!("{}", s2);
}

fn concat(a: &String, b: &String) -> String {
    let c: String = format!("{} {}", a, b);
    c
}