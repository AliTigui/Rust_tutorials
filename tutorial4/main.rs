// basic struct definition
struct Person {
    name: String,
    age: u8,
}
// struct with function
struct Cat {
    name: String,
    age: u8,
}
impl Cat {
    fn meow(&self) {
        println!("meow i am {}", self.name);
    }
}

//Tuple structure
struct Color(u8, u8, u8);
struct Empty;
//print rectange format
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// constructor function
    impl Rectangle {
        fn Square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

//
fn main() {
    let person1 = Person {
        name: String::from("Ali"),
        age: 22,
    };
    // copy data from structure to another one
    let person2 = Person {
        name: String::from("Ali"),
        ..person1
    };
    println!("{}", person2.age);

    let cat1 = Cat {
        name: String::from("mach mach"),
        age: 20,
    };
    let mut cat2 = Cat {
        name: String::from("mesho"),
        age: 20,
    };
    cat2.name = String::from("Mesho");
    cat2.meow();
    println!("{}", person1.name);
    cat1.meow();

    // making struct with  function
    let person3 = make_person(String::from("Vale"), 20);
    println!("{}", person3.name);
    let red = Color(255, 0, 0);
    println!("{}", red.0);

    // print structure formate using debug methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    let square = Rectangle::Square(10);
    println!("{}", square.width);
}
fn make_person(name: String, age: u8) -> Person {
    Person { name, age }
}
