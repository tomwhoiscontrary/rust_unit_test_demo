mod test;

mod test_included {
    include!("ololo_test.rs");
}

pub fn get_message() -> String {
    return get_message_start().to_owned() + " " + get_message_end();
}

fn get_message_start() -> &'static str {
    "ululu"
}

fn get_message_end() -> &'static str {
    "blblbl"
}
