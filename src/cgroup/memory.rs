use std::fs;
use std::io;
use std::path::Path;

pub struct MemoryCgroup;

impl MemoryCgroup {
    pub fn apply(path: &Path, bytes: u64) -> io::Result<()> {
        if bytes == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "la limite mémoire doit être supérieure à zéro",
            ));
        }
        fs::write(path.join("memory.max"), bytes.to_string())
    }
}