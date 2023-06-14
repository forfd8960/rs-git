pub mod file;

pub trait Storage {
    fn create_file(&self, path: &str, content: String) -> Result<(), ()>;
}
