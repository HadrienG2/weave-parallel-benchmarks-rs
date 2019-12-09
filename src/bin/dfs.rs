use rayon::prelude::*;

fn dfs(depth: usize, breadth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let sums = (0..breadth).into_par_iter()
                           .map(|_| dfs(depth-1, breadth))
                           .collect::<Vec<_>>();

    sums.into_iter().sum()
}

fn main() {
    let mut args = std::env::args().skip(1);

    let depth = args.next()
                    .map(|s| s.parse().expect("Expected recursion depth"))
                    .unwrap_or(8);

    let breadth = args.next()
                      .map(|s| s.parse().expect("Expected recursion breadth"))
                      .unwrap_or(8);

    println!("dfs(depth={}, breadth={}) = {}", depth, breadth, dfs(depth, breadth));
}
