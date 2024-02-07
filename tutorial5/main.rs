// simple enum
enum Car{
    Golf,
    Bmw,
    Pursche,   
}
impl Car {
    fn turn_on(&self){
        println!("rm rm rm");
    }
}
// enum with values / mixed enum and struct
struct Point(u8,u8,u8);
enum Message{
    Greeting(String),
    Position(Point),
    Quit,

}
// Remarque when using match we need to match all data if we just interested in some we can represent the rest as _ or other
fn main(){
    let mycar=Car::Golf;
    let mycar2=Car::Bmw;
    mycar.turn_on();
    let message=Message::Greeting(String::from("Hello"));
    let messagep=Message::Position(Point(4,5,8));
    match mycar{
        Car::Golf => println!("good car"),
        Car::Bmw => println!("i don't like it"),
        Car::Pursche => println!("expensive one"),

    }
    match message{
        Message::Quit=> println!("bye"),
        Message::Greeting(s)=> println!("{}",s),
        Message::Position(p) => println!("your position is {} {} {}",p.0,p.1,p.2),

    }
    match messagep{
        Message::Quit=> println!("bye"),
        Message::Greeting(s)=> println!("{}",s),
        Message::Position(p) => println!("your position is {} {} {}",p.0,p.1,p.2),

    }

    // using if let
    if let Car::Bmw=mycar2{
        println!("Oh you get this car");
    }else{
        println!("good car");
    }
    
}