pub struct CompileError {
    pub message: String,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compilation Error: {}", self.message)
    }
}

impl std::fmt::Debug for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileError")
            .field("message", &self.message)
            .finish()
    }
}

impl CompileError {
    pub fn new(message: impl Into<String>) -> Self {
        CompileError {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, CompileError>;
