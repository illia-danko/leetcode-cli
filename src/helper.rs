//! A set of helper traits
pub use self::file::code_path;
pub use self::digit::Digit;
pub use self::html::HTML;
pub use self::filter::filter;

/// Convert i32 to specific digits string.
mod digit {
    /// Abstract Digit trait, fill the empty space to specific length.
    pub trait Digit<T> {
        fn digit(self, d: T) -> String;
    }

    impl Digit<i32> for i32 {
        fn digit(self, d: i32) -> String {
            let mut s = self.to_string();
            let space = " ".repeat((d as usize) - s.len());
            s.push_str(&space);

            s
        }
    }

    impl Digit<i32> for String {
        fn digit(self, d: i32) -> String {
            let mut s = self.clone();
            let space = " ".repeat((d as usize) - self.len());
            s.push_str(&space);

            s
        }
    }

    impl Digit<i32> for &'static str {
        fn digit(self, d: i32) -> String {
            let mut s = self.to_string();
            let space = " ".repeat((d as usize) - self.len());
            s.push_str(&space);

            s
        }
    }
}

/// question filter tool
mod filter {
    use crate::cache::models::Problem;
    ///     -q, --query <query>          Fliter questions by conditions:
    ///                                  Uppercase means negative
    ///                                  e = easy     E = m+h
    ///                                  m = medium   M = e+h
    ///                                  h = hard     H = e+m
    ///                                  d = done     D = not done
    ///                                  l = locked   L = not locked
    ///                                  s = starred  S = not starred
    pub fn filter(ps: &mut Vec<Problem>, query: String) {
        for p in query.chars() {
            match p {
                'l' => ps.retain(|x| x.locked),
                'L' => ps.retain(|x| !x.locked),
                's' => ps.retain(|x| x.starred),
                'S' => ps.retain(|x| !x.starred),
                'e' => ps.retain(|x| x.level == 1),
                'E' => ps.retain(|x| x.level != 1),
                'm' => ps.retain(|x| x.level == 2),
                'M' => ps.retain(|x| x.level != 2),
                'h' => ps.retain(|x| x.level == 3),
                'H' => ps.retain(|x| x.level != 3),
                'd' => ps.retain(|x| x.status == "ac".to_string()),
                'D' => ps.retain(|x| x.status != "ac".to_string()),
                _ => {}
            }
        }
    }
}

/// Render html to command-line
mod html {
    // use crate::Error;
    use colored::Colorize;
    pub enum Token {
        Plain(String),
        Bold(String),
        Eof(String)
    }

    /// html render plugin
    pub trait HTML {
        fn ser(&self) -> Vec<Token>;
        fn render(&self) -> String;
    }

    impl HTML for String {
        fn ser(&self) -> Vec<Token> {
            // empty tags
            let mut tks = self.to_string();
            
            // converting symbols
            tks = tks.replace(r#"&lt;"#, "<");
            tks = tks.replace(r#"&gt;"#, ">");
            tks = tks.replace(r#"&amp;"#, "&");
            tks = tks.replace(r#"&quot;"#, "\"");
            tks = tks.replace(r#"&nbsp;"#, " ");

            let res: Vec<Token>;
            // styled
            {
                let mut ptr = 0;
                let mut output = vec![];
                let mut bold = false;
                for (i, e) in tks.chars().enumerate() {
                    match e {
                        '<' => {
                            match bold {
                                true => {
                                    output.push(Token::Bold(tks[ptr..i].to_string()));
                                    bold = false;
                                }
                                false => output.push(
                                    Token::Plain(tks[ptr..i].to_string())
                                ),
                            }
                            ptr = i;
                        },
                        '>' => {
                            match &tks[i-1..i] {
                                "-" => continue,
                                _ => match &tks[(ptr + 1)..i] {
                                    "b" | "strong" => bold = true,
                                    _ => {},
                                },
                            }
                            ptr = i + 1;
                        },
                        _ => {},
                    }
                };
                output.push(Token::Eof(tks[ptr..tks.len()].to_string()));
                res = output;
            }
            
            res
        }

        fn render(&self) -> String {
            let ts = self.ser();
            let mut tks: Vec<String> = vec![];
            
            for i in ts {
                match i {
                    Token::Plain(s) => tks.push(s.normal().to_string()),
                    Token::Bold(s) => {
                        if s.contains("Example") {
                            let mut br = "-".repeat(50).dimmed().to_string();
                            br.push_str("\n\n");
                            tks.push(br);
                        } else if s.contains("Note") {
                            let mut br = "* ".repeat(25).dimmed().to_string();
                            br.push_str("\n\n");
                            tks.push(br);
                        }
                        
                        tks.push(s.bold().to_string());
                    }
                    Token::Eof(s) => tks.push(s.normal().to_string()),
                }
            }

            tks.join("")
        }
    }
}

mod file {
    /// convert file suffix from language type
    pub fn suffix(l: &str) -> Result<&'static str, crate::Error> {
        match l {
            "bash" => Ok("sh"),
            "c" => Ok("c"),
            "cpp" => Ok("c"),
            "csharp" => Ok("c"),
            "golang" => Ok("go"),
            "java" => Ok("java"),
            "javascript" => Ok("js"),
            "kotlin" => Ok("kt"),
            "mysql" => Ok("sql"),
            "php" => Ok("php"),
            "python" => Ok("py"),
            "python3" => Ok("py"),
            "ruby" => Ok("rb"),
            "rust" => Ok("rs"),
            "scala" => Ok("scala"),
            "swift" => Ok("swift"),
            _ => Ok("c")
        }    
    }

    /// generate code path by fid
    use crate::cache::models::Problem;
    pub fn code_path(target: &Problem) -> Result<String, crate::Error> {
        let conf = crate::cfg::locate();
        
        let lang = conf.code.lang;
        let mut path = format!(
            "{}/{}.{}",
            conf.storage.code()?,
            conf.code.pick,
            suffix(&lang)?,
        );

        path = path.replace("${fid}", &target.fid.to_string());
        path = path.replace("${slug}", &target.slug.to_string());

        Ok(path)
    }
}
