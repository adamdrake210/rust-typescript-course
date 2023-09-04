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

    let file = std::fs::read_to_string("lines").unwrap();

    file.lines()
        .enumerate()
        .filter(|(idx, _)|  idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));

      
}
