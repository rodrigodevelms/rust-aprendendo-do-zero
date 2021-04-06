fn main() {

    println!("The result of adding number one, two, three and four is: {}", calculating());
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
}

fn add(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn calculating() -> i32 {
    let number_one = 10;
    let number_two: i32 = 20;
    let number_three = 5i32;
    let number_four = 9_i32;
    
    add(add(number_one, number_two), add(number_three, number_four))
}
