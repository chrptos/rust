fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 3];
    println!("v1 pointer: {:?}", v1.as_ptr());
    println!("v1 len: {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());
    // スタック領域のポインタがv1の最初の要素のアドレスであることの確認
    println!("v1 [0]: {:?}", &v1[0]);

    // 要素数増加のために新たに領域確保したことを確認する
    v1.push(4);
    println!("v1 pointer: {:?}", v1.as_ptr());
    println!("v1 len: {:?}", v1.len());
    println!("v1 capacity: {:?}", v1.capacity());

    let v2 = v1;
    // 所有権が移動しているためv1を参照できないことを確認する
    // println!("v1 pointer: {:?}", v1.as_ptr());

    // ポインタのアドレスがv1とv2で一致しているから、
    // メモリ上で値がコピーされたのではなく所有権が移動したことが確認できる
    println!("v2 pointer: {:?}", v2.as_ptr());

    // v1を参照したい場合は。。。もちろん所有権の移動前に行う必要がある
    // let v2 = v1.clone();
}
