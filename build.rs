///
/// before project is build, execute this program to produce some file
pub fn main() -> std::io::Result<()> {
    println!("=====================prepare to build project=========================");
    std::process::Command::new("git").arg("rev-parse").arg("HEAD").status()?;
    Ok(())
}
