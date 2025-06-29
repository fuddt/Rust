---

marp: true

paginate: true

math: true

---

# <center>4_Rustを始める前にCargoについて使い方を学ぶ

---

# Cargoってなに？
CargoはRustのビルドシステム兼パッケージマネージャ

・ソースコードを基に実行可能ファイルを生成してくれる(ビルドシステム)
・ライブラリのダウンロード・ライブラリのビルドをしてくれる（パッケージマネージャ）

一言で表すと：開発をサポートしてくれるツール

※前回「1_Rustをインストールする」でも軽く使いました。

---

# CargoはRustの開発をサポートしてくれる

新規にRustでアプリ・ツールを作成したいとなった際に`cargo new <ツール名>`で開発に必要なものを一式用意してくれます。

例) tmpというディレクトリに何もない状態でcargo new hello_worldを打つ
```bash
tmp % ls
./      ../
```
```bash
tmp % cargo new hello_world
    Creating binary (application) `hello_world` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```
hello_worldディレクトリが生成される
```bash
tmp % ls
./              ../             hello_world/
```

---
# Cargoは便利

hello_worldに入って、ディレクトリの中身を見ると...

```bash
tmp % cd hello_world 
hello_world % ls
./              ../             .git/           .gitignore      Cargo.toml      src
```

gitの初期化処理もしてくれて.gitignoreも作ってくれてCargo.tomlも作ってくれて...
srcの中にはmain.rsファイルが生成されていて、
```rust
fn main() {
    println!("Hello, world!");
}
```
まで書いてくれている。

---


