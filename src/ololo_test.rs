#[cfg(test)]
use super::*;

#[test]
fn message_start_is_correct() {
    assert_eq!(get_message_start(), "ululu");
}
