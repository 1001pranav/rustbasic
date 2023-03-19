// -> used for specifying return types
fn sum_of_two_num(num1:i32, num2:i32) -> i32 {
  return num1 + num2;
}
fn main() {
  println!("INSIDE main function");
  let two_sums:i32 = sum_of_two_num(12, 14);
  println!("Sum of two {two_sums}");
}

