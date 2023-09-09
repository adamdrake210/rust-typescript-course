#[derive(Debug)]
struct Item {
  count: usize
}

fn add_one(item: &mut Item) {
  item.count += 1;
}

fn main() {
  let mut item = Item { count: 1 };
  println!("{:?}", item);
  add_one(&mut item);
  println!("{:?}", item);
}

// Questions
// - Who owns the value?
// - How long does this value live?
// There can only be one owner. There can only be one lord of the ring