use anyhow::{bail, Result};

pub fn get_path(path: &str) -> Result<String> {
    if path.starts_with('~') {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_default());
        if home.is_empty() {
            bail!("Cannot find user home directory, please specify absolute path")
        }
        Ok(format!("{}{}", home, path.trim_start_matches('~')))
    } else {
        Ok(path.to_string())
    }
}
