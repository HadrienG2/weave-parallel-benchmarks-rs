use rayon::prelude::*;

fn dfs(depth: usize, breadth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    // It always start with scheduling "breadth" tasks, as a parallel iterator
    // since that's the most natural Rayon tool for that job
    let res_iter = (0..breadth).into_par_iter()
                               .map(|_| dfs(depth-1, breadth));

    if cfg!(feature = "idiomatic") {
      // Idiomatic mode, do what a Rayon user normally would and sum in the same
      // Rayon transaction for optimal performance
      res_iter.sum()
    } else {
      // Non-idiomatic mode, strictly imitate the Weave version and collect the
      // data into a heap-allocated Vec, followed by a sequential sum
      let vec = res_iter.collect::<Vec<_>>();
      vec.into_iter().sum()
    }
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
