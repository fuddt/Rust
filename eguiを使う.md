---

marp: true

paginate: true

math: true

---

# Eguiを使う

---

作りたいもの：
- 2次元の配列を受け取る
- その配列を元にキャンバスに描画する
- セルのサイズを変更できる
- 背景は黒
- セルの値が1のときは白で描画する
- 要は格子状の柄をキャンバスに描画する

どのように作っていくか？:
- 大元のアプリケーションのウィンドウとアプリ内に用意する部品は分けたい！
その方が好き！管理しやすいから！
- アプリケーションのウィンドウを用意するモジュール
- キャンバスを描画するモジュール
を用意する！

### キャンバス部分に関して
- どんなキャンバスにするかの構造体を定義する
コンストラクタで
  - 2次元配列を受け取る
  - セルのサイズを指定できるようにする ->あとでUIから変更できるようにする

実装すべき公開メソッド※今のところパッと思いつくもの:
- 基盤となるアプリのウィンドウを用意
- セルのサイズを設定する
- キャンバスを描画する

---





---

```rust
use eframe::egui; // EguiのUIコンポーネントを使用
use ndarray::Array2; // ndarrayライブラリを使用して2次元配列を扱う

pub struct CanvasRenderer {
    array: Array2<i32>,
    cell_size: f32,
}

//　コンストラクタ
impl CanvasRenderer {
    pub fn new(array: Array2<i32>) -> Self {
        Self {
            array,
            cell_size: 20.0, // デフォルトのセルサイズ
        }
    }

    // セルのサイズを設定するメソッド
    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size;
    }

    // キャンバスのサイズを取得するメソッド
    pub fn get_canvas_size(&self) -> egui::Vec2 {
        let (rows, cols) = self.array.dim();
        egui::Vec2::new(
            cols as f32 * self.cell_size,
            rows as f32 * self.cell_size,
        )
    }
}
```

---
memo:　
デフォルト値は : 20.0　"="じゃないよ
as f32は型変換　colsはusize型なのでf32に変換する必要がある
最初みた時, 「f32*self.cell_size」に見えた。
掛け算自体はcols * self.cell_sizeであって、colsをf32に変換して掛け算っていう意味。


---
早速テストしてみる

単体テストは同じファイルに書く

```rust

mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_set_cell_size() {
        let array = Array2::from_shape_vec((2, 2), vec![0, 1, 1, 0]).unwrap();
        let mut renderer = CanvasRenderer::new(array);
        
        // デフォルトのセルサイズは20.0
        assert_eq!(renderer.cell_size, 20.0);
        
        // セルサイズを変更
        renderer.set_cell_size(30.0);
        assert_eq!(renderer.cell_size, 30.0);
        
        // 別の値に変更
        renderer.set_cell_size(15.5);
        assert_eq!(renderer.cell_size, 15.5);
    }


    #[test]
    fn test_get_canvas_size() {
        // 2x3の配列でテスト
        let array = Array2::from_shape_vec((2, 3), vec![0, 1, 0, 1, 0, 1]).unwrap();
        let mut renderer = CanvasRenderer::new(array);
        
        // デフォルトのセルサイズ（20.0）での計算
        // 幅: 3列 * 20.0 = 60.0
        // 高さ: 2行 * 20.0 = 40.0
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 60.0);
        assert_eq!(size.y, 40.0);
        
        // セルサイズを変更してテスト
        renderer.set_cell_size(10.0);
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 30.0); // 3列 * 10.0
        assert_eq!(size.y, 20.0); // 2行 * 10.0
        
        // 小数点のセルサイズでテスト
        renderer.set_cell_size(15.5);
        let size = renderer.get_canvas_size();
        assert_eq!(size.x, 46.5); // 3列 * 15.5
        assert_eq!(size.y, 31.0); // 2行 * 15.5
    }
}

```
---
次キャンバスを描画してみる。

eframeのチュートリアルをそのままぶちこむ

```rust
use eframe::egui;



#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
    );
}
```

---
わけわからんので1個ずつ解読

