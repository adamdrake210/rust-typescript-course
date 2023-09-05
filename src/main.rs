fn multiple(num: Option<usize>) -> Option<usize> {
  let num = num?;
  // return num.unwrap_or(0) * 5;
  if num > 0 {
    return Some(num * 5)
  } else {
    return None
  }
}

fn main() {
  let num = multiple(Some(40));

  println!("{}", num.unwrap())
}