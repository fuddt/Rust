use rand::random_range;
use serde_json::{Value, from_str};

fn main() {
    // ランダムなIDを生成
    let id = random_range(1..100);

    // JSON文字列（本来はファイルから読み込んだりする）
    let data = r#"
    {
        "name": "Alice",
        "age": 28
    }
    "#;

    // JSON文字列をValue型にパース
    let parsed: Value = from_str(data).unwrap();

    // 値の取得
    let name = parsed["name"].as_str().unwrap_or("unknown");

    println!("ID: {}, Name: {}", id, name);
}
