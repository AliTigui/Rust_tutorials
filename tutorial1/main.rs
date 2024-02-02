// this line comment
/*
  This multiple line commane /*this nested comment */
*/
fn main() {
    /* we can see below primitive data type in Rust */
    /*
     * Integer : signed [i8, i16, i32, i64, i128, isize] | unsigned [u8, u16, u32, u64, u128, usize]
     * Float : [f8,f16,f32,f64,f128]
     * Boolean : bool [true, false]
     * Char : ['A', 'b', ...]
     * Tuple : Collection of deferent value with defferent  type (4,5,'a') to access them we use dot notation with index
     * Array :Collection of deferent value of same  type [4,5] to access them we use brackets with index
     */

    const PI: f64 = 3.14;
    let a: i32 = 4;
    let b: f64 = 4.5;
    let name: &str = "Ali";
    let b: bool = true;
    let c = 'c';
    let position = (4, 5, 6);
    let array = [4, 5, 6];
    let array=[0;500];// array of 500 zeros
    println!("{}",array[1]);
    // all those type are stored in stack S
    let name:String=String::from("Ali");
    println!("my name is {}",name);
}
