fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let (x, y) =
        if cfg!(feature = "idiomatic") {
            // Idiomatic mode, use optimized Rayon path for the common-case
            // scenario of binary fork-join parallelization
            rayon::join(|| fib(n-1), || fib(n-2))
        } else {
            // Non-idiomatic mode, strictly imitate the Weave version by
            // explicitly spawning one task. This will be quite slow as "scope"
            // has not received as much optimization love as "join"...
            let (mut x, mut y) = (0, 0);
            rayon::scope(|s| {
                s.spawn(|_| x = fib(n - 1));
                y = fib(n - 2);
            });
            (x, y)
        };
    x + y
}

fn main() {
    let mut args = std::env::args().skip(1);

    let n = args.next()
                .map(|s| s.parse().expect("Expected fibonacci number"))
                .unwrap_or(40);

    println!("fib({}) = {}", n, fib(n));
}
