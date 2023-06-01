fn main(){
    let cmd_line = std::env::args();
    println!("Len of elements in arguments is :{}",cmd_line.len()); 
    //print total number of values passed
    let mut text = String::new();
    let mut c = 0;
    for arg in cmd_line {
        if c == 0 {
            //pass
        }else if c == 1 {
            * &mut text += &arg;
        }else{
            * &mut text += &((" ".to_string()) + &arg);
        }
        c += 1;
    }
    println!("{}", text); //print all values passed as commandline arguments
 }