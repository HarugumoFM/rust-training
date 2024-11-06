fn in_char_vec(char_vec: &Vec<char>, check:char) {
    println!("is {check} inside? {}", char_vec.contains(&check));
}


fn main() {
    let some_numbers = vec![9, 6, 9, 10, 11];

    println!("{}", some_numbers.iter().fold(0, |total, next| total+next));
}