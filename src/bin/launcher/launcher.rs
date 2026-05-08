use std::{env,io,path::PathBuf,process::Command};

pub fn launch_vortex() -> io::Result<()> {
    #[cfg(windows)]
    {
        let exe_path = env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap();
        let game_path: PathBuf = exe_dir.join("vortex.exe");

        Command::new(&game_path).spawn()?;
        println!("vortex.exe launched");
    } Ok(())
}