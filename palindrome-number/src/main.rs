fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut num = x;
    let mut reversed: i32 = 0;

    while num > 0 {
        let digit = num % 10;
        reversed = reversed.checked_mul(10).and_then(|r| r.checked_add(digit)).unwrap_or(0);
        num /= 10;
    }

    reversed == x
}

fn main() {
    let test_cases = [121, -121, 12321, 123, 0];

    for &num in &test_cases {
        if is_palindrome(num) {
            println!("{} é um palíndromo", num);
        } else {
            println!("{} não é um palíndromo", num);
        }
    }
}
