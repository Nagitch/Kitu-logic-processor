// imports the crate
// ライブラリ名はCargo.tomlの[lib]のnameに指定した名前
extern crate kitu_logic_processor;
use kitu_logic_processor::{add, hello};

fn main() {
    // 共通機能の使用
    let sum = add(5, 3);

    // 結果の表示
    println!("Sum of 5 and 3 is: {}", sum);

    // プラットフォーム固有の機能の使用
    hello();
}