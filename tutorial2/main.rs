fn main(){
    
    /*
     * we can control how our code work using [if, else, else if]
     * there is 3 type of loopi n rust
       - for we use it to go trough collection of data or a range /* we create range using  1..4 (4 is exclusive )*/
       - loop it infinite loop to stop it we need break 
       - while it repeate itself until the condition inside it become not valide
    * Expresssion vs stateent 
       - statement is line or block of code that return nothing
       - expression is line or block of code that return value
       - expression end without ;
    * Functions
       - we create function using fn keyword
       - there is defferent types of funtions
         1: function without return value anr paramatre
         2: function with parametre (we should specify the parametre type)
         3: function with return value and parametre (we should spicify the return type and the parametres type)  
     */

    // ===================== Loop and conditions ==============================
    for i in 0..4 {
        println!("i : {}",i);
    }
    let mut j=0;
    loop{
        j +=1 ;
        if j>10{
            break;
        }
        else if j==2{
            continue;
        }else{
            println!("j : {}",j);
        }
    }
    let mut j=0;
    while j<10{
        j=j+1;
        println!("j*j ={}",j*j);

    }
    func1();
    func2(4);
    let r=func3(4,5);
    println!("returned value is {}",r);
    
    
}
// =========================== Functions ====================
fn func1(){
    println!("this function don't take parametre and don't return value");
}
fn func2(p:i32){
    println!("this function take parametre {} and return nothing",p);
}
fn func3(p1:i32,p2:i32)->i32{
    println!("this function take parameters {} {},  and return return value p1+p2 :{}",p1,p2,p1+p2);
    p1+p2
    // we can return value from function by just using expression at and of it or using |return| keyboard
}