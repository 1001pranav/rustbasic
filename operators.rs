fn main() {
    let mut sum;
    let mut mul:i32;
    let mut div:i32;
    let div_float : f32;
    let mut modulus : i32;
    let a = 131;
    let b = 20;
    let a_float =  131.0;
    let b_float = 20.0;

    sum  = a + b; //Can use += if you want to add to same variables
    print(a, b, sum, '+');
    
    mul = a * b;
    print(a, b , mul, '*');
    
    div = a / b;
    print(a, b, div, '/');

    div_float = (a_float / b_float) as f32;
    printfl(a_float, b_float, div_float, '/');

    modulus = a % b;
    print(a, b, modulus, '%');

    sum += mul;
    mul *= div;
    div /= modulus;
    modulus %= sum;

    //int data type and float data type together can't be used for divisible
    println!("{sum} {mul} {div} {modulus}");
}
fn print(a:i32, b:i32, res:i32, symb:char){
    println!("* {a} {symb} {b} = {res}");
}
fn printfl(a: f32, b:f32, res: f32, symb:char) {
    println!("* {a} {symb} {b} = {res} ");
} 