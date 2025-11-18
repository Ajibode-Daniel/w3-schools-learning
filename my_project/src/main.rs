// fn main() {
//     print!("Hello, world!\n");
//     print!("I am learnig Rust\n");
//     print!("It is awesome")
// }

// fn main () {
//     let name = "John";
//     let age = 30;
//     println!("my first name is: {}, I am {} years old", name, age)
// }

// fn main () {
//    let mut x = 5;
//    println!("Before: {}", x);
//    x = 10;
//    println!("After: {}", x);
// }

// fn main () {
//    let my_num: i32 = 5;
//    let my_double: f64 = 5.99;
//    let my_letter: char = "D";
//    let my_bool: bool = true;
//    let my_text: &str = "Daniel"

//    const BIRTHYEAR: i32 = 1998;
//    const FULLNAME: &str = "AJIBODE DANIEL"
   
// }

// fn main() {
//   let mut x = 10;
//   println!("Start: {}", x);

//   x += 5;
//   println!("After += 5: {}", x);

//   x -= 2;
//   println!("After -= 2: {}", x);

//   x *= 2;
//   println!("After *= 2: {}", x);

//   x /= 3;
//   println!("After /= 3: {}", x);

//   x %= 4;
//   println!("After %= 4: {}", x);
// }

// fn main() {
//   let a = 5;
//   let b = 10;

//   println!("5 == 10: {}", a == b);
//   println!("5 != 10: {}", a != b);
//   println!("5 < 10: {}", a < b);
//   println!("5 >= 10: {}", a >= b);
// }

// fn main(){

//     let logged_in = true;
//     let is_admin = false;

//     println!("Is regular user:{}", logged_in && !is_admin);
//     println!("Has any access:{}", logged_in || is_admin);
//     println!("Not logged in:{}", !logged_in);
// }

// fn main(){
//     let is_programming_fun = true;
// let is_fish_tasty = false;

// println!("Is Programming Fun? {}", is_programming_fun);
// println!("Is Fish Tasty? {}", is_fish_tasty);
// }

// fn main(){
    
//     let age = 20;
//     let can_vote = age >= 18;
    
//     println!("Can vote? {}", can_vote);
// }

// fn main(){
//     let is_logged_in = false;

// if is_logged_in {
//   println!("Welcome back!");
// } else {
//   println!("Please log in.");
// }
// }

// fn main(){
//     if 7 < 5 {
//         println!("7 is greater than 5")
//     } else {
//         println!("5 is less than 7")
//     }
// }

// fn main(){
//     let score = 50;

// if score >= 90 {
//     println!("Grade: A")
// } else if score >=80 {
//     println!("Grade: B")
// }else if score >=70 {
//     println!("Grade: C")
// }else if score >=60 {
//     println!("Grade: D")
// } else {
//     println!("Grade: F")
// }
// }

// fn main(){
//     let time = 20;
// let greeting = if time < 18 {
//   "Good day."
// } else {
//   "Good evening."
// };
// println!("{}", greeting);
// }

// fn main(){
//     let time = 15;
//     let greeting = if time < 18 {"Good Day.."} else {"Good Evening.."};
//     println!("{}", greeting)
// }

// fn main(){
//     let day = 4;

//   match day {
//     1 => println!("Monday"),
//     2 => println!("Tuesday"),
//     3 => println!("Wednesday"),
//     4 => println!("Thursday"),
//     5 => println!("Friday"),
//     6 => println!("Saturday"),
//     7 => println!("Sunday"),
//     _ => println!("Invalid day."),
//   }
// }

// fn main(){
//     let day = 5;


//     if day == 1 {
//     println!("Monday")
// } else if day == 2  {
//     println!("Tuesday")
// }else if day == 3 {
//     println!("Wednesday")
// }else if day == 4 {
//     println!("Thursday")
// } else if day == 5 {
//     println!("Friday")
// } else if day == 6 {
//     println!("Saturday")
// }else if day == 7{
//     println!("Sunday")
// }
// }

// fn main() {
//   let day = 6;

//   match day {
//     1 | 2 | 3 | 4 | 5 => println!("Weekday"),
//     6 | 7 => println!("Weekend"),
//     _ => println!("Invalid day"),
//   }
// }

// fn main() {
//   let day = 4;

//   let result = match day {
//     1 => "Monday",
//     2 => "Tuesday",
//     3 => "Wednesday",
//     4 => "Thursday",
//     5 => "Friday",
//     6 => "Saturday",
//     7 => "Sunday",
//     _ => "Invalid day.",
//   };

//   println!("{}", result);
// }

// fn main() {
//   let mut count = 10;

// while count <= 50{
//   println!("This won't be printed.");
//   println!("Count: {}", count);
//   count += 1;
// }
// }

// fn main() {
//   let mut count = 10;

