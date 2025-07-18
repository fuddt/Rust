---

marp: true

paginate: true

math: true

---

# <center>enumについて


---

# enum列挙型とは？

enum列挙型は、関連する値の集合を定義するための型です。Rustでは、`enum`キーワードを使って列挙型を定義します。

例えばスーパーで夜になると、お惣菜が安くなることを表す列挙型を定義する場合、

```rust
enum Discount {
    None, // 割引なし
    ThirtyPercent, // 30%割引
    HalfPrice, // 半額
}
```

このように、割引の種類を列挙型で定義することで、コードの可読性が向上します。
このenumを実際に使ってみましょう。

---
スーパーの割引商品 structを定義して商品を定義します
```rust
struct Item {
    name: String,
    price: f64,
    discount: Discount, // 列挙型を使用
}

fn calc_price(item: &Item) -> f64 {
    match item.discount { // 列挙型のパターンマッチング
        // 各割引に応じて価格を計算
        Discount::None => item.price,
        Discount::ThirtyPercent => item.price * 0.7, // 30%割引
        Discount::HalfPrice => item.price * 0.5, // 半額
    }
}

fn main() {
    let item1 = Item {
        name: String::from("唐揚げ"),
        price: 500.0,
        discount: Discount::ThirtyPercent, // 30%割引
    };

    let price = calc_price(&item1);
    println!("{}の合計金額は{}円です", item1.name, price);
}
```

---
# 何が便利だったのかを解説

列挙型を使用することで、割引の種類を明確に定義でき、コードの可読性が向上します。また、列挙型で定義した**状態**を使って処理が分岐する関数を用意することで、**状態に応じた処理**を簡潔に記述できます。
<br>
ポイントとなるのは「**状態**」
<br>
状態に応じて処理を分岐したいときにとっても便利です。

---

# enumにデータを持たせる
列挙型は、単なる値の集合だけでなく、各値にデータを持たせることもできます。これにより、より複雑なデータ構造を表現できます。
例えば、先ほどは割引の種類だけを定義しましたが、どうせなら、
「割引の種類+その割引率」が揃っていた方が何かと便利なはず！それができます。
※Pythonでは定数を定義するようなものだったのが、Rustではenumを使うことで、定数に加えてデータも持たせることができます。

さっきは30%や半額と割引率の数だけ列挙型を定義しましたが、これを改良して、割引率をその場で定義できるようにしてみます。

```rust
enum Discount {
    None, // 割引なし
    Rate(f64), // 割引率
}
```

---

```rust
enum Discount {
    None, // 割引なし
    Rate(f64), // 割引率
}
struct Item {
    name: String,
    price: f64,
    discount: Discount, // 列挙型を使用
}

fn calc_price(item: &Item) -> f64 {
    match item.discount { // 列挙型のパターンマッチング
        Discount::None => item.price,
        Discount::Rate(rate) => item.price * (1.0 - rate), // 割引率を使用
    }
}

fn main() {
    
    let item_1 = Item {
        name: String::from("唐揚げ"),
        price: 500.0,
        discount: Discount::Rate(0.3), // 30%割引
    };

    let item_2 = Item {
        name: String::from("ポテト"),
        price: 300.0,
        discount: Discount::None, // 割引なし
    };

    let price = calc_price(&item_1);
    println!("{}の合計金額は{}円です", item_1.name, price);

    let price = calc_price(&item_2);
    println!("{}の合計金額は{}円です", item_2.name, price);
}
```

---

# enumに関数も持たせられる

先ほどは関数を別途定義していましたが、列挙型に関連する処理をまとめて定義することもできます。これにより、列挙型の値に対する操作を一元管理できます。

```rust
impl Discount {
    fn apply(&self, price: f64) -> f64 {
        match self {
            Discount::None => price,
            Discount::Rate(rate) => price * (1.0 - rate),
        }
    }
}
```

