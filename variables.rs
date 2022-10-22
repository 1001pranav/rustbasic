fn main() {

    /*  ### DECLARE VARIABLES ###  
         ** Cant declare variables like **
            let x = 1, y = 2; 
        Here we can't update x and y variables 
    */    

    let x = 1;
    let y = 2;
     
    println!("* printing variables \n* x = {} And y = {} ", x, y);
    println!("* {{}}=> will print variables", );
    
    let sum;
    let mut mul = 10;

    sum = x + y;
    print!("* {x} + {y} = {sum}" );

    /* 
        *** "mut" keyword is used to change the declaration time assigned data ***
        In the case of mul previously it was declared as 10.
        Here we wanted to re-assign the same variable with 
        multiplication of same number with sum of X and Y
        therefore we have used mut to reassign
    */

    mul *= sum;
    print!("\n* {} * {sum} = {mul}", mul/sum);
    
    /*  
                ### DATA Types ###
        
            *) int - :<letter><bitSize>
                <letter> - 
                    i - For signed numbers
                    u - For unsigned number
                <bitSize> - 8, 16, 32, 64, 128, size
            *) float - :f<bitSize>
            *) boolean - :bool (true, false)
            *) character(including special char) - use single quotes for char
    */

    let int : i32 = 10;
    let float : f32 = 13.33;
    let total : i32 = 15_00_000;
    let character : char = 'a';
    let boolean :bool = false;

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


    println!("\n- int => {int} \n- float => {float} \n- char => {character}\n-bool => {boolean} ");
    println!("\n int with _ {total}");

    /*
            *** Data type conversion ***
        -> We can only explicitly convert 
            Rust wont automatically convert.

        "as" used for type conversion
    */

    let decimal :f32 = 32.69540;
    let integer :i32 = decimal as i32;
    println!("- decimal {decimal} to integer typecasting {integer}");

    let char_to_int :char = 'a';
    let int_var_as_char = char_to_int as i32;
    println!("- character {char_to_int} to integer {int_var_as_char}");

    /*
        for converting int to char int needs to be "u8" 
    */

    let int_to_char :u8 = 192;
    let char_as_int :char = int_to_char as char;
    println!("- int {int_to_char} as char {char_as_int}");

    let boolean_true_data :bool = true;
    let boolean_false_data :bool = false;

    let bool_to_int :u8 = boolean_true_data as u8;

    println!("- true is represented in boolean as '{boolean_true_data}' ");
    println!("- false is represented in boolean as '{boolean_false_data}' ");
    println!("- type conversion from boolean to int {bool_to_int}");

}