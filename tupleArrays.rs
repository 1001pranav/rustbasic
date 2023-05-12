fn main() {
  //Tuple 
    let student: (&str, bool, i32, f32) = ("Pranav", true, 23, 5.35);
    println!(
        "Name is {:?}, is_male -> {:?}, Age is {:?}, CGPA = {:?}",
        student.0, student.1, student.2, student.3
    );
    println!("Print tuple at a time {:?}", student);
}

fn destructureTuple()