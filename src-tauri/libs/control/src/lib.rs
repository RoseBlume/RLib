use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::read_to_string;
// pub fn writer(filename: &str, content: &[u8]) {
//     let mut file = File::create(filename);
//     file.write_all(content);
// }

// pub fn opener(filename: &str) {
//     let mut file = File::open(filename);
//     let mut contents = String::new();
//     File::read_to_string(&mut File::open(filename), &mut contents);
//     println!("{}", contents);
// }

// pub fn opener(filename: &str) ->String {
//     //let mut f = File::open(filename)?;
//     let path = Path::new(filename);
//     let display = path.display();
//     let mut file = File::open(&path); //{
//         //Err(why) => panic!("couldn't open {}: {}", display, why),
//     //};
//
//     // Read the file contents into a string, returns `io::Result<usize>`
//     let mut s = String::new();
//     file.read_to_string(&mut s) //{
//       //  Err(why) => panic!("couldn't read {}: {}", display, why),
//     //}
// }

// pub fn opener(filename: &str) -> Vec<String> {
//     let mut result = Vec::new();
//
//     for line in read_to_string(filename).unwrap().lines() {
//         result.push(line.to_string());
//         println!("{}", line.to_string());
//     }
//
//     result
// }
// pub fn opener(filename: &str) -> String {
//     let maker = read_to_string(filename);
//     let made = maker.to_string();
//     println!("{}", made);
// }
pub fn opener(filename: &str) -> Vec<String> {
    read_to_string(filename)
    .unwrap()  // panic on possible file-reading errors
    .lines()  // split the string into an iterator of string slices
    .map(String::from)  // make each slice into a string
    .collect()  // gather them together into a vector
}
pub fn writer(filename: &str, content: &str){
    let mut data_file = File::create(filename).expect("creation failed");

    // Write contents to the file
    data_file.write(content.as_bytes()).expect("write failed");
}