---
```rust
enum Discount {
    None, // 割引なし
    Rate(f64), // 割引率
}
impl Discount {
    fn apply(&self, price: f64) -> f64 {
        match self {
            Discount::None => price,
            Discount::Rate(rate) => price * (1.0 - rate),
        }
    }
}
struct Item {
    name: String,
    price: f64,
    discount: Discount, // 列挙型を使用
}
fn main() {
    let item_1 = Item {
        name: String::from("唐揚げ"),
        price: 500.0,
        discount: Discount::Rate(0.3), // 30%割引
    };

    let price_1 = item_1.discount.apply(item_1.price);
    println!("{}の合計金額は{}円です", item_1.name, price_1);
}
```

---
# enumの注意点
enumってmatch構文を使用するときは必ず全てのパターンを網羅する必要があります。
```rust
enum Discount {
    None, // 割引なし
    TentyPercent(f64), // 10%割引
    ThirtyPercent(f64), // 30%割引
    HalfPrice(f64), // 半額
}
```
上記のように列挙型を定義した場合は`match`構文で全てのパターンを網羅する必要があります。

---

```rust
fn calc_price(item: &Item) -> f64 {
    match item.discount { // 列挙型のパターンマッチング
        Discount::None => item.price,
        Discount::TentyPercent(rate) => item.price * (1.0 - rate), // 10%割引
        Discount::ThirtyPercent(rate) => item.price * (1.0 - rate), // 30%割引
        Discount::HalfPrice(rate) => item.price * (1.0 - rate), // 半額
    }
}
```
もしすべてを網羅しないと、コンパイルエラーになります。
```rust
error[E0004]: non-exhaustive patterns: `TentyPercent(_)` and ``HalfPrice(_)` not covered
 --> src/main.rs:10:9
```
このエラーは、列挙型のすべてのパターンを`match`で網羅していないことを示しています。超親切！これで漏れがなくなりますね！めんどくさいと思うかもしれませんが、漏れがないことを保証してくれるので、むしろありがたいです。

---
## じゃあ１０００とかパターンあったらどうするの？
列挙型のパターンが多い場合、すべてを網羅するのは大変ですが、Rustでは`_`（アンダースコア）を使って、他のすべてのパターンをキャッチすることができます。

```rust
enum Day {
    January_1,
    January_2,
    January_3,
    // ... 他の月や日も定義
}
fn print_day(day: Day) {
    match day {
        Day::January_1 => println!("今日は1月1日です"),
        Day::January_2 => println!("今日は1月2日です"),
        Day::January_3 => println!("今日は1月3日です"),
        // ... 他の月や日も処理
        _ => println!("他の日です"), // 他のすべてのパターンをキャッチ
    }
}
```

---

# if let構文
列挙型の値を簡単に扱うために、`if let`構文を使用することもできます。これにより、特定のパターンにマッチした場合のみ処理を実行できます。

```rust
if let Day::January_1 = day {
    println!("今日は1月1日です！明けましておめでとうございます！");
} else {
    println!("何でもない日バンザイ！");
}
```

この構文は、特定のパターンにマッチした場合にのみ処理を実行するため、コードが簡潔になります。

---

# Rustでよく出るSome・None（Option型）

Rustでよく見る`Some`は、標準ライブラリの`Option`型の一部です。`Option`は「値があるかもしれないし、ないかもしれない」を表現するための重要なEnum型です。

```rust
enum Option<T> {
    Some(T), // 値がある場合 
    None,    // 値がない場合
}
```

Optionは標準で定義されているので、自分で定義する必要はありません。Rustの標準ライブラリに組み込まれています。
※Tについてはまた別の機会に！とりあえず何の型でも入ると思ってください。

---

例えば、配列から特定の要素を取得する場合、存在しない場合もあります：

```rust
fn find_item(items: &[String], target: &str) -> Option<String> {
    for item in items {
        if item == target {
            return Some(item.clone()); // 見つかった場合
        }
    }
    None // 見つからなかった場合
}

