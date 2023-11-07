use super::Expiry;

/// This config is used to set the desired session type.
/// Supports json web tokens or classic table sessions. Can also be disabled with None.
pub enum SessionType {
    JWT(Expiry),
    Session(Expiry),
    None,
}
