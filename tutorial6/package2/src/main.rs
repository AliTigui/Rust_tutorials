/*pub mod resturant;
pub mod worker;
ifwe compile using rustc we add modules name here and we use direct name
if we compile using cargo
we put module name in lib.rs and use package name
*/

fn main() {
    println!("Hello, world!");
    package2::worker::cheff::cook(); 
    package2::user::hello(); 
}