```rust
struct MyEguiApp {}
```
アプリの情報の受け皿用の構造体を定義する

```rust
cc // CreationContextはアプリケーションの初期化に必要な情報を提供する
```

コンストラクタ等で、アプリケーションの初期設定的なものを渡せばいいのね。
で、Self::default()は、Defaultトレイトを実装しているので、デフォルト値を返す。空のMyEguiAppが返ってくる。
```rust
eframe::App for MyEguiApp {
```
Eguiのアプリケーションを実装するために、eframe::Appトレイトを実装する。
このトレイトには、アプリケーションの更新処理を行うupdateメソッドがある。

```rust
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
```
このメソッドは、アプリケーションの状態を更新し、UIを描画するために呼び出される。
- ctxはEguiのコンテキスト : コンテキスト＝つまりはアプリケーションの状態や設定を保持するもの
- frameはアプリケーションのフレーム : フレーム＝つまりは1つの描画サイクルを表すもの

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Hello World!");
});
```
Eguiの中央パネルを表示し、その中に"Hello World!"という見出しを描画する。
- CentralPanelはEguiの中央に配置されるパネル
- showメソッドは、パネルを描画するためのクロージャを受け取る。
- uiは描画用のUIコンテキストで、ここでUI要素を追加することができる。

```rust
let native_options = eframe::NativeOptions::default();
```
アプリケーションのネイティブオプションをデフォルト値で設定


```rust
let _ = eframe::run_native(
    "My egui App",
    native_options,
    Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
);
```
run_nativeの引数を確認すると
```rust
pub fn run_native(
    app_name: &str, 
    mut native_options: NativeOptions, 
    app_creator: AppCreator<'_>) -> Result
```

- run_native関数は、Eguiアプリケーションを実行するための関数
- app_nameはアプリケーションのタイトル 
- native_optionsはアプリケーションのオプション　つまりはデスクトップで動かすよーっていう設定
- app_creatorはAppCreatorという型で、アプリケーションのインスタンスを生成するためのクロージャを受け取る。
AppCreatorってどんな型？って思ったので調べたら実体としては以下のような型だった。

```
pub type AppCreator<'app> = Box<dyn FnOnce(&CreationContext<'_>) -> Result<Box<dyn App + 'app>, Box<dyn Error + Send + Sync>> + 'app>;
```
長い！って思ったけど落ち着いて先頭から見ていくと
Box\<xxxxxxxxx\>って書いてあるから、最後Boxになっていればいい
中身が、<dyn FnOnce(&CreationContext<'_>) -> Result<Box<dyn App + 'app>, Box<dyn Error + Send + Sync>> + 'app>となっている。

- dyn FnOnceは関数を表す型で、CreationContextを受け取り、ResultでAppのインスタンスを返す。
- Resultは成功時にBox<dyn App>を返し、失敗時にはBox<dyn Error + Send + Sync>を返す。
- 'appはライフタイムパラメータで、アプリケーションのインスタンスが生存する期間を示す。

1個ずつおさらいすると
最後はBoxにしよう。
そのBoxの中身は関数である必要があり、
その関数は以下の条件を満たす
- CreationContextを受け取る
- ResultでAppのインスタンスを返す
- 失敗した場合はErrorを返す
ような関数

長い！

ということでサンプルコードでは
```rust
Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc))))
```
となっている。

MyEguiはimplでeframe::Appトレイトを実装しているので、Box\<dyn App\>を返すことができる。

クロージャのせいで括弧"()"がいっぱいあるから焦る・・・
クロージャを別途関数定義してみた
```rust
fn create_app(cc: &eframe::CreationContext<'_>) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(Box::new(MyEguiApp::new(cc)))
}

