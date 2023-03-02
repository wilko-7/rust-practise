use std::io::{self, Write};

pub fn ass() {
let width_calc = String::new();
let height_calc = String::new();
let text = output(width_calc, height_calc);
println!("{}", text)
}

fn output (width_calc: String, height_calc: String) -> String{
    let width_calc = no_space(inputs(width_calc, "the width in feet".to_string()));
    let height_calc = no_space(inputs(height_calc, "the height in feet".to_string()));
    let squar = calc_squar(height_calc,width_calc);
    let mut output_text  = format!("\n\n {} squar feet\n", squar );
    let fetome = to_m2(squar); 
    let output_text2 = format!("\n {} squar meters\n\n", fetome );
    output_text.push_str(&output_text2);
    return output_text
}

fn inputs(mut input1: String, extra: String) -> String{
    print!("Enter your {} in a postive number: ", extra);
    io::stdout().flush().ok();
    std::io::stdin().read_line(&mut input1).ok();
    let input2 = input1;
    for i in no_space(input2.trim().to_string()).chars(){
        if i.is_numeric() == false {
            println!("\nplease put in a nummber without letters or negative numbers\n");
            let x = String::from("");
            let output1 = inputs(x, extra);
            return output1;
        } 
    };
    let output2 = input2;
    return output2;
}

fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
  } 

fn calc_squar(x: String,y: String) -> String {
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    let time = num1 + num2 ;
    return time.to_string();
}

fn to_m2(x: String) -> String{
    let num1: f32 = x.parse().unwrap();
    const CONV_NUM: f32 = 0.09290304;
    let num2 = CONV_NUM * num1;
    return num2.to_string();
}