// while count <= 50{
//   println!("This won't be printed.");
//   println!("Count: {}", count);
//   count += 1;
// }
// }

// fn main(){
//   let mut num = 1;

//   while num <=10{
//     if num == 6 {
//       break;
//     }
//     println!("Number:{}", num);
//     num += 1;
//   }
// }

// 

// fn main(){
//   for i in 1..6{
//     println!("i is: {}", i);
//   }
// }

// fn main(){
//   for i in 1..=6{
//     println!("i is: {}", i);
//   }
// }

// fn main(){
//   for i in 1..=10{
//     if i == 3{
//       continue;
//     }
//     if i == 7{
//       break;
//     }
//     println!("i is: {}", i);
//   }
// }

// fn main(){

// fn say_hello(){
//     println!("Hello from a function!");
// }

// say_hello();

// }

// fn main(){

//     fn greet(name: &str ){
//         println!("Hello, {}!", name);
//     }

//     greet("John");
// }


// fn main(){
//     fn add (a:i32, b:i32) -> i32{
//         return a + b;
//     }

//     let sum = add (4,7);
//     println!("Sum is {}", sum);
// }

// fn main(){
//     fn add (a:i32, b:i32) -> i32{
//          a + b
//     }

//     let sum = add (4,7);
//     println!("Sum is {}", sum);
// }

// fn main(){
//     fn myFunction(){
//         let message = "Hello!";
//         println!("{}", message);
//     }

//     myFunction();

//     // println!("{}", message);
// }

// fn main(){
//     let score = 40;

//     if score > 50 {
//         let result = "Pass";
//         println!("Result: {}", result);
//     } else {
//         let result = "Fail";
//         println!("Result: {}", result);
//     }
    
// }

// fn main(){
//     let score = 80;
//     let result = if score > 50 {
//         "Pass" // No semicolon here
//     } else {
//         "Fail" // No semicolon here
//     }; // Semicolon here to end the entire let statement

//     println!("Result: {}", result)
// }

// fn main(){
//     let x = 4;
//     {
//         let x= 12;
//         println!("Inside block: {}", x);
//     }

//     println!("Outside block: {}", x);
// }

// fn main(){
//     let greeting: &str = "Hello";
//     println!("{}", greeting);


// }

// fn main(){
//     let text1 = "Hello World".to_string();
//     let text2 = String::from("Hello World2");

//     println!("{} & {}", text1,text2);
// }

// fn main(){
//     let mut greeting = "Hello".to_string();
//     greeting.push_str(" World");
//     println!("{}", greeting);
// }

// fn main(){
//     let mut word = "Hello".to_string();
//     word.push('!');
//     println!("{}", word);
    
// }

// fn main(){
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let s3 = String::from("What a beautiful day!");

//     let result = format!("{} {} {}", s1, s2, s3);

//     println!("{}", result);
// }

// fn main(){
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let s3 = String::from("What a beautiful day!");

//     let result = s1 + " " + &s2 + " " + &s3;
//     println!("{}", result);
// }

// fn main(){
//     let name = String::from("Daniel");
//     println!("Length: {}", name.len());
// }

// fn main(){
//     let a = String::from("Hello");
//     let b = a;

//     // println!("{}", a);
//     println!("{}", b);
// }

// fn main(){
//     let a = 5;
//     let b = a;

//     println!("a = {}", a);
//     println!("b = {}", b);
// }

// fn main(){
//     let a = String::from("Hello");
//     let b = a.clone();

//     println!("a = {}", a);
//     println!("b = {}", b);
// }

// fn main(){
//     let a = String::from("Hello");
//     let b = &a;

//     println!("a = {}", a);
//     println!("b = {}", b);
// }

// fn main(){
//     let mut name = String::from("John");
//     let name_ref = &mut name;
//     name_ref.push_str(" Doe");

//     println!("{}", name_ref);
// }

// fn main(){
//     let mut word = "John".to_string();
//     word.push_str(" Doe");
//     println!("{}", word);
    
// }

// fn main(){
//     let fruits = ["apple", "banana", "orange", "mango"];
//     println!("Fav fruit: {}", fruits[2]);
// }

// fn main(){
//     let mut fruits = vec!["apple", "banana"];
//     fruits.push("cherry");
    
//     println!("Last fruit: {}", fruits[2]);
// }

// fn main(){
//     let person = ("John", 30, true);
//     println!("Name: {}", person.0);
//     println!("Age: {}", person.1);
//     println!("Is_Active: {}", person.2);
// }

// use std::collections::HashMap;

// fn main(){
//     let mut capitalCities = HashMap::new();
//     capitalCities.insert("France", "Paris");
//     capitalCities.insert("Japan", "Tokyo");

//     println!("Capital of Japan is {}", capitalCities["Japan"]);
// }