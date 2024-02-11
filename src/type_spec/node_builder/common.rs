use crate::compiler::CompilerEnv;

pub trait ImportAndUsingDetector {
    fn detect_import(&self, env: &CompilerEnv) -> Vec<String>;
    fn detect_using(&self, env: &CompilerEnv) -> Vec<String>;
}
