
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

