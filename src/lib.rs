// 共通の処理: すべてのプラットフォームで使用可能な関数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
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