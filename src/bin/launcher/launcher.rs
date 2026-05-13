use std::{env, io, path::PathBuf, process::Command, fs};

pub fn launch_vortex() -> io::Result<()> {
    #[cfg(windows)]
    {
        let path = env::current_exe()?;
        let dir = path.parent().unwrap();
        let vortex: PathBuf = dir.join("vortex.exe");

        Command::new(&vortex).spawn()?;
        println!("vortex.exe launched");
    }
    Ok(())
}

pub fn open_mods_folder() -> std::io::Result<()> {
    #[cfg(windows)]
    {
        let path = env::current_exe()?;
        let dir = path.parent().unwrap();

        let mods: PathBuf = dir.join("mods");

        // if it doesnt exist it will create a folder (doesnt rpelace tho)
        fs::create_dir_all(&mods)?;

        Command::new("explorer").arg(&mods).spawn()?;

        Ok(())
    }
}
