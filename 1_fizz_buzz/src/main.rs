use fizz_buzz::{
  error_end_bounds,
  error_int_args,
  fizz_buzz,
  print_usage_and_exit,
};

fn main() {
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

  // ensure remaining arguments are valid
  if args.len() == 1 {
    match args.remove(0).parse::<i32>() {
      Ok(val) => {
        end = val;
      },
      Err(_) => {
        error_int_args();
      }
    }
  } else if args.len() == 2 {
    match args.remove(0).parse::<i32>() {
      Ok(val) => {
        start = val;
      },
      Err(_) => {
        error_int_args();
      }
    }
    match args.remove(0).parse::<i32>() {
      Ok(val) => {
        end = val;
      },
      Err(_) => {
        error_int_args();
      }
    }
  } else {
    print_usage_and_exit();
  }

  if start <= 0 || end <= 0 {
    error_int_args();
  }

  if end < start {
    error_end_bounds();
  }

  fizz_buzz(start, end, quiet);
}
