use std::convert::identity;
use std::i8;
use std::collections::HashMap;
fn main() {
    // variable
    let unsigned: u32 = 10;
    let float_variable: f32 = 10.5;
    let char_variable = "char variable";
    let mut bool_char: bool = true;
    bool_char = false;
    println!("Unsigned variable {}", unsigned);
    println!("Float variable {}", float_variable);
    println!("Char and String variable {}", char_variable);
    println!("Boolean variable {}", bool_char);

    //  Array
    let arr: [u8; 3] = [1, 2, 3];
    let arr2: [u8; 3] = [100; 3];
    let arr3: [i32; 5] = [1, 3, 5, 7, 9];
    println!("Array 1 {:?}", arr);
    println!("Array 1 length {}", arr.len());
    println!("Array 1 index 0 {}", arr[0]);

    println!("Array 2 {:?}", arr2);

    //  tuples
    let tup: (i32, bool, &str) = (1, true, "zulfikra");
    println!("tuples {:?}", tup);
    println!("tuple 1 {}", tup.0);

    println!("value of is even {} ", is_even(1));
    // Function

    println!("{}", call_name("Zulfikra !!"));

    // Array slice
    let _result = &arr3[1..3];
    println!("Result array slice {:?}", _result);

    // String
    let my_name: &str = "My name is ";
    let slice_name = &my_name[0..5];
    println!("{}", slice_name);
    // For Loop
    for i in 0..10 {
        println!("call my name in {}", i);
    }
    // While loop
    let mut iteration = 0;
    while iteration < 5 {
        println!("Call me in iteration {}", iteration);
        if iteration == 4 {
            println!("Ayay the while loop is stop right now ");
            break;
        }
        iteration += 1;
    }

    // Match
    let my_name2 = "lahmudin";

    match my_name2 {
        "zulfikra" => println!("Hello zulfikra"),
        "lahmudin" => {
            println!("You are not zulfikra, you are lahmudin")
        }
        _ => {
            println!("You are nothing")
        }
    }

    //  Struct
    let cat = Cat {
        name: String::from("Maxiwiliam"),
        _type: String::from("Maincone"),
        ages: 8,
    };

    cat.introduce();
    // trait || interface
    println!("{}", cat.can_scratch());

    // Enum
    let a: Type = Type::A(2);
    let b: Type = Type::zulfikra;

    println!("Zulfikra {:?} ", a);
    println!("My name is {:?}", b);
    // print!("Hello {}", let Type::A(val))
    if let Type::A(val) = a {
        println!("hello {}", val)
    }

    // Vectr || Dynamic Array
    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.push(20);
    println!("This is vector dynamic array {:?}", vec);
    println!("Length of vector {:?} ", vec.len());

    // Hashing map
    let mut map = HashMap::new();
        map.insert("nama", "Zulfikra");
        // map.insert("nama", "Lahmudin");
        map.insert("umur", "27");
        
    match map.get(&"nama") {
        Some(str) => {
            println!("Hello {} this is hash map", str)
        },
        None => {
            println!("Oppss, there is not data here {}","");
        }
    }
    map.remove(&"nama");
    println!("Show all map  {:?}", map);
    println!("Map first value {:?}", map.get("nama"));

    // Option
    match  devide(10, 2) {
        Some(val) => println!(" Devide result {}", val),
        None => println!("Nothing has printed") 
    }

    let deviden_value = devide(10, 3);
    println!("Deviden result {:?}", deviden_value);
    println!("Deviden result {:?}", deviden_value.unwrap());

    // Result
    // almost same with option but it returns Ok and error

    let identity: Result<&str, &str> = identified("mantap");
    match identity {
        Ok(val) => println!("You are {}", val),
        Err(val) => println!("{}", val)
    }

    if identity.is_ok() {
        println!("Yess you are my {} ", identity.unwrap())
    }else {
        println!("Upss you are not my zulfikra");
    }

    // identity.unwrap();
    identity.expect("Upss expect happen");
}


fn devide(devidend: i32, divisor: i32) -> Option<i32> {
    if devidend % divisor  == 0 {
        None
    }else {
        Some(devidend / divisor)
    }
}


fn identified(name: &str) -> Result<&str, &str> {
    if name == "zulfikra" {
        Ok("Zulfikra")
    }else {
        Err("You are not Zulfikra")
    }
}

//enum
#[derive(Debug)]
enum Type {
    A(i8),
    zulfikra
}

struct Cat {
    name: String,
    _type: String,
    ages: u8,
}

impl Cat {
    fn introduce(&self) {
        println!("Hello my name is {} and my age is {}", self.name, self.ages);
    }
}

trait Animal {
    fn can_scratch(&self) -> bool;
}


impl Animal for Cat {
    fn can_scratch(&self) -> bool {
        true
    }

}



fn is_even(val: i8) -> bool {
    if val % 2 == 0 {
        return true;
    }

    false
}

fn call_name(val: &str) -> String {
    format!("{} {}", "hello ", val)
}
