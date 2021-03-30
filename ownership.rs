fn is_odd(n: u32) -> bool {
    let numero = n % 2 == 1;
    return numero;
}

fn main() {
    let check_number = 5;
    let teste = is_odd(check_number);
    println!("{}", teste);
}

