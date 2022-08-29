use std::rc::Rc;

fn main() {
    // 通常、スタックに格納されるデータはコンパイル時にサイズが確定している。
    // サイズを確定できない場合、Boxを使用すれば値をヒープに置くことができる。
    // つまり、サイズが未確定でもコンパイルを通すことができる。
    let x = Box::new(1);
    println!("x: {:p}", x);
    println!("*x + 2 = {}", *x + 2);

    // RC: reference count
    // 値の参照している数をカウントする機能。
    // RCを利用することで例外的に複数の所有者を存在させることができる。
    // RCはPythonなどのガベージコレクションを利用する言語で使用されている。

    // rcのstring型で定義
    let a = Rc::new("hello".to_string());
    // &aの所有者をカウントする => 1
    println!("count1: {}", Rc::strong_count(&a));
    {
        // 所有権をコピー
        let b = Rc::clone(&a);
        println!("a: {:p}", a);
        println!("b: {:p}", b);
        // 所有者が増えていることを確認 => 2
        println!("count2: {}", Rc::strong_count(&a));

    }
    // ブロックを抜けたのでaのみが所有権を持っている状態 => 1
    println!("count3: {}", Rc::strong_count(&a));
}
