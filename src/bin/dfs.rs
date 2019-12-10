use rayon::prelude::*;

fn dfs(depth: usize, breadth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    if cfg!(feature = "idiomatic") {
      // Idiomatic mode, do what a Rayon user normally would and carry out all
      // the work in a single map-reduce transaction for optimal performance
      (0..breadth).into_par_iter()
                  .map(|_| dfs(depth-1, breadth))
                  .sum()
    } else {
      // Non-idiomatic mode, strictly imitate the Weave version by...
      //
      // 1. Preallocating a vector of outputs instead of collecting an iterator
      // 2. Explicitly spawning one task for each of the "breadth" iterations
      // 3. Performing a serial sum at the end
      //
      // You can also experiment with ligher shades of non-idiomatic code, by
      // e.g. using a parallel Rayon iterator instead of an explicit loop, but
      // still collecting the outputs in a vector and ending with a serial sum.
      //
      let mut vec = vec![0; breadth];

      rayon::scope(|s|{
        for target in vec.iter_mut() {
          s.spawn(move |_| *target = dfs(depth-1, breadth));
        }
      });

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
