use anyhow::Result;
use tokio::process::Command;

pub async fn is_repo() -> Result<bool> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()
        .await?
        .stdout;
    let output = String::from_utf8(output)?;

    Ok(output.trim() == "true")
}

pub async fn get_staged_diff() -> Result<String> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .await?
        .stdout;

    Ok(String::from_utf8(output)?)
}

pub async fn create_commit<S: AsRef<str>>(msg: S) -> Result<()> {
    Command::new("git")
        .arg("commit")
        .arg("-e")
        .arg("-m")
        .arg(msg.as_ref())
        .status()
        .await?;

    Ok(())
}
