#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;   
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

//external modules
mod restraunt;
use crate::restraunt::ordure_food;

fn say_hello(){
    println!("Hello!");
}

fn get_sum(x: i32, y: i32) -> i32{
    println!("{}+{}={}", x, y, x+y);
    return x+y // this value is returned
}

fn get_2(x: i32)->(i32, i32){
    return (x+1, x+2);
}

fn sum_list(list: &[i32])->i32 // &[i32] is a reference to a vector
{
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    return sum;
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T)->T{ // uses generic types (unknown types)
    return x+y;
}

fn print_str(x: String){
    println!("A string: {}", x);
}

fn print_return_str(x: String)->String{
    println!("A string: {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("msg: {}", name);
}

fn main() {
    const ONE_MIL: u32 = 1_000_000; // unsigned 32 bit integer
    const PI: f32 = 3.141592; // 32 bit floating point number
    let age: &str = "20"; // &str is a string slice
    let mut age: u32 = age.trim().parse() // two different data type vars can have same name in
                                          // rust. trim() removes the whitespace in string
        .expect("Age was'nt assigned a number"); // expect() is used to handle errors
    println!("What is your name?");
    let mut name: String = String::new(); //empty string object 
    // by default all rust vars are immutable
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name) // &mut is a mutable reference
        .expect("Did'nt recieve input");

    println!("Hello {}! {}", name.trim_end(), greeting); // trim_end() removes the newline char at
                                                         // the end of the line
                                                         //
    println!("I am {} and I want ${}", age, ONE_MIL);
    println!("Max u32: {}", u32::MAX);

    let is_true: bool = true;
    let my_grade: char = 'A';

    let random_num = rand::thread_rng().gen_range(1..101); // gen_range() generates a random number
                                                           // between 1 and 100
    println!("Random number: {}", random_num);

    //ternary operator
    let mut my_age: i32 = 20;
    let can_vote: bool = if my_age >= 18{
        true
    } else {
        false
    };

    let age2: i32 = 8;
    match age2{
        1..=18 => println!("Important Birthday!"),
        21 | 50 => println!("Important Birthday!"),
        65..=i32::MAX => println!("Important Birthday!"),
        _ => println!("Not Important Birthday!"),
    };

    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("cant vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("You gained the right to vote!"),
    };

    let arr_1: [i32; 6] = [1, 2, 3, 4, 5, 6]; // fixed size and data type
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    //loops
    let mut loop_idx: usize = 0;
    loop {
       if loop_idx >= 6{
            break;
       }
       if arr_1[loop_idx] % 2 == 0{
            loop_idx += 1;
            continue;
       }
       println!("Val: {}", arr_1[loop_idx]);
       loop_idx += 1;
    }

    while loop_idx < arr_1.len(){
        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
    for val in arr_1.iter(){
        println!("Val: {}", val);
    }

    // tuples: multiple data types in fixed length
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("my tuple: {}", my_tuple.1); // reference a tuple val like this
    let (v1, v2, v3) = my_tuple;

    // String: The String class is an array of bytes and the &str is a reference to a string
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" Word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup(); // remove consecutive duplicates
    for char in v1{
        println!("{}", char);
    }

    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6]; // python like slicing. Grabs index 0 to 5
                          //
    println!("String length: {}", st5.len());
    st5.clear();

    let st6 = String::from("Just Some");
    let st7 = String::from("Words");
    let st8 = st6 + &st7; // + operator is overloaded to concatenate strings
                          // st6 does not exist after concatenation but st8 exist as it is
                          // reference
                          //
    for char in st8.bytes(){
        println!("{}", char);
    }

    //type casting
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    //enums
    enum Days{
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Days{
        fn is_weekend(&self) -> bool{
            match self{
                Days::Saturday | Days::Sunday => true,
                _=>false
            }
        }
    }

    let today: Days = Days::Monday;
    match today{
        Days::Monday => println!("Everyone hates monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost weekend"),
        Days::Saturday => println!("Weekend"),
        Days::Sunday => println!("Weekend"),
    }

    println!("Is today weekend? {}", today.is_weekend());

    // vector
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second) => println!("3nd: {}", second),
        None => println!("No second value"),
    }
    for i in &mut vec2{
        *i *= 2;
    }
    for i in &vec2{
        println!("{}", i);
    }
    println!("Vec length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());

    // functions
    say_hello();
    println!("{}",get_sum(5, 4));
    let (val_1, val_2) = get_2(3);
    println!("Nums: {}, {}", val_1, val_2);
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));

    //generics
    println!("5+4={}", get_sum_gen(5, 4));
    println!("5.2+4.6={}", get_sum_gen(5.2, 4.6));

    //ownership
    let str1 = String::from("World");
    let str2 = str1.clone(); //without using clone str1 would no longer exist
    println!("{}", str1);
    // print_str(str1);  after this line str gets moved and does not exist anymore
    let str3 = print_return_str(str1);

    // hashmap
    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent"); // key val pair
    heros.insert("Batman", "Bruce wayne");
    heros.insert("The flash", "Barry Allen");
    for (k, v) in heros.iter(){
        println!("{}={}", k, v);
    }
    println!("Length: {}", heros.len());
    if heros.contains_key("Batman"){
        let the_batman = heros.get("Batman");
        match the_batman{
            Some(x) => println!("Batman is a hero {}", x),
            None => println!("Batman is not a hero"),
        }
    }

    struct Customer{
       name: String,
       address: String,
       balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("Frankfut"),
        balance: 234.50,
    };

    bob.address = String::from("Zurich");
    
    // generics in struct
    struct Rectangle<T, U>{
        length: T,
        height: U,
    }

    let rect = Rectangle{length: 10, height: 4.5};

    trait Shape{
        fn new(length: f32, height: f32)->Self;
        fn area(&self)->f32;
    }

    struct Square{length: f32, height: f32};
    struct Circle{length: f32, width: f32};
    //implementing the trait for square
    impl Shape for Square{
        fn new(length: f32, height: f32)->Square{
            return Square{length, height};
        }
        fn area(&self)->f32 {
            return self.length * self.height;
        }
    }
    // can implement again for circle
    let rec: Square = Shape::new(10.0, 10.0);
    println!("{}", rec.area());

    ordure_food();

    //error handling and file i/o
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output{
        Ok(file)=>file,
        Err(error)=>{
            panic!("Problem creating file {:?}", error);
        },
    };

    write!(output, "Just some \nrandom words").expect("Failed to write to file");
    let input = File::open(path).unwrap(); // opens the file for reading
    let buffered = BufReader:: new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap()); //prints the contents of the line
    }
    let output2 = File::create("rand.txt");
    let output2 = match output2{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Unknown error: {:?}", error),
        },
    };

    //iterators
    let arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val); //this iterator gets values by borrowing, so we cannot change values
                             //in an array using this.
    }
    arr_it.into_iter(); // consumes the array so no longer array is accessible
    //custom iterators
    let mut iter1 = arr_it.iter();
    println!("1st value: {:?}", iter1.next());
    
    //closures (similiar to lambda or arrow functions)
    let can_vote = |myage: i32|{
        myage>=18 //this gets returned as it doesnt have semicolon at end
    };
    println!("Can vote: {}", can_vote(8));
    //can access and change values outside its scope
    let mut samp1 = 10;
    let mut change_var=|| samp1 += 2;
    change_var();
    println!("{}", samp1);

    //pass closure to a function
    fn use_func<T>(a: i32, b: i32, func: T)-> i32 where T: Fn(i32, i32)->i32{
       func(a, b) 
    }
    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5+4={}", use_func(5, 4, sum));
    println!("5*4={}", use_func(5, 4, prod));
    
    // box: it is a smart pointer that can store large amount of data into heap
    let b_int = Box::new(10);
    println!("b_int={}", b_int);

    //creating a binary tree data structure in RUST
    struct TreeNode<T>{
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T>{
       pub fn new(key: T)->Self{
            TreeNode{left: None, right: None, key,}
       }
       pub fn left(mut self, node: TreeNode<T>)->Self{
           self.left = Some(Box::new(node));
           self
       }
       pub fn right(mut self, node: TreeNode<T>)->Self{
           self.right = Some(Box::new(node));
           self
       }
    }
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
    // concurency
    let thread1=thread::spawn(||{
        for i in 1..25{
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20{
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // joining the thread to guarantee that it will execute completely
    thread1.join().unwrap();

    pub struct Bank{
        balance: f32,
    }
    // fn withdraw(the_bank: &mut Bank, amt: f32){
    //     the_bank.balance -= amt
    // }

    // let mut bank = Bank{balance: 100.0};
    // withdraw(&mut bank, 5.00);
    // println!("balance: {}", bank.balance);

    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00);
    // }
    
    // causes error
    // thread::spawn(||{
    //     customer(&mut bank)
    // }).join().unwrap();
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref=the_bank.lock().unwrap();
        if bank_ref.balance < 5.00{
            println!("Low balance: {}", bank_ref.balance);
        }else{
            bank_ref.balance -= amt;
            println!("Withdrawal done. Balanace: {}", bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>){
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>>=Arc::new(Mutex::new(Bank{balance: 20.00}));
    // create threads
    let handles = (0..10).map(|_|{
        let bank_ref = bank.clone();
        return thread::spawn(||{
            customer(bank_ref);
        });
    });

    for handle in handles{
        handle.join().unwrap();
    }

    println!("balance: {}", bank.lock().unwrap().balance);
}
