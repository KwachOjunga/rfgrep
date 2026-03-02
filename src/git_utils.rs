use std::process::Command;

pub fn call_git_instance() -> Command {
    Command::new("git")
}

// obtain a list of branches in the current repo
pub fn branches() -> Result<Vec<String>, std::io::Error> {
    let mut git = call_git_instance();
    let output = git.arg("branch").arg("-a").output()?;
    let branches = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();
    Ok(branches)
}

pub fn switch_branch(branch_name: &str) -> Result<(), std::io::Error> {
    let mut git = call_git_instance();
    git.arg("checkout").arg(branch_name).output()?;
    Ok(())
}
