---

marp: true

paginate: true

math: true

---

# <center>　パッケージ・クレート・モジュールについて

---
公式より
>Rustには、どの詳細を公開するか、どの詳細を非公開にするか、どの名前がプログラムのそれぞれのスコープにあるか、といったコードのまとまりを保つためのたくさんの機能があります。 これらの機能は、まとめて「モジュールシステム」と呼ばれることがあり、以下のようなものが含まれます。
- パッケージ: クレートをビルドし、テストし、共有することができるCargoの機能
- クレート: ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
- モジュール と use: これを使うことで、パスの構成、スコープ、公開するか否かを決定できます
- パス: 要素（例えば構造体や関数やモジュール）に名前をつける方法
---
# パッケージとクレート

## パッケージとは
パッケージは、1つ以上のクレートを含むプロジェクト全体のことです。`Cargo.toml`ファイルによって定義されている。

```toml
[package]
name = "sample"
version = "0.1.0"
edition = "2024"
```
そういえばありましたね。

---

## クレートとは
クレートは、Rustコンパイラがコンパイルの単位として扱うコードのまとまりです。
- **バイナリクレート**：実行可能ファイル（`main.rs`から作られる）
- **ライブラリクレート**：ライブラリ（`lib.rs`から作られる）

---

ディレクトリ構造は以下のようになります：

```
my_project/           ← パッケージ
├── Cargo.toml        ← パッケージの設定ファイル
├── src/
│   ├── main.rs       ← バイナリクレートのルート
│   └── lib.rs        ← ライブラリクレートのルート
└── target/           ← ビルド成果物
```

この例では：
- **パッケージ**：`my_project`ディレクトリ全体
- **クレート**：`main.rs`から作られるバイナリクレートと、`lib.rs`から作られるライブラリクレート

---
# モジュールとuse
モジュールは、Rustのコードを整理するための仕組みです。モジュールを使うことで、コードを論理的にグループ化できる。

モジュールは、`mod`キーワードを使って定義します。

```rust
mod my_module {
    pub fn my_function() {
        println!("Hello from my_module!");
    }
}
```
モジュール内の関数や構造体は、`pub`キーワードを使って公開することができます。公開された要素は、他のモジュールからアクセス可能になります。

---
# どんな時に使うの?
例えば関数をすごーくたくさん定義したけど、部分部分でグルーピングしたいときに使います。
以下のように、算数や文字列操作などの関数がバラバラで定義されていると見づらい！
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}
```

---
# モジュールを使って整理する
モジュールを使うと、関数をグループ化して整理できます。
```rust
//　算数モジュール
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
// 文字列モジュール
mod string_utils {
    pub fn concatenate(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }

    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }
}



```

---
# 名前の衝突も避けられる

モジュールを使うと、同じ名前の関数が異なるモジュールに存在しても衝突を避けられます。

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
mod string_utils {
    pub fn add(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }
}
```
名前を何個も考える心配がなくなる！

---

# pubとは？
`pub`は、モジュール内の要素を公開するためのキーワードです。公開された要素は、他のモジュールからアクセス可能になります。

```rust
mod my_module {
    pub fn my_function() {
        println!("Hello from my_module!");
    }
    fn private_function() {
        println!("This is a private function.");
    }
}

fn main() {
    my_module::my_function(); // 公開された関数は呼び出せる
    my_module::private_function(); // エラー
}

12 |     my_module::private_function(); // エラー: private function cannot be accessed
   |                ^^^^^^^^^^^^^^^^ private function



```
---
# １つのファイルだけじゃ管理しきれないくらいにデカくなってきたら・・・

modで関数をグループ化できるのはわかったけど、mathとstring_utilsがもっと大きくなったり、modがもっと増えて30個くらいなってきて、１つのファイルに1000行とかになったら、うんざりしてしまう。

その場合、モジュールを別のファイルに分割することができます。

---

# モジュールを別ファイルに分割する
先ほどのmathとstring_utilsを別のファイルに分割してみましょう。

### ディレクトリ構成
```bash
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── math.rs
│   └── string_utils.rs
└── target/
```

---
## math.rs
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

※ファイル分割した場合は`mod`は不要です。

---

### string_utils.rs
```rust
pub fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}
pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}
```


---
### main.rs
```rust
mod math; // math.rsをモジュールとして読み込む
mod string_utils; // string_utils.rsをモジュールとして読み込む
fn main() {
    let sum = math::add(5, 3);
    let difference = math::subtract(10, 4);
    let concatenated = string_utils::concatenate("Hello, ", "world!");
    let uppercased = string_utils::to_uppercase("hello");

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Concatenated: {}", concatenated);
    println!("Uppercased: {}", uppercased);
}
```

---

# モジュールアクセスの仕方self, super, crate
以下のような階層構造が複雑な場合のアクセスの仕方

```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Hello from inner_function!");
        }
    }

    pub fn greet() {
        println!("Hello from outer!");
    }
}
```
---

# アクセス方法としてself, super, crate
- `self`：現在のモジュールを指します。
- `super`：親モジュールを指します。
- `crate`：ルートモジュール（パッケージ全体）を指します。


---

# selfを使ったアクセス
```rust
例えばfn greet()の中で、inner_functionを呼び出したい場合は以下のように書きます。
```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Hello from inner_function!");
        }
    }

    pub fn greet() {
        // selfを使ってinner_functionにアクセス
        self::inner::inner_function();
    }
}
```

selfは現在のモジュールを指すので、`self::inner::inner_function()`と書くことで、`inner_function`にアクセスできます。

---

# superを使ったアクセス
例えば`mod inner`の中にもう一個関数を作って、`greet`関数から呼び出したい場合は以下のように書きます。
```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Hello from inner_function!");
        }
        pub fn another_function() {
            println!("Hello from another_function!");
            super::greet(); // superは1個上の親(outer)から始まるので、super::greet関数となる
        }
    }
    pub fn greet() {
        println!("Hello from outer!");
    }
}
```

---
# crateを使ったアクセス
```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Hello from inner_function!");
        }
    }
    pub fn greet() {
        // crateを使ってinner_functionにアクセス
        crate::outer::inner::inner_function();
    }
}
```
crateはパッケージ全体を指すので、`crate::outer::inner::inner_function()`と書くことで、`inner_function`にアクセスできます。一番、間違いがなさそう。長くなっちゃうけど・・・


---

# ファイル分割した際のサブモジュール
以下のモジュールを`outer.rs`と`inner.rs`に分割した場合のファイル構成について
```rust
mod outer {
    pub mod inner {
        pub fn inner_function() {
            println!("Hello from inner_function!");
        }
    }
    pub fn greet() {
        // crateを使ってinner_functionにアクセス
        crate::outer::inner::inner_function();
    }
}
```

---
# ディレクトリ構成
outerディレクトリを作成して、その中に`inner.rs`を配置します。
```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── outer.rs
│   └── outer/
│       └── inner.rs
└── target/

```
---
# outer.rs
```rust
mod inner; // inner.rsをサブモジュールとして読み込む
pub fn greet() {
    crate::outer::inner::inner_function();
}
```
# inner.rs
```rust
pub fn inner_function() {
    println!("Hello from inner_function!");
}
```

---
```bash
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── outer.rs
│   └── outer/　<--親モジュールと同じ名前のディレクトリを作る
│       └── inner.rs
└── target/
```
※以前は以下のような構成でした。
```bash
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── outer/
│       ├── mod.rs <-- outer.rsの代わり
│       └── inner.rs
└── target/