#[cfg(test)]
use super::*;

#[test]
fn message_end_is_correct() {
    assert_eq!(get_message_end(), "blblbl");
}
