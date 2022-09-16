#![allow(unused)]

fn even_uneven(x: u8)->String {
    if x%2 == 0 {return "YES".to_string()}
    
    return "NO".to_string()
    
}

    
    
    fn main() {  
    let mut i: u8 = 1;
    loop{
        
        
        println!("Can the value: {}\n be divided by two evenly?   {:?}",i, even_uneven(i));
        i+=1;
        if i ==10{break;}
        }
            
    }
