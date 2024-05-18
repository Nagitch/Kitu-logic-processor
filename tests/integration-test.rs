extern crate kitu_logic_processor;
extern crate rosc;

use kitu_logic_processor::run_actor_system;
use rosc::{OscType, OscPacket};

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
        47, 116, 101, 115, 116, 0, 0, 0, // "/test" アドレス
        44, 105, 0, 0, // 型タグ（,i は整数型）
        0, 0, 0, 42, // 整数のデータ
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
        },
        OscPacket::Bundle(bundle) => {
            // OSC バンドルの場合の処理
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

#[actix::test]
async fn test_my_actor() {
    let res = run_actor_system().await;
    assert_eq!(res, "Pong");
}