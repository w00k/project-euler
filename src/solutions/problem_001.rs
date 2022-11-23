/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
 */


pub fn problem001() {

    let mut number_vector: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for index in 1..1000 {
        if is_multiple(index, 3) || is_multiple(index, 5) {
            number_vector.push(index); 
        }
    }

    for number in number_vector {
        sum += number;
    }
    
    println!("The sum of all number equals to {}", sum);
}

fn is_multiple(multiple: i32, div: i32) -> bool {
    if multiple % div == 0 {
        return true;
    }
    return false;
}
