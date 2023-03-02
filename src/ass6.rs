// using this so i can put a input on the same line as text
use std::io::{self, Write};
// using chrono to get the time form the system 
use chrono::prelude::*;

// replaces the main function for quick assingment switching
pub fn ass() {
    // here we create the variables we use throughout the app
    let current_age_in = String::new();
    let retirement_age_in = String::new();
    //here we create text we need
    let text = output(current_age_in, retirement_age_in);
    println!("{}",text);

 }

// this function was here so i can seperate the in and output 
 fn output(current_age_in   : String,retirement_age_in: String ) -> String{
    let dt = Utc::now();
    let current_age_out = no_space(inputs(current_age_in, "current age".to_string()));
    let retirement_age_out = no_space(inputs(retirement_age_in, "the age you want to retire as".to_string()));
    let calc_age_diff = age_diff(current_age_out, retirement_age_out);
    let mut output_text = format!("\n\nYou have years {} left until you can retire.", calc_age_diff );
    let retirement = till_year(calc_age_diff);
    let new_text = format!("\n\nit now {}, and you have {} years left until you can retire.\n", dt.year().to_string() ,retirement );
    output_text.push_str(&new_text);
    return output_text;
 }

 /*
    the re_run function was made so i could fix a bug.
    this bug caused the the exorted text to dubble it self
 */
 fn re_run(current_age_in: String,retirement_age_in: String) -> String{
    let current_age_out = no_space(inputs(current_age_in, "current age".to_string()));
    let retirement_age_out = no_space(inputs(retirement_age_in, "the age you want to retire as".to_string()));
    let calc_age_diff = age_diff(current_age_out, retirement_age_out);
    return calc_age_diff.to_string();
 }

// creates the input qustion
 fn inputs(mut input1: String, extra: String) -> String{
    print!("Enter your {} in a postive number: ", extra);
    // we use flush to be able to put the input next to the text instead of below it
    io::stdout().flush().ok();
    std::io::stdin().read_line(&mut input1).ok();
    /*
        to create a filter we use a for loop.
        the for loop checks if there are any charcters other then numbers in the loop
        if there are any charcters other then numbers in the loop it repeats the code
    */
    for i in no_space(input1.trim().to_string()).chars(){
        if i.is_numeric() == false {
            println!("\nplease put in a nummber without letters or negative numbers\n");
            let x = String::from("");
            let output1 = inputs(x, extra);
            return output1;
        } 
    };
    let output2  = input1;
    return output2;
}

/*
    this function basically removes all the whitespace from a strong.
    this is so i dont have to rewrite the command over and over again.
*/
fn no_space(x : String) -> String{
    x.replace(" ", "").trim().to_string()
  } 

// creates the value for the age difference
fn age_diff(x : String, y: String) -> String{
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = y.parse().unwrap();
    /*
        for this filter we just use a if statement,
        to check if num1 is bigger then num2
    */
    if num1 >= num2{
        println!("\nplease but your current age first and then the age you want to retire at\n");
        let x = String::from("");
        let y = String::from("");
        let output = re_run(x, y);
        return output;
    } else{
    let add = num2 - num1 ;
    return add.to_string();
    }
}

// creates the value for the time inbetween
fn till_year (x: String) -> String {
    let dt = Utc::now();
    let num1: i32 = x.parse().unwrap();
    let num2: i32 = dt.year();
    let add = num1 + num2 ;
    let retirement = add;
    return retirement.to_string();
}