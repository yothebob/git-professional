use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["--no-pager", "diff"])
        .spawn()
	.expect("failed to execute process");
    
	
    println!("{:?}", output)
}
