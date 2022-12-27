use std::i8;

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
    println!("Zulfikra {:?} ", a);
  
}


//enum
#[derive(Debug)]
enum Type {
    A(i8)
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
