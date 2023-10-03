pub trait Authenticate {
    fn register(columns: Vec<(&str, &str)>) -> bool;
}
