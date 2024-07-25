fn main() {
    let num1 = 4001889888977788995;
    let num2 = 3999208000077778889;

    let lcm_result = lcm(num1, num2);

    println!("The LCM of {} and {} is {}", num1, num2, lcm_result);
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd(a, b)
}