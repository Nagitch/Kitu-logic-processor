// imports the crate
// ライブラリ名はCargo.tomlの[lib]のnameに指定した名前
extern crate kitu_logic_processor;
use kitu_logic_processor::{add, multiply, hello};

fn main() {
    // 共通機能の使用
    let sum = add(5, 3);
    let product = multiply(4, 2);

    // 結果の表示
    println!("Sum of 5 and 3 is: {}", sum);
    println!("Product of 4 and 2 is: {}", product);

    // プラットフォーム固有の機能の使用
    hello();
}