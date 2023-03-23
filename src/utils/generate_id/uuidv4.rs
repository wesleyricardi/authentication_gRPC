use uuid::Uuid;

pub fn new_uuidv4() -> String {
    Uuid::new_v4().to_string()
}