fn main() {
    let items = vec![
        String::from("りんご"),
        String::from("バナナ"),
        String::from("みかん"),
    ];

    // 存在する場合はその名前を表示
    match find_item(&items, "りんご") {
        Some(fruit) => println!("見つかりました: {}", fruit),
        None => println!("見つかりませんでした"),
    }
}
```
---

# Option型の便利なメソッド

`Option`型には、値を安全に扱うための便利なメソッドがたくさんあります：

## unwrap_or: 値がない場合にデフォルト値を使用

```rust
fn main() {
    let maybe_number = Some(42);
    let nothing: Option<i32> = None;

    // unwrap_or: 値がない場合にデフォルト値を使用
    println!("値: {}", maybe_number.unwrap_or(0)); // 42
    println!("値: {}", nothing.unwrap_or(0));      // 0
}
```

---
# is_some / is_none: 値があるかどうかを確認

```rust
fn main() {
    let maybe_number = Some(42);
    // is_some / is_none: 値があるかどうかを確認
    if maybe_number.is_some() {
        println!("値があります");
    }

    let nothing: Option<i32> = None;
    if nothing.is_none() {
        println!("値がありません");
    }
}
```
---
# 値がある場合のみ処理を実行

```rust
fn main() {
    let maybe_number: Option<i32> = Some(42);
    let nothing: Option<i32> = None;
    // map: 値がある場合のみ処理を実行
    let doubled = maybe_number.map(|x| x * 2);
    println!("2倍した値: {:?}", doubled); // Some(84)

    let doubled_nothing = nothing.map(|x| x * 2);
    println!("2倍した値: {:?}", doubled_nothing); // None
}
```

---

## なぜOption型が重要なのか？


```rust
// 他の言語だと、こんなエラーが実行時に発生することがある
// let name = get_user_name(); // もしnullが返ってきたら...
// println!("{}", name.length()); // 💥 NullPointerException!

// Rustでは、コンパイル時に値がない可能性をチェックできる
fn get_user_name() -> Option<String> {
    // 何らかの処理...
    Some(String::from("田中太郎"))
}

fn main() {
    let name = get_user_name();
    
    // match文で安全に処理
    match name {
        Some(n) => println!("ユーザー名: {}", n),
        None => println!("ユーザー名が見つかりません"),
    }
}
```
---

# Result型（Ok・Err）

`Result`は、処理が成功するか失敗するかを表現するための重要で、これもEnum型です。エラーハンドリングの基本となります。

```rust
enum Result<T, E> {
    Ok(T),  // 成功した場合の値
    Err(E), // 失敗した場合のエラー
}
```
---
例えば、文字列を数値に変換する処理は失敗する可能性があります：

```rust
fn parse_number(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("「{}」は数値ではありません", input)),
    }
}

fn main() {
    // 成功する場合
    match parse_number("42") {
        Ok(num) => println!("数値: {}", num),
        Err(error) => println!("エラー: {}", error),
    }

    // 失敗する場合
    match parse_number("abc") {
        Ok(num) => println!("数値: {}", num),
        Err(error) => println!("エラー: {}", error),
    }
}
```

---
というわけで、EnumはRustの強力な機能で、状態を表現したり、関連するデータや処理をまとめたりするのに役立ち、いたるところで使われています。特に、Option型やResult型は、エラーハンドリングや値の存在を明示的に扱うために非常に重要です。


---
# ガチャポン作った

---
```rust
use rand::Rng;

enum GachaResult {
    Hit(u32),  // 当たり金額
    Miss,      // ハズレ（メッセージは処理側で決める）
}

fn gacha() -> GachaResult {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..10);

    if roll == 0 || roll == 1 || roll == 2 {
        GachaResult::Hit(500)
    } else {
        GachaResult::Miss
    }
}

fn open_gacha(result: GachaResult) {
    match result {
        GachaResult::Hit(amount) => {
            println!("当たり！{}円を獲得！", amount);
        }
        GachaResult::Miss => {
            println!("ハズレ...また回してね！");
        }
    }
}

fn main() {
    for i in 1..=5 {
        println!("--- {}回目のガチャ ---", i);
        let result = gacha();
        open_gacha(result);
    }
}
```