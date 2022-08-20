use std::{fs, io, path::Path};

use sha1::{Sha1, Digest};

fn get_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    
    let mut file = fs::File::open(path)?;

    let mut hasher = Sha1::new();

    io::copy(&mut file, &mut hasher)?;

    let hash = hasher.finalize();

    Ok(format!("{:x}", hash))
}

fn main() {

    let hex_hash = get_hash(r"E:\Users\jakub\Downloads\bottom.png").expect("File cannot be read");
    
    println!("{}", hex_hash);





}
