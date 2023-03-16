fn main() {
    // If condition.
    let num: i32 = 6;
    if num > 5 {
        println!("Number is greater than  5, Number is {}", num);
    }

    const NUM: i32 = 11;
    // IF...ELSE
    if NUM % 2 == 0 {
        println!(" Number is Even {NUM}");
    } else {
        println!("Number is Odd {NUM}");
    }

    // IF..ELSE IF..IF
    if NUM > 0 {
        println!("Number {NUM} is positive");
    } else if NUM < 0 {
        println!("Number {NUM} is negative ");
    } else {
        println!("Number is 0");
    }

    let mut sum: i32 = 0;
    //             startNum..endNum -> not inclusive NUM-1
    for temp_data in num..NUM {
        if temp_data % 2 == 0 {
            sum += temp_data;
        } else {
            continue;
        }
    }

    println!("Sum is {sum}");

    // Same like switch.
    let state_code = "KA";
    let state = match state_code {
        "MH" => {
            println!("Found match for MH");
            "Maharashtra"
        }
        "KL" => "Kerala",
        "KA" => "Karnataka",
        "GA" => "Goa",
        _ => "Unknown", //Default
    };
    println!("State name is {}", state);

    let mut x = 0;
    while x < 10 {
        x += 1;
    }
    println!("After while loop {x}");

    loop {
        x -= 1;
        if x == 0 {
            break;
        }
    }
    println!("After loop x = {x}");
}
