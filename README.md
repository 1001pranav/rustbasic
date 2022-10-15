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

# Varibles
[variables.rs](/variables.rs)   
           
* variables.rs Contains the variables and basics of Data types  
* let and const are two variables in Rust
* Use of mut is displayed
* int (signed, unsigned), char, float, bool
* type conversion of different data types 

## output of Variables.rs
```
   PS D:\Code-Rust\rustbasic> rustc .\variables.rs
   PS D:\Code-Rust\rustbasic> .\variables.exe
   * printing variables
   * x = 1 And y = 2
   * {}=> will print variables
   * we can print multiple variables with  beginning  => Rust or Rust,  beginning
   * 1 + 2 = 3
   * 10 * 3 = 30
   - int => 10
   - float => 13.33
   - char => a
   -bool => false
   - int 192 as char À
   - true in boolean true
   -  false in boolean false
   - type conversion from boolean to int 1
```