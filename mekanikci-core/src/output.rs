use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_output_dir(base: &Path) -> anyhow::Result<PathBuf> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let dir = base.join(format!("conveyor_{}", timestamp));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_output_dir_creates_directory() {
        let tmp = std::env::temp_dir().join("mekhanikci-test-output");
        let dir = create_output_dir(&tmp).unwrap();
        assert!(dir.exists());
        assert!(dir.to_string_lossy().contains("conveyor_"));
        std::fs::remove_dir_all(&tmp).unwrap();
    }
}
