/* 
 * we will see in this tutorial collections 
 * we will see vectors
 * strings
 * hash map
 * */
 use std::collections::HashMap;
 fn main(){

    // create empty vector
    let v: Vec<i32> = Vec::new();
    //create vectore using value
    let v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    v3.pop();
    //access value inside vector
    let third: &i32 = &v2[1];
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
    let str0="hello";
    let mut s:String=String::from(str0);
    let s2:String=String::from(" world");
    s.push('c');
    println!("{s}");
    s=s+&s2;
    println!("{s}");
    let name="ali";
    let me = name.to_string();
    println!("{me}");
    let ss = format!("{me}-{s}-{name}"); // like println but print to string
    println!("{ss}");
    // hash map
    let mut scores = HashMap::new();//create hashmap

    scores.insert(String::from("Blue"), 10);// add value
    scores.insert(String::from("Yellow"), 50);
    let va=scores.get("Blue").copied().unwrap_or(0);
    println!("{va}");
    // iterate over hashmap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    scores.entry(String::from("Blue")).or_insert(50);// entry return reference; to value off added key
    //if key exist entery will do nothing

    //exemple use of hash map and updaating value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map{
        println!("{key}: {value}");
    }
 }