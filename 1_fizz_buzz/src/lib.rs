pub fn fizz_buzz(start: i32, end: i32, quiet: bool) {
  for i in start..=end {
    /*
     * "PRETTY NAIVE" APPROACH
     * idk, just do the thing, maybe?
     * good: multiples of 3 are the most frequent
     */
    if i % 3 == 0 {
      if i % 5 == 0 {
        println!("FizzBuzz");
      } else {
        println!("Fizz");
      }
    } else if i % 5 == 0 {
      println!("Buzz");
    } else if !quiet {
      println!("{}", i);
    }

    // /*
    //  * "CLEANER CODE" APPROACH
    //  * we know what 3 * 5 is, so...
    //  * bad: multiples of 15 are the most infrequent
    //  */
    // if i % 15 == 0 {
    //   println!("FizzBuzz");
    // } else if i % 3 == 0 {
    //   println!("Fizz");
    // } else if i % 5 == 0 {
    //   println!("Buzz");
    // } else {
    //   println!("{}", i);
    // }

    // /*
    //  * "OVERARCHITECTED" APPROACH
    //  * maybe the two predicates can play nicely with each other
    //  * bad: ...I never wanted this
    //  */
    // let mut results: Vec<&str> = Vec::new();
    // if i % 3 == 0 {
    //   results.push("Fizz");
    // }
    // if i % 5 == 0 {
    //   results.push("Buzz");
    // }
    // if results.len() > 0 {
    //   println!("{}", results.join(""));
    // } else {
    //   println!("{}", i);
    // }
  }
}


pub fn error_int_args() {
  println!("Arguments must be integers greater than 0");
  std::process::exit(-1);
}

pub fn error_end_bounds() {
  println!("Second argument must be greater than the first argument when both are provided");
  std::process::exit(-1);
}

pub fn print_usage_and_exit() {
  println!("FizzBuzz: so simple, a third-grader might have nightmares about it");
  println!("USAGE: provided one or two integer inputs, it does the thing *inclusively*");
  std::process::exit(-1);
}
