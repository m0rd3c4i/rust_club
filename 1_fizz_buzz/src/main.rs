use fizz_buzz::{
  error_end_bounds,
  error_int_args,
  fizz_buzz,
  print_usage_and_exit,
};

fn main() {
  let mut bounds: Vec<i32> = Vec::new();
  let mut start: i32 = 1;
  let mut end: i32 = 1;
  let mut quiet: bool = false;
  let quiet_flag: String = String::from("--quiet");

  // collect all args (skip the binary call)
  let mut args: Vec<String> = std::env::args().skip(1).collect();

  // check for the quiet flag and handle
  if args.contains(&quiet_flag) {
    quiet = true;
    args.retain(|arg| *arg != quiet_flag)
  }

  // ensure remaining args are valid
  if args.len() > 2 {
    print_usage_and_exit();
  }

  for arg in &args {
    match arg.parse::<i32>() {
      Ok(val) => {
        if val <= 0 {
          error_int_args();
        } else {
          bounds.push(val)
        }
      },
      Err(_) => {
        error_int_args();
      }
    }
  }

  // update actual bounds from processed args
  if bounds.len() > 0  {
    end = bounds.pop().unwrap()
  }

  if bounds.len() > 0 {
    start = bounds.pop().unwrap()
  }

  if end < start {
    error_end_bounds();
  }

  fizz_buzz(start, end, quiet);
}
