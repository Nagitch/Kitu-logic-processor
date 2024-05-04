extern crate rosc;

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