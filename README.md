# This contains Basics of **RUST** programming language
## Rust file will have the extension _.rs_ 
> To install Rust in your system please visit [install](https://rustup.rs/#)

> To execute the Rust  program follow the steps below  (plain rust not using cargo)
   
   - `rustc ./<filename>.rs` 
   > The above line will convert the source code to executable code _.exe_  file

   - `./<filename>.exe` (incase of Windows) `./<filename>` (for linux and mac)
   > Will execute the rust commands 

# [HelloWorld.rs](/HelloWorld.rs)
* `HelloWord.rs` Contains Basic macro to print output
* Here macro means family of features in rust more can be learnt in official docs [macro](https://doc.rust-lang.org/book/ch19-06-macros.html#:~:text=The%20term%20macro%20refers%20to,attributes%20usable%20on%20any%20item)

![hello world](https://user-images.githubusercontent.com/73541801/195999485-778f94e6-689f-4179-bbaa-557f033f93e6.png)

# Varibles [variables and basic data types](/variables.rs)   
           
* variables.rs Contains the variables and basics of Data types  
* let and const are two variables in Rust
* Use of mut 
* Basic Data types are -
   - *_int_* (signed, unsigned)
   - *_float_*
   - *_bool_*
   - *_char_*
* type conversion of different data types 

## output of [Variables.rs](/variables.exe)
```   
* printing variables 
* x = 1 And y = 2 
* {}=> will print variables
* 1 + 2 = 3
* 10 * 3 = 30
* Before assigning length is 0 And string is ''
* After assigning length is 18,  And string is 'String is Assigned'
* we can print multiple variables with ' beginning ' => 'Rust' or 'Rust', ' beginning '
* Length of string Rust is 4

- int => 10
- float => 13.33
- char => a
- bool => false

 int with _ 1500000
- decimal 32.6954 to integer typecasting 32
- character a to integer 97
- int 192 as char À
- true in boolean 'true'
- false in boolean 'false'
- type conversion from boolean to int 1
```
# Strings [strings and Operations on string](/strings.rs)
## Different functions of strings and use of strings is mentioned
* &str and String::new() methods
* Use of different string methods
* to_string(), 
   * push()
   * push_str()
   * len() 
   * split_whitespace()
   * split()
   * to_owned()
   * chars()
   * format!()
   * reference concatenation

### output of [stings.rs](/strings.exe)

```
* Before assigning length is 0 And string is ''
* After assigning length is 18,  And string is 'String is Assigned'
* we can print multiple variables with ' beginning ' => 'Rust' or 'Rust', ' beginning '
* Length of string Rust is 4
* Replaced string is 'String is Replaced'
to_string() demo
    String Example
        * to_string(),
        * push(),
        * push_str()
        * len()
        * split_whitespace()
        * split()


 Final string is
    String Example
        * to_string(),
        * push(),
        * push_str()
        * len()
        * split_whitespace()
        * split()

    .
 Added string
Length before .trim(), 176
Length after .trim(), 163, and string is String Example
        * to_string(),
        * push(),
        * push_str()
        * len()
        * split_whitespace()
        * split()

    .
 Added string
****** split_whitespace method ******
  - String 1
  - Example 2
  - * 3
  - to_string(), 4
  - * 5
  - push(), 6
  - * 7
  - push_str() 8
  - * 9
  - len() 10
  - * 11
  - split_whitespace() 12
  - * 13
  - split() 14
  - . 15
  - Added 16
  - string 17
****** split method ******
 -
    String Example

 -  to_string(),

 -  push(),

 -  push_str()

 -  len()

 -  split_whitespace()

 -  split()

    .
 Added string
 Assigning using reference Rust beginning
**** Printing char from string ****
R
u
s
t
 using macro function format!() - ' beginning  in learning Rust'
```

# [Arithmetic Operators](/operators.rs)

* use of +, -, /, *, %
* use of int and float

_Output of Arithmetic operators_
```
* 131 + 20 = 151
* 131 * 20 = 2620
* 131 / 20 = 6
* 131 / 20 = 6.55
* 131 % 20 = 11
* 2771 15720 0 11
```

# [Conditions and Loops](/conditionsNdLoops.rs)

* Use of conditional statements like `if, if..else, if..else if..else, match `
* Use of Loops like `for`
