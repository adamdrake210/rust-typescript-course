enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}

impl Color {
  fn is_green(&self) -> bool {
    if let Color::Green = self {
      return true;
    }
    return false;
  }

  fn is_green_parts(&self) -> bool {
    match self {
      Color::Blue => return true,
      Color::Green => return false,
      Color::Red => return false,
      Color::Yellow => return true
    }

  }
}

fn print_color(color: Color) {
  // match color {
  //   Color::Red => println!("red"),
  //   Color::Blue => println!("blue"),
  //   Color::Green => println!("green"),
  //   Color::Yellow => println!("yellow")
  // }

  let foo = Color::Blue;

  let is_it_green = foo.is_green();

  println!("{}", is_it_green);
}

fn main() {

    // let data = vec![1, 2, 3];

    // let mut list = data.iter().map(|x| x + 1);

    // // Iterator is another kind of data structure that can
    // // iterate over a collection

    // let mut new_vector = vec![];

    // while let Some(x) = list.next() {
    //   new_vector.push(x);
    // }


    // let foo: HashMap<&str, usize> = vec!["this", "is", "super", "cool"]
    // .into_iter()
    // .enumerate()
    // .map(|(idx, item)| (item, idx))
    // .collect();



    // println!("{:?}", foo);

    // let file = std::fs::read_to_string("lines").unwrap();

    // file.lines()
    //     .enumerate()
    //     .filter(|(idx, _)|  idx % 2 == 0)
    //     .skip(2)
    //     .take(2)
    //     .for_each(|(_, line)| println!("{}", line));

    print_color(Color::Green)
    

}
