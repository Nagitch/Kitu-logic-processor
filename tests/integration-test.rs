extern crate kitu_logic_processor;

#[test]
fn test_add_integration() {
    assert_eq!(kitu_logic_processor::add(10, 20), 30);
}

#[test]
fn test_ping_integration() {
    assert_eq!(kitu_logic_processor::ping(), "pong".to_string());
}