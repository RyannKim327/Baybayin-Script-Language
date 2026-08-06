pub struct Cli {
    pub file_path: Option<String>,
}

impl Cli {
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let file_path = args.get(1).cloned();
        Self { file_path }
    }
}
