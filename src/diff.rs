use regex::Regex;
use console::style;
use std::fs;

pub struct Diff {
    pub files: Vec<Dfile>,
    pub wordlist: Vec<String>,
    pub current_file: bool
	
}

#[derive(Debug)]
pub struct Curse {
    pub line_number: i32,
    pub line_contents: String
}

#[derive(Debug)]
pub struct Dfile {
    pub filename: String,
    pub line_start: i32,
    pub current_line: i32,
    pub offenses: i32,
    pub curses: Vec<Curse>,
    pub closed: bool,
}

impl Diff {

    pub fn read_curse_lib(&mut self) {
	let contents = fs::read_to_string("words.txt").expect("Should be able to read file");
	for word in contents.split_whitespace() {
	    self.wordlist.push(word.to_owned())
	}
	if self.wordlist.len() <= 0 {
	    self.load_bad_word_list()
	}
    }
    
    pub fn disbatch_line(&mut self) {
	if self.current_file {
	    let last_index = self.files.len() - 1;
	    self.files[last_index].closed = true;
	    self.files[last_index].current_line += 1
	}
    }
    
    pub fn close_dfile(&mut self) {
	if &self.files.len() > &0 {
	    let last_index = self.files.len() - 1;
	    self.files[last_index].closed = true;
	    self.current_file = false
	}
    }
    
    pub fn add_dfile(&mut self, _filename: String) {
	self.close_dfile();
	self.files.push(Dfile{
	    filename: _filename.to_owned(),
	    line_start: 0,
	    current_line: 0,
	    curses: Vec::new(),
	    offenses: 0,
	    closed: false
	});
	self.current_file = true
    }
    
    pub fn get_linestart(&mut self, _line: String) { // @@ -34,6 +34,8 @@ //line 34
	// I saw a bug with this not working right
	let line_start = Regex::new(r"\+\d\d?\d?\d?\d?").unwrap();
	let m = line_start.find(&_line).unwrap();
	let last_index = self.files.len() - 1;
	self.files[last_index].line_start = m.
	    as_str()
	    .split_at(1).1.parse::<i32>().unwrap();
	self.files[last_index].current_line = m.
	    as_str()
	    .split_at(1).1.parse::<i32>().unwrap();
    }

    pub fn load_bad_word_list(&mut self) {
    // load bad words from a file? else fallback
	self.wordlist = vec!["fuck".to_string(), "shit".to_string(), "dick".to_string()]
   }
    
    pub fn check_diff_line(&mut self, _line: String) {
	let last_index = self.files.len() - 1;
	let mut cursed = false;
	let current_line = self.files[last_index].current_line.to_owned();
	for word in _line.split_whitespace() {
	    if self.wordlist.iter().any(|e| word.contains(e)){
		if !cursed {
		    cursed = true;
		    self.files[last_index].curses.push(Curse{
			line_number: current_line,
			line_contents: _line.to_owned()
		    });
		}
		self.files[last_index].offenses += 1
	    } 
	}
	self.files[last_index].current_line += 1;
    }

    pub fn display_diff(&mut self) {
	let mut total_offense = 0;
	for ff in &self.files {
	    if !ff.curses.is_empty() {
		total_offense += &ff.offenses;
		println!("{:?}", ff.filename);
		for cur  in &ff.curses {
		    println!("ln {}: {}", style(cur.line_number).red(), style(&cur.line_contents).red())
		}
	    }
	}
	match total_offense {
	    x if x > 10 => println!("total offenses: {}", style(total_offense).red()),
	    x if x > 5 => println!("total offenses: {}", style(total_offense).color256(6)),
	    x if x >= 1 => println!("total offenses: {}", style(total_offense).yellow()),
	    x if x <= 0 => println!("total offenses: {}", style(total_offense).green()),
	    _ => println!("total offenses: {}", total_offense)
	}
    }
}
