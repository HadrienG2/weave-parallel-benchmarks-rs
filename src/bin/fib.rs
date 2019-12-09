fn fib(n: usize) -> usize {
    if n < 2 {
        n
    } else {
        let (i, j) = rayon::join(|| fib(n-1), || fib(n-2));
        i + j
    }
}

fn main() {
    let mut args = std::env::args().skip(1);

    let n = args.next()
                .map(|s| s.parse().expect("Expected fibonacci number"))
                .unwrap_or(40);

    println!("fib({}) = {}", n, fib(n));
}
