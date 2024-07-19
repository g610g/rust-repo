use std::{env, fs};

#[derive(Debug)]
enum Args {
    ReadBytes,
    ReadLines,
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
            Args::ReadBytes => {
                self.count_bytes();
            }
            Args::ReadLines => {}
            Args::ReadWords => {}
            Args::Input(_) => {}
        });
    }
    fn count_bytes(&self) {
        let bytes_count = fs::read(&self.file_name).unwrap();
        println!("{} {}", bytes_count.len(), self.file_name);
    }
}

fn main() {
    let wc = WC::initialize().unwrap();
    wc.execute();
}
