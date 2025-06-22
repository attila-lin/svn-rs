pub type CancelFunc = Box<dyn Fn() -> bool + Send + Sync>;
