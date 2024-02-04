use std::{error::Error, io, process};
#[derive(Debug)]
struct Process{
    id:u32,
    burst_time:u32,
    turn_around_time:u32,
    wait_time:u32
}
impl Process {
    fn build (burst_time:u32, id:u32)-> Process {
        Process{
            id,
            burst_time,
            turn_around_time: 0,
            wait_time:0 
        }
    }
    fn assign_waiting_turn(&mut self, wait_time: u32){
        self.wait_time = wait_time;
        self.turn_around_time = self.wait_time + self.burst_time;

    }
}
fn create_processes(length: u32, processes: &mut Vec<Process>) -> Result< &Vec<Process>, Box<dyn Error>> {
    for i in 0..length{
        let mut burst_time = String::new();
        println!("Enter burst time for process {} : ", i);
        io::stdin().read_line(&mut burst_time)?;        
        let burst_time = burst_time.trim().parse::<u32>()?;
        processes.push(Process::build(burst_time, i));
    }
    Ok(processes)
}
fn  process_waiting_turn(processes: &mut Vec<Process>){
    let mut  wait_time = 0;
    processes.iter_mut().for_each(|p| {
        p.assign_waiting_turn(wait_time);
        wait_time += p.burst_time;
    });

    processes.iter().for_each(|p| println!("{:?}", p));
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut process_length = String::new();
    let mut processes:Vec<Process> = Vec::new();
    println!("Enter number of processes: ");
    io::stdin().read_line(&mut process_length)?;
    let process_length = process_length.trim();
    if let Err(e) = create_processes(process_length.parse()?, &mut processes){
        println!("Error processing the program: {}", e);
        process::exit(1);
    }
    process_waiting_turn(&mut processes);
    Ok(())

}