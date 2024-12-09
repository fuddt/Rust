fn main() {
    // エントリーポイント
    let s1 = String::from("Hello");
    let s2 = s1; // 所有権がs1からs2に移動
    println!("{}", s1); // コンパイルエラー: s1は無効

    let s3 = &s2; // 借用
    println!("s2: {}, s3: {}", s2, s3);
}

fn main() {
    // エントリーポイント
    let s1 = String::from("Hello");
    let s2 = &s1; // 借用
    println!("s2: {}", s2);
}