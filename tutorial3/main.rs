fn main(){
    /*
     * scope is where variable is valide to use when variable get out of scope it get droped 
     * integer char literal-string array boolean and tupple are craeted in the stack
     * complex type like String use head and those type of variables apply ownership rules
     * when we assign variable that are stored in heap to another one the new one become owner and old become invalide to use
     * passing those complex variables to function make function parameter take ownership of them
     * function don't take ownership of simple variables like integer boolean ...
     * to fix ownership problems we use reference
     * reference is like making pointer to variables in the heap
     * Reference rule :
       - At any given time, you can have either one mutable reference or any number of immutable references.
       - References must always be valid.
     * to change value of referenced variable we have to create it mutable also it reference mutable
     * 
     */
    //===================== Scope ===================== 
    let a=4;
    let b=a;
    {
        let c=4;
        println!("a {} and b {} c {} ",a,b,c);
    }//c here is out of scope
    println!("both a {} and b {} still working",a,b);

    //====================== ownership ========================
    let s1:String=String::from("hello");
    let s2=s1; //s1 get new owner s2 if we try use s1 after this line we get error
    //println!("{}",s1);
    println!("{}",s2);
    let mut s3=s2.clone();
    func1(s2);// if you use s2 after this function you get error
    //println!("{}",s2);
    func2(a);// you can use a after this function
    println!("{}",a);
    //================== Reference and browing ================
    func3(&s3);// we can still be able to use s3 after function call
    println!("s3 still valide /{}/",s3);
    let r:&mut String=&mut s3;
    func4(r);
    println!("s3 still valide and it value changed /{}/",s3)
    /*
    let r1 = &mut s;
    let r2 = &mut s;
    -this code cus error we can't have 2 mut reference for variable
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    -this code cus error too w can't have mut reference with non mut reference
     */
    //======================= Slicing  ================




}
fn func1(p:String){
    println!("this function take ownership of p /{}/ you can't use it more",p);
}
fn func2(p:i32){
    println!("this function just made copy of p /{}/ you can use after the function",p);
}
fn func3(p:&String){
    println!("this value take only reference of /{}/ it don't take ownership",*(p))
}
fn func4(p:&mut String){
    p.push_str(" world");
}