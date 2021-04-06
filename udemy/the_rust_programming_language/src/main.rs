use std::mem;

fn main() {
    stack_and_heap()
}

fn stack_and_heap() {
    let stack_address = 32;
    let heap_address = Box::new(32);

    println!("the stack value is {} and the heap address is {}",
        mem::size_of_val(&stack_address) , mem::size_of_val(&heap_address) );
    println!("The address of heap_address is {:p} and the real value o heap_address is {}.", 
            heap_address,  *heap_address)    
}
