mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
use front_of_house::hosting::add_to_waitlist;
mod customer {
    fn eat_at_restaurant(){
        super::add_to_waitlist();
    }
}
pub mod authentication{
    pub trait  Authenticatable{
        fn display_user_credentials(&self);
        fn username(&self)->&str;
    }
    #[derive(Debug)]
    pub struct User<'a>{
        username:&'a String,
        password:String,
   }
    impl<'a> Authenticatable for User<'a>{
        fn display_user_credentials(&self){
            println!("username: {}, password:{}", self.username, self.password);
        }
        fn username(&self)-> &str {
            &self.username[..]
        }
    }
    impl<'a> User<'a>{
        pub fn new(username: &'a String, password:String)->User{
            User{
                username,
                password
            }
        }
        // pub fn check_list<'a, 'b>(&'a self, vip_users: &'b [String]) -> &'b str{
        //     for (index, user) in vip_users.iter().enumerate(){
        //         if (user == &self.username){
        //             return vip_users.get(index).unwrap();
        //         }
        //     }
        //     return vip_users.first().unwrap(); 
        // }
    }
    pub fn verify_email<T: Authenticatable>(user1: &T){
    }

   pub struct Profile<T>{
        user:T,
        likes:u64,
        has_badge:bool
   }
   //implements only this method on inner T where T has a Authenticatable trait
   impl <T: Authenticatable> Profile<T>{
    pub fn show_profile(&self){
        if self.has_badge{
            println!("username:{} has {} likes and has a badge", self.user.username(), self.likes);
        }else{
            println!("user {} has no badge", self.user.username());
        }
    }
   }
   impl<T> Profile<T>{
    pub fn new(user:T)->Self{
        Profile{
            user,
            likes:120,
            has_badge:false
        }
    }
   }
}
pub mod os{
    use std::{error::Error, io, process};
    #[derive(Debug)]
    pub struct Process{
        id:u32,
        burst_time:u32,
        turn_around_time:u32,
        wait_time:u32
    }
    impl Process {
        pub fn build (burst_time:u32, id:u32)-> Process {
            Process{
                id,
                burst_time,
                turn_around_time: 0,
                wait_time:0 
            }
        }
        pub fn assign_waiting_turn(&mut self, wait_time: u32){
            self.wait_time = wait_time;
            self.turn_around_time = self.wait_time + self.burst_time;
    
        }
    }
    pub fn create_processes(length: u32, processes: &mut Vec<Process>) -> Result< &Vec<Process>, Box<dyn Error>> {
        for i in 0..length{
            let mut burst_time = String::new();
            println!("Enter burst time for process {} : ", i);
            io::stdin().read_line(&mut burst_time)?;        
            let burst_time = burst_time.trim().parse::<u32>()?;
            processes.push(Process::build(burst_time, i));
        }
        Ok(processes)
    }
    pub fn  process_waiting_turn(processes: &mut Vec<Process>){
        let mut  wait_time = 0;
        processes.iter_mut().for_each(|p| {
            p.assign_waiting_turn(wait_time);
            wait_time += p.burst_time;
        });
        processes.iter().for_each(|p| println!("{:?}", p));
    }
    pub fn process_short_job_first(processes: &mut Vec<Process>){
        let mut wait_time = 0;
        processes.sort_by_key(|p| p.burst_time);
        processes.iter_mut().for_each(|p|{
            p.assign_waiting_turn(wait_time);
            wait_time+=p.burst_time;
        });
    }
    pub fn calculate_averages(processes: &Vec<Process>) -> (f32, f32){
        let mut overall_waiting: f32 = 0.0;
        let mut overall_turnaround: f32 = 0.0;
        processes.iter().for_each(|p|{
            overall_waiting+=p.wait_time as f32;
            overall_turnaround+=p.turn_around_time as f32;
        });
        let overall_waiting = overall_waiting / processes.len() as f32; 
        let overall_turnaround = overall_turnaround / processes.len() as f32;
        (overall_waiting, overall_turnaround)
    }
    pub fn print_results(processes: &Vec<Process>, average_tup: (f32, f32)){
        println!("Process\t\t\tBurst Time\t\t\tWaiting Time\t\t\tTurnaround Time");
        processes.iter().for_each(|p| {
            println!("P:{}\t\t\t{}\t\t\t\t\t{}\t\t\t\t\t{}", p.id, p.burst_time, p.wait_time, p.turn_around_time);
        });
        let (average_wait, average_turn) = average_tup;
        println!("Average Waiting Time: {}", average_wait);
        println!("Average Turnaround Time: {}", average_turn);
    }
}
