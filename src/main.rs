use std::{error::Error, io, process};
use practice_package::os;



fn main() -> Result<(), Box<dyn Error>> {
    let mut process_length = String::new();
    let mut processes:Vec<os::Process> = Vec::new();
    println!("Enter number of processes: ");
    io::stdin().read_line(&mut process_length)?;
    let process_length = process_length.trim();
    if let Err(e) = os::create_processes(process_length.parse()?, &mut processes){
        println!("Error processing the program: {}", e);
        process::exit(1);
    }
    os::process_short_job_first(&mut processes);
    
    os::print_results(&processes, os::calculate_averages(&processes));
    Ok(())

}