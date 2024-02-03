fn main(){
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
}