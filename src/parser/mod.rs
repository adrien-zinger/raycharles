use std::fs;
use std::vec::Vec;

pub fn read(input: &str) -> Vec::<Vec::<i8>> {
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut ret = Vec::<Vec::<i8>>::new();
    for &line in lines.iter() {
      if line == "end" { break; }
      let mut vecline = Vec::<i8>::new();
      let el: Vec<&str> = line.split(" ").collect();
      for &v in el.iter() {
        vecline.push(i8::from_str_radix(v, 10).unwrap());
      }
      ret.push(vecline);
    }
    return ret;
}
