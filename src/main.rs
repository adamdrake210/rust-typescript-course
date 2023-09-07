// fn multiply(nums: Vec<usize>, index: usize) -> usize {
//   return nums.get(index).unwrap_or(&index) * 5;
// }

// fn main() {

//   let vec = vec![1, 2, 4, 5, 6];

//   println!("{}", multiply(vec, 0))

// }

// Rust Error Libraries
// thiserror
// anyhow - for applications

fn main() {
  let file_name = std::env::args().nth(1)
      .expect("the file name to be passed in");

  let file = std::fs::read_to_string(file_name)
      .expect("unable to read file contents");

  file.lines().for_each(|line| println!("{}", line));
  
}