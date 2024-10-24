use std::{fs, io, path::PathBuf, process::Command};

pub struct Wine {
    prefix_path: PathBuf,
}

impl Wine {
    pub fn new(prefix_path: &str) -> Self {
        Wine {
            prefix_path: PathBuf::from(prefix_path),
        }
    }

    pub fn init(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-i")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix initialization failed: {}", stderr));
        }

        Ok(())
    }

    pub fn update(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-u")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix killing failed: {}", stderr));
        }

        Ok(())
    }

    pub fn kill(&self) -> Result<(), String> {
        let output = &self.cmd()
            .arg("wineboot")
            .arg("-k")
            .output()
            .map_err(|e| format!("Failed to execute wine: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Wine prefix killing failed: {}", stderr));
        }

        Ok(())
    }

    pub fn delete(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.prefix_path)
    }

    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new("wine");
        cmd.env("WINEPREFIX", &self.prefix_path);
    
        cmd
    }
}