#![allow(dead_code)]

use std::mem;
use std::io::stdin;

fn main() {
    slices()
}

fn stack_and_heap() {
    let stack_address = 32;
    let heap_address = Box::new(32);

    println!("the stack value is {} and the heap address is {}",
        mem::size_of_val(&stack_address) , mem::size_of_val(&heap_address) );
    println!("The address of heap_address is {:p} and the real value o heap_address is {}.", 
            heap_address,  *heap_address)    
}

enum State {
    Locked,
    Failed,
    Unlocked 
}

fn lock_unlock() {
    let code = String::from("12345");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;   
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                    continue;
                }
            }
            State::Failed => { 
                println!("FAILED!");
                entry.clear();
                state = State::Locked;
                continue
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return
            }
        }
    }
}

fn arrays() {
    let array_one = [1u64; 10];
    println!("All numbers in array_one {:?}", array_one);
    for i in 0..array_one.len() {
        println!("This is in for loop: Index = {} and Value = {}", i, array_one[i]);
    }
    println!("The ammount of memory in bytes, consumming by array_one = {}", mem::size_of_val(&array_one))
} 

fn slices() {
    let array_two = [10, 11, 12, 13, 14, 15];

    fn use_slice(slice: &[i32])
    {
        println!("Begin with {} and the length is {}, and the entiry array is {:?}", slice[0], slice.len(), slice);
    }

    use_slice(&array_two[2..6])
}