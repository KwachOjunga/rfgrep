use std::process::Command;

fn call_git_instance() -> Command {
    Command::new("git")
}

// obtain a list of branches in the current repo
fn branches() -> Result<Vec<String>, std::io::Error> {
    let mut git = call_git_instance();
    let output = git.arg("branch").arg("-a").output()?;
    let branches = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>();
    Ok(branches)
}

fn switch_branch(branch_name: &str) -> Result<(), std::io::Error> {
    let mut git = call_git_instance();
    git.arg("checkout").arg(branch_name).output()?;
    Ok(())
}

//[NOTE:] This can be done in parrallel.
// But serialize it first.
fn perform_branch_search() {
    // find a way to capture current branch instance.
    // ensure that that branch is the one you get back to.
    let branches = branches().unwrap();
    for branch in branches {
        println!("Searching branch: {}", branch);
    }
}
