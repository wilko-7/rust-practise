use std::io::Read;



pub fn ass() {
    let mut input1 = String::new();
    let mut output1 = inputs(input1);
    print!("number {}", output1)
}

fn inputs(mut input1: String) -> String{
    println!("Enter a postive number:");
    std::io::empty().read_to_string(&mut input1);
    std::io::stdin().read_line(&mut input1);
    for i in no_space(input1.trim().to_string()).chars(){
        if i.is_numeric() == false {
            println!("please put in a nummber without letters or negative numbers \n");
            let err = String::from("err");
            let mut x = String::from("");
            inputs(x);
            return err;
        } 
       return  input1
    };
    let mut output1 = input1; 
    return output1;
}


fn no_space(x : String) -> String{
    x.replace(" ", "")
  }
