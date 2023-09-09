#[derive(Debug)]
struct Item {
  count: usize
}


fn print_all(items: &Vec<Item>) {
  for item in items {
    println!("{:?}", item);
  }
}

fn main() {
  let mut items = vec![Item { count: 1}];
  // let first = items.first_mut();
  // println!("{:?}", first);

  // print_all(&items);

  // This is a mutable reference
  let first = items.get_mut(0);
  // This is a mutable reference
  let second = items.get_mut(2);
  println!("{:?}", second);

}

// Rust rules
// - There can only be one value owner
// - There can be unlimited immutable borrows (reference) with no mutable references
// - There can be only one mutable reference and no immutable references