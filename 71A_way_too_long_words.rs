#[allow(unused)]

fn no_more_than_10_string(text: &str) -> String {
    if text.to_string().len() < 10 {
        return text.to_string()
    }
    else {
        let length = text.to_string().len() - 1;
        let char_between = text.to_string().len() -2;
        let last_letter = length + 1;
        let result = text[0..1].to_string() + &char_between.to_string() + &text[last_letter-1..last_letter].to_string();
        return result 
    }
    
}
    
pub fn main() {  
    let x: Vec<&str> = vec!["word","localization",
"internationalization","pneumonoultramicroscopicsilicovolcanoconiosis"];
    
    for i in 0..4{

    println!("Test: {:?}", no_more_than_10_string(&x[i]));
    
        }
}
