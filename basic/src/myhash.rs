use std::collections::HashMap;

pub fn my_hash() -> () {
    // 新しいHashMapを作成
    let mut scores = HashMap::new();

    // 値を追加
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 80);

    // 値を取得
    let alice_score = scores.get("Alice"); // Option<&V>を返す
    if let Some(score) = alice_score {
        println!("Alice's score: {}", score);
    } else {
        println!("Alice's score not found");
    }

    // HashMapを表示
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
