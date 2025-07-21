use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}
