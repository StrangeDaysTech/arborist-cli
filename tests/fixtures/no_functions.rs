const MAX_SIZE: usize = 100;
static GREETING: &str = "hello";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
