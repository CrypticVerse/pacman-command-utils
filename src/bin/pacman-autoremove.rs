use std::process::{Command, Stdio};

fn main() {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("Please run this function as sudo!");
        std::process::exit(1);
    }

    let output = Command::new("pacman").args(["-Qdtq"]).output().expect("failed to run pacman -Qdtq");
    let packages = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if packages.is_empty() {
        println!("nothing to do");
        return;
    }

    let status = Command::new("pacman").arg("-Rns").args(packages.split_whitespace())
        .stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()
        .expect("failed to run pacman -Rns!");

    if !status.success() {
        eprintln!("pacman exited with an error!");
    }
}
