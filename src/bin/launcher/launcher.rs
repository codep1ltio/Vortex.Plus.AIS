use std::{env, fs, io, path::PathBuf, process::{Child, Command}};
use std::path::Path;
use zip::ZipArchive;

pub fn launch_vortex() -> io::Result<Child> {
    #[cfg(windows)]
    {
        let path = env::current_exe()?;
        let dir = path.parent().unwrap();
        let vortex: PathBuf = dir.join("vortex.exe");

        let child = Command::new(&vortex).spawn()?;
        return Ok(child);
    }

    #[allow(unreachable_code)]
    Err(io::Error::other("unsupported platform"))
}

pub fn load_mods_folder() -> io::Result<()> {
    #[cfg(windows)]
    {
        let path = env::current_exe()?;
        let dir = path.parent().unwrap();

        let mods: PathBuf = dir.join("mods");
        fs::create_dir_all(&mods)?;
    }
    Ok(())
}

pub fn open_mods_folder() -> io::Result<()> {
    #[cfg(windows)]
    {
        let path = env::current_exe()?;
        let dir = path.parent().unwrap();

        let mods: PathBuf = dir.join("mods");
        fs::create_dir_all(&mods)?;
        Command::new("explorer").arg(&mods).spawn()?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(serde::Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn get_latest_release() -> io::Result<GithubRelease> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.github.com/repos/codep1ltio/Vortex.AIS/releases/latest")
        .header("User-Agent", "VortexAIS")
        .send()
        .await
        .map_err(io::Error::other)?;

    let release: GithubRelease = response
        .json()
        .await
        .map_err(io::Error::other)?;

    Ok(release)
}

pub async fn download_zip(url: &str) -> io::Result<Vec<u8>> {
    let client = reqwest::Client::new();

    let bytes = client
        .get(url)
        .header("User-Agent", "VortexAIS")
        .send()
        .await
        .map_err(io::Error::other)?
        .bytes()
        .await
        .map_err(io::Error::other)?;

    Ok(bytes.to_vec())
}

pub fn extract_zip(data: &[u8], out_dir: &Path) -> io::Result<()> {
    let reader = std::io::Cursor::new(data);
    let mut archive = ZipArchive::new(reader).map_err(io::Error::other)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(io::Error::other)?;
        let outpath = out_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub async fn install_update(zip_url: &str) -> io::Result<()> {
    let exe_path = env::current_exe()?;
    let base_dir = exe_path.parent().ok_or_else(|| io::Error::other("no dir"))?;

    let zip_data = download_zip(zip_url).await?;

    let tmp = base_dir.join("update_tmp");
    fs::create_dir_all(&tmp)?;

    extract_zip(&zip_data, &tmp)?;

    let new_exe = tmp
      .read_dir()?
      .filter_map(|e| e.ok())
      .map(|e| e.path())
      .find(|p| p.file_name().unwrap() == "vortex.exe")
      .ok_or_else(|| io::Error::other("vortex.exe not found"))?;

    let new_src = tmp.join("src");
    let old_src = base_dir.join("src");

    let _ = fs::remove_dir_all(&old_src);
    fs::rename(new_src, old_src)?;

    let final_path = base_dir.join("vortex.exe");
    fs::rename(&new_exe, &final_path)?;

    std::thread::sleep(std::time::Duration::from_millis(500));
    fs::remove_dir_all(&tmp)?;

    Ok(())
}