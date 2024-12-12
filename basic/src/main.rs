mod myhash;
fn main() {
    // エントリーポイント
    let s1: String = String::from("Hello");
    let s2: &String = &s1; // 借用
    println!("s2: {}", s2);

    myhash::my_hash();
}