use std::io::{self, Write};
pub fn ass() {
    let input1 = String::new();
    let output1 = no_space(inputs(input1, "first".to_string()));
    let input2 = String::new();
    let output2 = no_space(inputs(input2, "second".to_string()));
    let plus = addition(output1.clone(), output2.clone());
    let sub = subtraction(output1.clone(), output2.clone());
    let time = times(output1.clone(), output2.clone());
    let divi = division(output1.clone(), output2.clone());
    println!("first number = {}, second number = {}", output1, output2);
    println!("{} + {} = {}", output1, output2, plus);
    println!("{} - {} = {}", output1, output2, sub);
    println!("{} X {} = {}", output1, output2, time);
    println!("{} / {} = {}", output1, output2, divi);
}

fn inputs(mut input1: String, extra: String) -> String{
    print!("Enter your {} postive number: ", extra);
    io::stdout().flush().ok();
    std::io::stdin().read_line(&mut input1).ok();
    for i in no_space(input2.trim().to_string()).chars(){
        if i.is_numeric() == false {
            println!("\nplease put in a nummber without letters or negative numbers\n");
            let x = String::from("");
            let output1 = inputs(x, extra);
            return output1;
        } 
    };
    let output2 = input1;
    return output2;
}

fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
  }

  fn addition (x: String, y : String) -> String {
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    let add = num1 + num2 ;
    return add.to_string();
}

fn subtraction (x: String, y : String) -> String {
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    let sub = num1 - num2 ;
    return sub.to_string();
}
fn times (x: String, y : String) -> String {
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    let time = num1 + num2 ;
    return time.to_string();
}
fn division (x: String, y : String) -> String {
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    let div = num1 / num2 ;
    return div.to_string();
}
