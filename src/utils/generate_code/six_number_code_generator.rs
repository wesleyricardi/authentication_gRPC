use rand::{distributions::Alphanumeric, Rng};

pub fn six_number_code_generator() -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}
