use crate::error::{FtlError, Result};
use std::fs;
use std::path::Path;

const DEFAULT_PID_FILE: &str = "/run/pihole-FTL.pid";

/// Trova il PID del processo FTL
pub fn find_ftl_pid() -> Result<u32> {
    find_ftl_pid_from_file(DEFAULT_PID_FILE)
}

/// Leggi PID da file specifico
pub fn find_ftl_pid_from_file(path: impl AsRef<Path>) -> Result<u32> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(FtlError::PidFileNotFound(path.to_path_buf()));
    }

    let content = fs::read_to_string(path).map_err(FtlError::PidFileReadError)?;

    let pid_str = content.trim();
    pid_str
        .parse::<u32>()
        .map_err(|_| FtlError::InvalidPid(pid_str.to_string()))
}

/// Verifica se il processo con questo PID esiste
pub fn is_process_running(pid: u32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_process_running() {
        // PID 1 (init/systemd) dovrebbe sempre esistere
        assert!(is_process_running(1));

        // PID molto alto probabilmente non esiste
        assert!(!is_process_running(99999));
    }
}
