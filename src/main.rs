mod diff;

use std::process::Command;
use diff::{ Diff };

fn main() {
    println!("Checking unstaged files...");
    let output = Command::new("git")
        .args(["--no-pager", "diff"])
        .output()
	.expect("failed to execute process");
    let cmd_output = String::from_utf8(output.stdout);
    match cmd_output {
	Ok(co) => {
	    let mut gitdiff: Diff = disbatch_output(co);
	    gitdiff.display_diff();
	},
	Err(_e) => { println!("an error occured") },
    }
}

// fuck!
fn disbatch_output(_output: String) -> Diff {
    let mut diff_file: Diff = Diff{files: Vec::new(),
				   wordlist: Vec::new(),
				   current_file: false
    };
    diff_file.read_curse_lib();
    println!("Wordlist: {:?}", diff_file.wordlist);

    for i in _output.lines() {
	match i {
	    x if x.starts_with("+++") => diff_file.add_dfile(x.to_string()),
	    x if x.starts_with("diff --git a") => diff_file.close_dfile(),
	    x if x.starts_with("@@") => diff_file.get_linestart(x.to_string()),
	    x if x.starts_with("+") => diff_file.check_diff_line(x.to_string()),
	    x if !x.starts_with("-")=> diff_file.disbatch_line(),
	    _ => ()
	}
    }
    diff_file
}


