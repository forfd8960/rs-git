pub trait Storage {
    fn create_file(path: &str, content: String) -> Result<(), ()>;
}
