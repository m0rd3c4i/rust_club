use fizz_buzz::fizz_buzz;

fn main() {
  let mut start: i32 = 1;
  let mut end: i32 = 1;

  let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 {
      start = 1;
      end = args.remove(0).parse::<i32>().unwrap();
    } else if args.len() == 2 {
      start = args.remove(0).parse::<i32>().unwrap();
      end = args.remove(0).parse::<i32>().unwrap();
    } else {
      print_usage_and_exit();
    }

  fizz_buzz(start, end);
}

fn print_usage_and_exit() {
  println!("FizzBuzz: so simple, a third-grader might have nightmares about it");
  println!("USAGE: provided one or two integer inputs, it does the thing *inclusively*");
  std::process::exit(-1);
}
