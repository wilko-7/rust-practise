pub fn ass () {
    let mut line = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line);
    let line_count = no_space(line);
    let final_line = line_count.len() - 1;
    println!("you have {}, charcters in you name", final_line);
 }



fn no_space(x : String) -> String{
    x.replace(" ", "")
  }
