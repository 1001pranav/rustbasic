fn main() {
    
    //To Assign a string
    let mut assigned_string = String::new(); 

    // For predefined string
    const LANGUAGE: &str = "Rust";
    const PHASE: &str = " beginning ";

    let length: usize = LANGUAGE.len();
    //const variables once assigned cannot be changed

    println!("\n* Before assigning length is {} And string is '{assigned_string}'", assigned_string.len());
    
    assigned_string = String::from("String is Assigned" );
    
    println!("* After assigning length is {},  And string is '{assigned_string}'" ,assigned_string.len());

    println!("* we can print multiple variables with '{1}' => '{0}' or '{LANGUAGE}', '{PHASE}'", LANGUAGE, PHASE);
    println!("* Length of string {LANGUAGE} is {length}");

    //To replace string
    let replaced_string = assigned_string.replace("Assigned", "Replaced");
    println!("* Replaced string is '{replaced_string}'");

    let mut string_methods = "   
    String Example
        * to_string(), 
        * push(), 
        * push_str()
        * len() 
        * split_whitespace()
        * split()

    ".to_string();
    
    println!("to_string() demo {string_methods}");

    //Can add a character only
    string_methods.push('.');

    string_methods.push_str("\n Added string     ");
    println!(" Final string is {string_methods}");
    
    println!("Length before .trim(), {}", string_methods.len());
    println!("Length after .trim(), {}, and string is {}", string_methods.trim().len(), string_methods.trim());

    let mut index = 1;
    println!("****** split_whitespace method ******");
    for token in string_methods.split_whitespace() {
        println!("  - {token} {}", index);
        index += 1;
    }

    println!("****** split method ******");
    for token in string_methods.split("*") {
        println!(" - {token}");
    }

    // &str cannot be used to concatenation
    //  if you want to use &str then use to_owned()
    let reference_assign = LANGUAGE.to_owned() + &PHASE;

    println!(" Assigning using reference {}", reference_assign);

    println!("**** Printing char from string ****");

    for chr in LANGUAGE.chars() {
        println!("{chr}");
    }

    let formatting= format!("{} in learning {}",  PHASE, LANGUAGE);
    
    println!(" using macro function format!() - '{formatting}'");
}