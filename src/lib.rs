
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
pub mod linked_list{
    #[derive(Debug)]
    pub struct linkedList{
        pub value:i32,
        pub next: Option<Box<linkedList>>,
    }
    impl linkedList {
        pub fn create_node(value:i32, next: Option<Box<linkedList>>) -> Box<linkedList>{
            Box::new(linkedList{
                value,
                next 
            })
        }
        pub fn traverse_list(head: Option<&Box<linkedList>>)-> Result<(), &str>{
            let mut temp = head;
            if let None = temp{
                return Err("Cannot traverse the list. Empty List!");
            }
            while let Some(node) = temp{
                println!("{}", node.value);
                temp = node.next.as_ref();
            }
            Ok(())
        }
        pub fn insert_node_constant(mut head:Option<Box<linkedList>>, mut new_node:Box<linkedList>)-> Option<Box<linkedList>>{
            if let None = head{
                head = Some(new_node);
                return head;
            
            }else if let Some(current_node) = head{
                new_node.next = Some(current_node);
                head = Some(new_node);
            }; 
            return head;
        }
    }
}

