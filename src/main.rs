use std::{fs::{self, create_dir_all}, io::{Read, self}, path::Path};

use sha2::{Sha256, Digest};

fn get_hash(content: &[u8]) -> String {

    let mut hasher = Sha256::new();

    hasher.update(content);

    let hash = hasher.finalize();

    format!("{:x}", hash)
}

fn save_object(object_content: &[u8]) -> io::Result<String> {

    let object_hash = get_hash(&object_content);

    let hash_prefix = &object_hash[0..2];
    let hash_suffix = &object_hash[2..];

    let mut path = Path::new(r"E:\Users\jakub\Downloads\test\.vcs\objects")
        .join(hash_prefix);

    create_dir_all(&path)?;

    path.push(hash_suffix);
    
    fs::write(path, &object_content)?;

    Ok(object_hash)
}

fn save_object_given_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    
    let mut file = fs::File::open(path)?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    save_object(buffer.as_slice())
}

fn main() {

    let hex_hash = save_object_given_file(r"E:\Users\jakub\Downloads\test.txt").expect("File cannot be read");
    
    println!("{}", hex_hash);
}
