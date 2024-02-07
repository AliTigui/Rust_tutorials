mod resturant{
    pub const MAIN_DISH:&str="meat";
    const BEST_WORKER:i32=10;
    pub fn make_order(){
        println!("i want to it romato");
    }
    pub mod worker{
        pub fn work(){
            println!("i work hard");
        }
        pub fn best(){
            println!("the best worker is {}",super::BEST_WORKER);
        }
    }
}
mod client{
    pub fn review (){
        println!("i loved {}",crate::resturant::MAIN_DISH);
    }
    pub mod kid{
        pub fn play(){
            println!("playing in the yard");
        }
    }
}
use client::kid;
fn main() {
    println!("Hello, world!");
    crate::resturant::make_order();
    println!("{}",crate::resturant::MAIN_DISH);
    crate::client::review();
    resturant::worker::work();
    resturant::worker::best();
    kid::play();
}

