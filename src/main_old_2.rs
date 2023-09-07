#[derive(Debug)]
struct Custom {
  age: usize,
  name: String
}

#[derive(Debug)]
enum Item {
  Number(usize),
  String(String),
  MyCustom(Custom)
} 

fn append(items: &mut Vec<Item>) {
  items.push(Item::String("hello, fem".to_string()));
  items.push(Item::Number(32));

}

fn main() {
  let mut items: Vec<Item> = vec![];

  append(&mut items);

  items.iter().enumerate().for_each(|(idx, line)| {
    match line {
      Item::String(line) => println!("{} - Woo, string!", line),
      Item::Number(line) => println!("{} - Woo, number!", line),
      _ => {}
    }
  });


}