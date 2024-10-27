use std::fs;

use crate::{utils, Wine};
use flate2::read::GzDecoder;
use tar::Archive;
use reqwest::Client;

const OWNER: &str = "doitsujin";
const REPO: &str = "dxvk";

const SYS32: &str = "drive_c/windows/system32";
const SYSWOW64: &str = "drive_c/windows/syswow64";

const DLL_LIST: [&str; 5] = [
    "d3d8",
    "d3d9",
    "d3d10core",
    "d3d11",
    "dxgi"
];

pub async fn get_latest_dxvk() -> Result<String, reqwest::Error> {
    utils::get_latest_release(&OWNER, &REPO).await
}

pub async fn install_dxvk(prefix: &Wine, version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "https://github.com/{}/{}/releases/download/{}/dxvk-{}.tar.gz",
        OWNER, REPO, version, &version[1..]
    );

    let response = client.get(&url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    let gz = GzDecoder::new(bytes.as_ref());
    let mut archive = Archive::new(gz);

    let x64_dir = &prefix.prefix_path.join(SYS32);
    let x32_dir = &prefix.prefix_path.join(SYSWOW64);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;

        if let Some(path_str) = path.to_str() {
            if path_str.to_lowercase().ends_with(".dll") {
                let target_dir = match path_str {
                    s if s.contains("x64") => x64_dir,
                    s if s.contains("x32") => x32_dir,
                    _ => continue,
                };

                let file_name = path.file_name().unwrap_or_default();
                let target_path = target_dir.join(file_name);
                entry.unpack(&target_path)?;
            }
        }
    }

    for dll in DLL_LIST {
        if let Err(e) = prefix.reg_add(r"HKEY_CURRENT_USER\Software\Wine\DllOverrides", &dll, "REG_SZ", "native") {
            return Err(format!("Failed to add registry key: {}", e).into());
        }
    }

    Ok(())
}

pub fn remove_dxvk(prefix: &Wine) -> Result<(), String> {
    let x64_dir = &prefix.prefix_path.join(SYS32);
    let x32_dir = &prefix.prefix_path.join(SYSWOW64);

    for dll in DLL_LIST {
        let dll_path = format!("{}.dll", dll);
        let x64_dir_path = x64_dir.join(&dll_path);
        let x32_dir_path = x32_dir.join(&dll_path);
        if x64_dir_path.exists() {
            if let Err(e) = fs::remove_file(&x64_dir_path) {
                return Err(format!("Failed to delete {}: {}", &x64_dir_path.display(), e.to_string()));
            }
        }
        if x32_dir_path.exists() {
            if let Err(e) = fs::remove_file(&x32_dir_path) {
                return Err(format!("Failed to delete {}: {}", &x32_dir_path.display(), e.to_string()));
            }
        }
        for dll in DLL_LIST {
            if let Err(e) = prefix.reg_delete(r"HKEY_CURRENT_USER\Software\Wine\DllOverrides", &dll) {
                return Err(format!("Failed to delete registry key: {}", e).into());
            }
        }
    }

    let output = &prefix.cmd()
        .arg("wineboot")
        .arg("-u")
        .output()
        .map_err(|e| format!("Failed to execute wine: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Wine prefix failed to update: {}", stderr));
    }

    Ok(())
}