pub(crate) fn main() {
    //println!("Hi, mom!");
    //#[warn(unused_mut)]

    //let name = "Shannon";
    println!("What is your name?");

    let output = get_name(get_input());
    
    println!("Hi, {}",output);
    println!("{}, How are you doing?", output);
    get_name(get_input());

}
//#[warn(dead_code)]
fn get_name(name:String) -> String{
    return name;
}

fn get_input() -> String{
  
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}