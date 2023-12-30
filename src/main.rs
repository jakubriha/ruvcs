mod tree;

use std::fs;

use crate::tree::*;

fn main() {
    let mut file = fs::File::open(r"E:\Users\jakub\Downloads\tree.txt").unwrap();

    let tree = Tree::from_reader(&mut file);


    let mut file = fs::File::create(r"E:\Users\jakub\Downloads\tree.written.txt").unwrap();

    Tree::write(&mut file, &tree).unwrap();

    // let hex_hash =
    //     save_object_given_file(r"E:\Users\jakub\Downloads\test.txt").expect("File cannot be read");

    // println!("{:x}", hex_hash);
}
