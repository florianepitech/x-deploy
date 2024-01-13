use rand::distributions::Alphanumeric;
use rand::Rng;

pub const API_KEY_VALUE_LENGTH: usize = 64;

pub fn new_key_value() -> String {
  rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(API_KEY_VALUE_LENGTH)
    .map(char::from)
    .collect::<String>()
    .to_uppercase()
}