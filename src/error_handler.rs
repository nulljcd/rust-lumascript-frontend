pub enum Error {
    SyntaxError(String),
}

pub struct ErrorHandler {
    error: Option<Error>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { error: None }
    }

    pub fn set_error(&mut self, error: Error) {
        self.error = Some(error);
    }

    pub fn print_error(&self) {
        if let Some(error) = &self.error {
            match error {
                Error::SyntaxError(message) => {
                    println!("SyntaxError:");
                    println!("    {message}");
                }
            }
        } else {
            println!("no error set");
        }
    }
}
