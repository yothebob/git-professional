use regex::Regex;
use console::style;

pub struct Diff {
    pub files: Vec<Dfile>,
    pub wordlist: Vec<String>
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
    
    pub fn close_dfile(&mut self) {
	if &self.files.len() > &0 {
	    let last_index = self.files.len() - 1;
	    self.files[last_index].closed = true
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
	})
    }
    
    pub fn get_linestart(&mut self, _line: String) { // @@ -34,6 +34,8 @@ //line 34
	// I saw a bug with this not working right
	println!("{}", _line);
	let line_start = Regex::new(r"\+\d\d?\d?\d?\d?").unwrap();
	let m = line_start.find(&_line).unwrap();
	let last_index = self.files.len() - 1;
	self.files[last_index].line_start = m.
	    as_str()
	    .split_at(1).1.parse::<i32>().unwrap();
	    println!("{:?}", self.files[last_index].line_start);
	    println!("{:?}", self.files[last_index].filename)
    }

    pub fn load_bad_word_list(&mut self) {
    // load bad words from a file? else fallback
	self.wordlist = vec!["fuck".to_string(), "shit".to_string(), "dick".to_string()]
   }
    
    pub fn check_diff_line(&mut self, _line: String) {
	println!("{}", _line);
	let last_index = self.files.len() - 1;
	self.files[last_index].current_line += 1;
	let mut cursed = false;
	let current_line = self.files[last_index].current_line.to_owned();
	println!("{}", self.files[last_index].current_line.to_owned());
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
