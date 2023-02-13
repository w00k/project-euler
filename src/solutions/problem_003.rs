/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?`
 */

const MAX_NUM: i64 = 600851475143;

pub fn problem003() {
    println!(" MAX is {}", MAX_NUM);
    get_prime(MAX_NUM);
}

fn get_prime(max_num: i64) -> i64 {
    let mut div = i64::from(2);
    let two = i64::from(2);
    let mut vec = Vec::new();

    while div < max_num/two {
        if max_num % div == 0 {
            vec.push(div);
        }

        div = index(div);
    }

    println!("All the factors are {:?}", vec);

    for num in vec.iter().rev() {
        let index = *num;
        let is_prime_num = is_prime(index);
        if is_prime_num == true {
            return index;
        }
    }

    return div;
}

fn is_prime(num: i64) -> bool {

    let mut div = i64::from(2);

    while div < num {
        if num % div == 0 {
            return false;
        }

        div = index(div);
    }

    println!("number {} is prime", num);
    return true;
}

fn index(num: i64) -> i64 {
    if num != 2 {
        return num + 2;
    }
    return num + 1;
}
