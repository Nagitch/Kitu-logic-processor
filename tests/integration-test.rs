extern crate kitu_logic_processor;
extern crate rosc;
use rhai::{ Engine, EvalAltResult, Scope, AST };
use std::time::Instant;

use kitu_logic_processor::run_actor_system;
use rosc::{ OscType, OscPacket };

#[test]
fn test_add_integration() {
    assert_eq!(kitu_logic_processor::add(10, 20), 30);
}

#[test]
fn test_ping_integration() {
    assert_eq!(kitu_logic_processor::ping(), "pong".to_string());
}

// rosc integration test
#[test]
fn test_rosc_functional() {
    let osc_bytes: [u8; 16] = [
        47,
        116,
        101,
        115,
        116,
        0,
        0,
        0, // "/test" アドレス
        44,
        105,
        0,
        0, // 型タグ（,i は整数型）
        0,
        0,
        0,
        42, // 整数のデータ
    ];
    let result = kitu_logic_processor::osc_decode(&osc_bytes);
    let packet = result.unwrap().1;

    match packet {
        OscPacket::Message(msg) => {
            // OSC メッセージの場合の処理
            println!("OSC Address: {}", msg.addr);
            for arg in msg.args {
                match arg {
                    OscType::Int(i) => println!("Int: {}", i),
                    // 他の可能な OscType バリアントに対する処理もここに追加
                    _ => println!("Unexpected argument type"),
                }
            }
        }
        OscPacket::Bundle(bundle) => {
            // OSC バンドルの場合の処理
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

#[test]
fn test_rhai_functional() {
    let engine = Engine::new();
    let result = engine.eval::<i64>("40 + 2").unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_rhai_benchmark() -> Result<(), Box<EvalAltResult>> {
    fn fibonacci(n: i64) -> i64 {
        if n <= 1 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
    }

    let n: i64 = 5; // フィボナッチ数列のこの項を計算します

    // Rust関数の実行時間を測定
    let start = Instant::now();
    let rust_result = fibonacci(n);
    let rust_duration = start.elapsed();

    // Rhaiスクリプト（ASTなし）の実行時間を測定
    let mut engine = Engine::new();
    engine.set_max_call_levels(10000);
    let script =
        "
        fn fibonacci(n) {
            if n <= 1 { return n; }
            else { return fibonacci(n-1) + fibonacci(n-2); }
        }
        fibonacci(n)
    ";
    let mut scope = Scope::new();
    scope.push("n", n); // 変数 `n` をスコープに追加
    let start = Instant::now();
    let direct_result: i64 = engine.eval_with_scope(&mut scope, script)?;
    let direct_duration = start.elapsed();

    // Rhaiスクリプト（ASTあり）の実行時間を測定
    let ast = engine.compile_with_scope(&mut scope, script)?;
    let start = Instant::now();
    let ast_result: i64 = engine.eval_ast_with_scope(&mut scope, &ast)?;
    let ast_duration = start.elapsed();

    println!("Rust function result: {}, time: {:?}", rust_result, rust_duration);
    println!("Rhai direct eval result: {}, time: {:?}", direct_result, direct_duration);
    println!("Rhai AST eval result: {}, time: {:?}", ast_result, ast_duration);

    Ok(())
}

#[actix::test]
async fn test_my_actor() {
    let res = run_actor_system().await;
    assert_eq!(res, "Pong");
}
