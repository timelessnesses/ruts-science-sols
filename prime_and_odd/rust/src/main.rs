use utils::input;

fn prime(n: isize) -> bool {
    if n <= 1 {
        return false;
    }
    for x in 2..n {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
fn stupid_timeless_prime(n: isize) -> bool {
    if n <= 1 {
        return false;
    }

    let mut a = vec![];
    for x in 1..n + 1 {
        if n % x == 0 {
            a.push(x);
        }
    }
    return a.len() == 2;
}

fn odd(n: isize) -> bool {
    return !(n % 2 == 0);
}

fn main() {
    #[allow(non_snake_case)]
    let N = input(Some("Enter number of times you want to check numbers: "))
        .parse::<usize>()
        .unwrap();
    let mut numbers = vec![];
    for x in 0..N {
        numbers.push(
            input(Some(&format!("Enter number {}: ", x + 1)))
                .parse::<isize>()
                .unwrap(),
        )
    }
    println!("Is Prime     |     Is Odd");
    for x in numbers {
        let p = prime(x);
        let o = odd(x);
        println!("{p}        |     {o}")
    }
}
