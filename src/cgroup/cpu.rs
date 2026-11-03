use std::fs;
use std::io;
use std::path::Path;

const CPU_PERIOD_MICROSECONDS: u64 = 100_000;

pub struct CpuCgroup;

impl CpuCgroup {
    pub fn apply(path: &Path, cores: u64) -> io::Result<()> {
        if cores == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "la limite CPU doit être supérieure à zéro",
            ));
        }
        let quota = cores.checked_mul(CPU_PERIOD_MICROSECONDS).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "limite CPU trop élevée")
        })?;
        fs::write(path.join("cpu.max"), format!("{quota} {CPU_PERIOD_MICROSECONDS}"))
    }
}