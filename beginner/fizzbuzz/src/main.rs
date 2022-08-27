fn main() {
    // fizzbuzz(30);
    let mut v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 pointer: {:?}", v1.as_ptr());
    println!("v1 len: {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());
    println!("v1 [0]: {:?}", &v1[0]);
}

// fn fizzbuzz(end: i32) {
//     let mut x: i32 = 1;
//     while x <= end {
//         if x % 3 == 0 && x % 5 == 0 {
//             println!("FizzBuzz");
//         } else if x % 3 == 0 {
//             println!("Fizz");
//         } else if x % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}", x);
//         }
//         x += 1;
//     }
// }

// fn fizzbuzz(end: i32) {
//     let mut r: = 1..=end;
//     while x in r {
//         match x % 15 {
//             0 => println!("FizzBuzz"),
//             3 | 6 | 9 | 12 => println!("Fizz"),
//             5 | 10 => println!("Buzz"),
//             _ => println("{}", x),
//         }
//     }
// }

// fn fizzbuzz(end: i32) {
//     for x: i32 in 1..=end {
//         match (x % 3, x % 5) {
//             (0, 0) => println!("FizzBuzz"),
//             (0, _) => println!("Fizz"),
//             (_, 0) => println!("Buzz"),
//             _ => println!("{}", x),
//         }
//     }
// }
