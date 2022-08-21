use std::{fs::{self, create_dir_all}, io::{Read, BufReader, self}, path::Path};

use sha2::{Sha256, Digest};

fn get_hash(content: &[u8]) -> String {

    let mut hasher = Sha256::new();

    hasher.update(content);

    let hash = hasher.finalize();

    format!("{:x}", hash)
}

fn hash_object(object_content: &[u8]) -> io::Result<String> {

    let object_hash = get_hash(&object_content);

    let path = Path::new(r"E:\Users\jakub\Downloads");

    let mut path = path.join(r"test\.vcs\objects\");

    create_dir_all(path.clone())?;

    path.push(object_hash.clone());
    
    fs::write(path, &object_content)?;

    Ok(object_hash)
}

fn hash_object_from_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    
    let file = fs::File::open(path)?;

    let mut reader = BufReader::new(file);
    
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    hash_object(buffer.as_slice())
}

fn main() {

    let hex_hash = hash_object_from_file(r"E:\Users\jakub\Downloads\test.txt").expect("File cannot be read");
    
    println!("{}", hex_hash);
}
