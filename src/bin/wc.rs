use std::{
    env,
    fs::{self, File},
    io::Read,
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
    file_name: String,
}
// impl Args{
//
// }
impl WC {
    fn initialize() -> Result<Self, &'static str> {
        let mut args_iter = env::args();
        if args_iter.len() < 2 {
            return Err("Invalid number of arguments");
        }
        args_iter.next().unwrap_or_else(|| {
            panic!("Something went wrong");
        });
        let mut file_name = String::new();
        let args = args_iter
            .map(|e| match e.as_str() {
                "-c" => Args::ReadBytes,
                "-l" => Args::ReadLines,
                "-w" => Args::ReadWords,
                "-m" => Args::ReadChars,
                _ => {
                    file_name.insert_str(0, e.as_str());
                    Args::Input(e)
                }
            })
            .collect();
        // println!("{:?}", args);
        Ok(WC { args, file_name })
    }
    fn execute(&self) {
        if self.args.len() == 1 {
            //execute all shit
        }
        self.args.iter().for_each(|item| match item {
            Args::ReadBytes => self.count_bytes(),
            Args::ReadLines => self.count_lines(),
            Args::ReadWords => self.read_words(),
            Args::ReadChars => self.read_chars(),
            Args::Input(_) => {}
        });
    }
    fn count_bytes(&self) {
        let bytes_count = fs::read(&self.file_name).unwrap();
        println!("{} {}", bytes_count.len(), self.file_name);
    }
    fn count_lines(&self) {
        let mut buffer = String::new();
        let mut file = File::open(&self.file_name).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let lines_count = buffer.lines().collect::<Vec<_>>().len();
        println!("{} {}", lines_count, self.file_name);
    }
    fn read_words(&self) {
        let mut buffer = String::new();
        let mut file = File::open(&self.file_name).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let word_count = buffer.trim().split_whitespace().collect::<Vec<_>>().len();
        println!("{} {}", word_count, self.file_name);
    }
    fn read_chars(&self) {
        let mut buffer = String::new();
        let mut file = File::open(&self.file_name).unwrap();
        file.read_to_string(&mut buffer).unwrap();
        let chars = buffer.chars().collect::<Vec<_>>().len();
        println!("{} {}", chars, self.file_name);
    }
}

fn main() {
    let wc = WC::initialize().unwrap();
    wc.execute();
}
