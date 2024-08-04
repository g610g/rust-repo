use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, Read},
};

#[derive(Debug)]
enum Args {
    ReadBytes,
    ReadLines,
    ReadChars,
    Input(String),
    ReadWords,
}
struct WC {
    args: Vec<Args>,
    buffer: String,
    file_name: String,
}
impl WC {
    fn initialize() -> Result<Self, Box<dyn Error>> {
        let mut args_iter = env::args();
        if args_iter.len() < 2 {
            return Err("Invalid number of arguments".into());
        }
        args_iter.next().unwrap_or_else(|| {
            panic!("Something went wrong");
        });
        match args_iter.next() {
            Some(_) => {}
            None => {
                return Err("Something went wrong".into());
            }
        }

        let mut file_name = String::new();
        let args = args_iter
            .map(|e| match e.as_str() {
                "-c" => Args::ReadBytes,
                "-l" => Args::ReadLines,
                "-w" => Args::ReadWords,
                "-m" => Args::ReadChars,
                _ => {
                    //append the str to the file_name variable
                    file_name.insert_str(0, e.as_str());
                    Args::Input(e)
                }
            })
            .collect();
        let (mut buffer, bytes_read) = read_from_stdin();
        if bytes_read != 0 {
            return Ok(WC {
                args,
                file_name,
                buffer,
            });
        }
        let mut file_handler = File::open(&file_name)?;

        let _ = file_handler.read_to_string(&mut buffer);
        Ok(WC {
            args,
            file_name,
            buffer,
        })
    }
    fn execute(&self) {
        //
        if self.args.len() == 1 {}

        let _ = self.args.iter().for_each(|item| {
            match item {
                Args::ReadBytes => self.count_bytes(),
                Args::ReadLines => self.count_lines(),
                Args::ReadWords => self.count_words(),
                Args::ReadChars => self.count_chars(),
                _ => self.count_lines().count_words().count_bytes(),
            };
        });
    }
    fn count_bytes(&self) -> &Self {
        //return an error since the buffer must be filled before executing this function
        if self.buffer.len() == 0 {}
        let bytes_count = self.buffer.bytes().len();
        println!("{} {}", bytes_count, self.file_name);
        self
    }
    fn count_lines(&self) -> &Self {
        let lines_count = self.buffer.lines().collect::<Vec<_>>().len();
        println!("{} {}", lines_count, self.file_name);
        self
    }
    fn count_words(&self) -> &Self {
        let word_count = self
            .buffer
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .len();
        println!("{} {}", word_count, self.file_name);
        self
    }
    fn count_chars(&self) -> &Self {
        let chars = self.buffer.chars().collect::<Vec<_>>().len();
        println!("{} {}", chars, self.file_name);
        self
    }
}
fn read_from_stdin() -> (String, usize) {
    let mut buffer = String::new();
    let bytes_read = io::stdin().read_to_string(&mut buffer).unwrap();
    (buffer, bytes_read)
}

fn main() {
    let wc = match WC::initialize() {
        Ok(wc) => wc,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    wc.execute();
}
