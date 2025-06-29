---

marp: true

paginate: true

math: true

---

# <center>Rustの変数について

---

# Rustでは変数はデフォルトで不変です
Rustでは変数の宣言は`let`キーワードを使います。
Rustでは変数は不変(immutable)です。値が一旦変数に入れられると、その値を変えることができません。

```rust
fn main() {
    // Rustの変数はデフォルトで不変です。
    let x = 10;
    x = 20; // これはコンパイル時エラーになります。`x`は不変です。
    println!("The value of x is: {}", x);
}
```

cargo checkで確認してみます！

---

```rust
cargo check
    Checking guess_the_number v0.1.0 (/Users/path/to/your)
warning: value assigned to `x` is never read
 --> src/main.rs:3:9
  |
3 |     let x = 10;
  |         ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `x` <--ここで丁寧におしえてくれている
 --> src/main.rs:4:5
  |
3 |     let x = 10;
  |         - first assignment to `x`
4 |     x = 20; // これはコンパイル時エラーになります。`x`は不変です。
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut x = 10;
  |         +++

For more information about this error, try `rustc --explain E0384`.
warning: `guess_the_number` (bin "guess_the_number") generated 1 warning
error: could not compile `guess_the_number` (bin "guess_the_number") due to 1 previous error; 1 warning emitted
```

cannot assign twice to immutable variable `x` :不変変数'x'に2回代入できません。

---
# なぜ不変なのか？不便じゃないか？
Rustでは、値が不変であると宣言したら、本当に変わらないことをコンパイラが担保してくれます。 つまり、コードを読み書きする際に、どこでどうやって値が変化しているかを追いかける必要がなくなります。 故にコードを通して正しいことを確認するのが簡単になるのです。　--公式ドキュメントより

---
# 変数を可変にするには？
Rustでは変数を可変(mutable)にするには、`mut`キーワードを使います。

```rust
fn main() {
    // Rustの変数はデフォルトで不変です。
    let mut x = 10; // `mut`キーワードを使って可変にします。
    x = 20; // これでコンパイルエラーは発生しません。
    println!("The value of x is: {}", x);
}
``` 

---
cargo checkで確認してみます！

```rust
cargo check
warning: value assigned to `x` is never read
 --> src/main.rs:3:13
  |
3 |     let mut x = 10; // `mut`キーワードを使って可変にします。
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: `guess_the_number` (bin "guess_the_number") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

warningが出ていますが、checkは通りました。

---
# cargoの凄いところ　--先ほどの警告はなに？
警告で
```plaintext
help: maybe it is overwritten before being read?
```
とあります。
これは、変数`x`に値を代入した後、その値を読み取る前に再度代入しているから、最初の代入が無意味になっているけど、大丈夫？という警告です。
`x=10;`と入れているにも関わらず、その`10`を使う前に`x=20;`で上書きしているから最初の`10`は無意味になっていますよ、ということです。

とても賢いですね！！

---

# 清書

```rust
fn main() {
    // Rustの変数はデフォルトで不変です。
    let mut x = 10; // `mut`キーワードを使って可変にします。
    println!("The value of x is: {}", x);
    x = 20; // これでコンパイルエラーは発生しません。
    println!("The value of x is: {}", x);
}
```
これでcheckを通すと何も警告は出ません。
```
cargo check
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
```

---
# 変数と定数
Rustでは変数と定数は異なります。変数はデフォルトで不変ですが、定数は常に不変です。定数は`const`キーワードを使って宣言します。

```rust
fn main() {
    const MAX_POINTS: u32 = 100_000; // 定数は常に不変です。
    println!("The maximum points are: {}", MAX_POINTS);
}
```

そのためconstは`mut`キーワードを使うことはできません。定数はコンパイル時に決定され、実行時に変更することはできません。

---

`const`には型を明示的に指定する必要があります。
例えば、`const MAX_POINTS　= 100_000;`はエラーとなります。
```rust
fn main() {
    const MAX_POINTS = 100_000; // 型を明示的に指定します。
    println!("The maximum points are: {}", MAX_POINTS);
}
```
```rust
cargo check
    Checking guess_the_number v0.1.0 (/Users/path/to/your)
error: missing type for `const` item
 --> src/main.rs:3:12
  |
3 |     const x = 10; // `mut`キーワードを使って可変にします。
  |            ^ help: provide a type for the constant: `: i32`

error: could not compile `guess_the_number` (bin "guess_the_number") due to 1 previous error
```

要チェック!: **constにはmutキーワードは使えない。型の明示的な指定が必要。**

---

# シャドーイング
Rustでは、同じ名前の変数を再宣言することができます。これをシャドーイングと呼びます。シャドーイングを使うと、変数の型を変更したり、値を再計算したりすることができます。

```rust
fn main() {
    let x = 5; // 最初の変数x
    let x = x + 1; // シャドーイングで新しい変数xを宣言
    println!("The value of x is: {}", x); // 新しい変数xの値を表示
}
```
```bash
 cargo run  
   Compiling guess_the_number v0.1.0 (/Users/path/to/your)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/guess_the_number`
The value of x is: 6
```

---
# ブロックを使って変数のスコープを制御する
Rustでは、ブロックを使って変数のスコープを制御することができます。ブロック内で新しい変数を宣言すると、外側の変数と同じ名前でも問題ありません。
```rust
fn main() {
    let x = 5; // 最初の変数x
    {
        let x = x + 1; // ブロック内で新しい変数xを宣言
        println!("The value of x in the block is: {}", x); // ブロック内の変数xの値を表示
    }
    println!("The value of x outside the block is: {}", x); // ブロック外の変数xの値を表示
}
```
```bash
cargo run
   Compiling guess_the_number v0.1.0 (/Users/path/to/your)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/guess_the_number`
The value of x in the block is: 6
The value of x outside the block is: 5
```

---

# 少しややこしいところだけど安全な設計

2回目の `let x = x + 1;` は、外側の`x`を使って新しい`x`を定義している。
この新しい`x`はブロック内だけで使える「別の変数」なので、ブロックを抜けると元の`x`（値 = 5）に戻る。
Rustではこのように<strong>スコープごとに変数を再定義（シャドーイング）</strong>できるため、
変数を上書きすることなく安全に値を変えて扱える。

---

# シャドーイングの利点
- **型の変更**: シャドーイングを使うと、同じ名前の変数を**異なる型で再定義**できます。

mut の可変は**値のみ**が可変です。ここ重要です。
どういうことかというと、mutを使っても型を変えることはできません。

```rust
fn main() {
    let mut x = 5; // 最初の変数x
    x = "Hello, world!"; // 変数xに文字列を代入
    println!("x: {}", x); // 変数xの値を表示
}
```
cargo checkで確認

---

```bash
cargo check
    Checking guess_the_number v0.1.0 (/Users/path/to/your)
error[E0308]: mismatched types
 --> src/main.rs:3:9
  |
2 |     let mut x = 5; // 最初の変数x
  |                 - expected due to this value
3 |     x = "Hello, world!"; // 変数xに文字列を代入
  |         ^^^^^^^^^^^^^^^ expected integer, found `&str`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guess_the_number` (bin "guess_the_number") due to 1 previous error
```
mismatched types: expected integer, found `&str` :整数が期待されているのに、文字列が見つかりました。が出ています。
このようにmutを使っても型を変えることはできません。
そのため変数の型を変える場合はシャドーイングを使います。

---

以上