use super::Expiry;

pub enum SessionType {
    JWT(Expiry),
    Session(Expiry),
    None,
}
