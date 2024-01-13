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
    pub struct User{
        username:String,
        password:String,
   }
    impl Authenticatable for User{
        fn display_user_credentials(&self){
            println!("username: {}, password:{}", self.username, self.password);
        }
        fn username(&self)-> &str {
            &self.username[..]
        }
    }
    impl User{
        pub fn new(username:String, password:String)->User{
            User{
                username,
                password
            }
        }
    }
    pub fn verify_email<T: Authenticatable>(user1: &T){
    }

   pub struct Profile<T>{
        user:T,
        likes:u64,
        has_badge:bool
   }
   impl <T: Authenticatable> Profile<T>{
    pub fn show_profile(&self){
        if self.has_badge{
            println!("username:{} has {} likes and has a badge", self.user.username(), self.likes);
        }else{
            println!("user {} has no badge", self.user.username());
        }
    }
   }
}