eframe::run_native(
    "My egui App",
    native_options,
    Box::new(create_app),
);
```
関数のとこが引数の型すごく長くなるから、クロージャの方が良かったです

---

と、解読に時間がかかりましたが・・・
結局のところアプリの描画の内容を定義しているところは
```rust
impl eframe::App for MyEguiApp {
   fn update(&mut 以下略
```

というところだけですね。

updateはrun_nativeの中で呼ばれてて、ctxとframeを受け取っている。
どういう過程で渡されるのかは、eframeの内部で管理されているので、ここでは詳細は気にしなくて良くて、
考えるべきは、updateはctxとframeを受け取ったあとどう処理をさせるのかっていうところのロジック。
要するにここで自分で好きなように書けばいいということです。



---

試しに

```rust
use egui::widgets; 

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
           ui.add(widgets::Label::new("This is a simple egui app."));
              if ui.button("Click me!").clicked() {
               ui.label("Button clicked!");
           }
       });
   }
}

```
って感じで実行したら、ラベルとボタンが追加された。






---

じゃあ次はキャンバスを描画するメソッドを実装する

```rust
pub fn render(&self, ui: &mut egui::Ui) {
    let canvas_size = self.get_canvas_size();
    let (rows, cols) = self.array.dim();

    // キャンバスの領域を確保
    let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());

    // 背景を黒で塗りつぶし
    painter.rect_filled(
        response.rect,
        egui::CornerRadius::ZERO,
        egui::Color32::BLACK,
    );

    // 各セルを描画
    for row in 0..rows {
        for col in 0..cols {
            let value = self.array[[row, col]];
            
            if value == 1 {
                // 1の場合は白で描画
                let cell_rect = egui::Rect::from_min_size(
                    response.rect.min + egui::Vec2::new(
                        col as f32 * self.cell_size,
                        row as f32 * self.cell_size,
                    ),
                    egui::Vec2::splat(self.cell_size),
                );

                painter.rect_filled(
                    cell_rect,
                    egui::CornerRadius::ZERO,
                    egui::Color32::WHITE,
                );
            }
        }
    }

    // グリッド線を描画（オプション）
    self.draw_grid(&painter, &response.rect, rows, cols);
}
```

---
```rust
use eframe::egui;
use ndarray::Array2;

pub struct CanvasRenderer {
    array: Array2<i32>,
    cell_size: f32,
}

impl CanvasRenderer {
    pub fn new(array: Array2<i32>) -> Self {
        Self {
            array,
            cell_size: 20.0, // デフォルトのセルサイズ
        }
    }

    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size;
    }

    pub fn get_canvas_size(&self) -> egui::Vec2 {
        let (rows, cols) = self.array.dim();
        egui::Vec2::new(
            cols as f32 * self.cell_size,
            rows as f32 * self.cell_size,
        )
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        let canvas_size = self.get_canvas_size();
        let (rows, cols) = self.array.dim();

        let (response, painter) = ui.allocate_painter(canvas_size, egui::Sense::hover());

        // 背景を黒で塗りつぶし
        painter.rect_filled(
            response.rect,
            egui::CornerRadius::ZERO,
            egui::Color32::BLACK,
        );

        // 各セルを描画
        for row in 0..rows {
            for col in 0..cols {
                let value = self.array[[row, col]];
                
                if value == 1 {
                    // 1の場合は白で描画
                    let cell_rect = egui::Rect::from_min_size(
                        response.rect.min + egui::Vec2::new(
                            col as f32 * self.cell_size,
                            row as f32 * self.cell_size,
                        ),
                        egui::Vec2::splat(self.cell_size),
                    );

                    painter.rect_filled(
                        cell_rect,
                        egui::CornerRadius::ZERO,
                        egui::Color32::WHITE,
                    );
                }
            }
        }

        // グリッド線を描画（オプション）
        self.draw_grid(&painter, &response.rect, rows, cols);
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: &egui::Rect, rows: usize, cols: usize) {
        let grid_color = egui::Color32::from_gray(64);
        let stroke = egui::Stroke::new(1.0, grid_color);

        // 垂直線
        for col in 0..=cols {
            let x = rect.min.x + col as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                stroke,
            );
        }

        // 水平線
        for row in 0..=rows {
            let y = rect.min.y + row as f32 * self.cell_size;
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                stroke,
            );
        }
    }
}

```