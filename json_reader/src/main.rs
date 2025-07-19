use std::fs::File; // ファイル操作のための標準ライブラリ
use std::io::BufReader; // バッファリングされた読み込みのための標準ライブラリ
use std::path::PathBuf; // パス操作のための標準ライブラリ
use serde::Deserialize; // SerdeライブラリのDeserializeトレイトをインポート

// 1. JSONの構造に対応するRustの構造体を定義
// `Deserialize`をderiveすることで、JSONからこの構造体に変換できるようになる
#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. パスの生成
    // プロジェクトのルートからの相対パスでJSONファイルへのパスを構築
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); // CARGO_MANIFEST_DIRはCargo.tomlのあるディレクトリのこと
    path.push("data/user.json"); // もとのパスにdata/user.jsonを追加

    println!("Reading file from: {}", path.display());

    // 3. ファイルの読み込み
    // File::openでファイルを開き、BufReaderで効率的に読み込む
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // 4. JSONの解析
    // serde_json::from_readerで、リーダーからJSONを読み込み、User構造体に変換
    let user: User = serde_json::from_reader(reader)?;
    // 5. 結果の表示
    // `Debug`をderiveしているので、{:?}や{:#?}で中身を綺麗に表示できる
    println!("\nSuccessfully parsed JSON:");
    println!("{:#?}", user);

    // 個別のフィールドにもアクセス可能
    println!("\nHello, {}!", user.name);

    Ok(())
}
