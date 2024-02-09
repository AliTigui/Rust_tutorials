/* 
 * we will see in this tutorial collections 
 * we will see vectors
 * strings
 * hash map
 * */

 fn main(){

    // create empty vector
    let v: Vec<i32> = Vec::new();
    //create vectore using value
    let v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    v3.pop();
    //access value inside vector
    let third: &i32 = &v[2];
    println!("The third elemenst is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // using get return for us Option<&T> so we need to use match with it
    // deference between those methodes is get return None when we out of range but indexing will cuz error
    //iterate throught value of vector
    //not mute
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    //mut
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // to make vector hold data of defference type we can use enum
 }