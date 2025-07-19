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
fn main() {
    // outerモジュールのgreet関数を呼び出す
    outer::greet();
}
