use std::io;
use std::process::Command;

pub fn launch_app(command: &[String]) -> std::io::Result<()> {
    let (program, args) = command
        .split_first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Empty command"))?;

    Command::new(program).args(args).spawn()?;

    Ok(())
}
