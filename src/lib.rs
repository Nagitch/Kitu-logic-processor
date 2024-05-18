extern crate rosc;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod actor;

use actix::prelude::*;
pub use actor::{MyActor, Ping};
use pest::Parser;

pub fn start_actor_system() -> Addr<MyActor> {
    let addr = MyActor.start();
    addr
}

// 既存のランタイムを使用して非同期コードを実行するためのヘルパー関数
pub async fn run_actor_system() -> String {
    let addr = start_actor_system();
    addr.send(Ping).await.unwrap()
}

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

use rosc::{decoder, OscPacket, Result};

// 共通の処理: すべてのプラットフォームで使用可能な関数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn ping() -> String {
    "pong".to_string()
}

pub fn osc_decode(osc_bytes: &[u8]) -> Result<(&[u8], OscPacket)> {
    decoder::decode_udp(&osc_bytes)
}

pub fn pest_parse() {
    let pairs = IdentParser::parse(Rule::ident_list, "a1 b2").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
}

// プラットフォーム固有のコード: 条件付きコンパイルを使用
#[cfg(target_os = "windows")]
pub fn hello() {
    println!("Hello from Windows!🦊");
}

#[cfg(target_os = "linux")]
pub fn hello() {
    println!("Hello from Linux!🦊");
}

#[cfg(target_os = "ios")]
pub fn hello() {
    println!("Hello from iOS!🦊");
}

#[cfg(target_arch = "wasm32")]
pub fn hello() {
    println!("Hello from WebAssembly!🦊");
}

// サポートされていないプラットフォーム用のデフォルト実装
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "ios", target_arch = "wasm32")))]
pub fn hello() {
    println!("Hello from an unsupported platform!🥲");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -1), -2);
    }
}