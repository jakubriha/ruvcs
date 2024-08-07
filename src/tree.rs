use std::{
    env,
    fmt::LowerHex,
    fs::{self, create_dir_all},
    io::{self, Read},
    path::{Path, PathBuf},
    str::FromStr,
};

use hex::FromHexError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{char, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

struct Key(GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>);

impl FromStr for Key {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let byte_array = hex::decode(&s)?;

        // TODO: Return error if byte array has wrong length
        Ok(Key(GenericArray::clone_from_slice(byte_array.as_slice())))
    }
}

impl LowerHex for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Key {
    fn generate_from_content(content: &[u8]) -> Self {
        Key(Sha256::digest(content))
    }
}

trait ObjectRepository {
    fn load(&self, key: &Key) -> io::Result<Vec<u8>>;

    fn save(&self, content: &[u8]) -> io::Result<Key>;
}

pub struct FileSystemObjectRepository {
    working_directory: PathBuf,
}

impl FileSystemObjectRepository {
    pub fn new() -> io::Result<Self> {
        let working_directory = env::current_dir()?;

        Ok(FileSystemObjectRepository {
            working_directory,
        })
    }
}

impl ObjectRepository for FileSystemObjectRepository {
    fn load(&self, key: &Key) -> io::Result<Vec<u8>> {
        let key_as_string = format!("{:x}", key);

        let key_prefix = &key_as_string[0..2];
        let key_suffix = &key_as_string[2..];

        let mut path = self.working_directory.join(key_prefix);

        path.push(key_suffix);

        fs::read(path)
    }

    fn save(&self, object_content: &[u8]) -> io::Result<Key> {
        let key = Key::generate_from_content(object_content);

        let key_as_string = format!("{:x}", key);

        let key_prefix = &key_as_string[0..2];
        let key_suffix = &key_as_string[2..];

        let mut path = self.working_directory.join(key_prefix);

        create_dir_all(&path)?;

        path.push(key_suffix);

        fs::write(path, object_content)?;

        Ok(key)
    }
}

// fn save_object_given_file(path: impl AsRef<Path>) -> io::Result<Key> {
//     let mut file = fs::File::open(path)?;

//     let mut buffer = Vec::new();

//     file.read_to_end(&mut buffer)?;

//     save_object(buffer.as_slice())
// }

enum Mode {
    Blob,
    Tree,
}

struct TreeEntry {
    mode: Mode,
    key: Key,
    name: String,
}

impl TreeEntry {
    pub fn write(writer: &mut impl io::Write, input: &Self) -> io::Result<()> {
        let serialized_mode = match input.mode {
            Mode::Blob => "blob",
            Mode::Tree => "tree",
        };

        write!(
            writer,
            "{} {:x}\t{}",
            serialized_mode, input.key, input.name
        )?;

        Ok(())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let pars = tuple((
            Self::mode_parser,
            char(' '),
            Self::key_parser,
            char('\t'),
            Self::name_parser,
        ));

        // TODO: I think there is no need to initialize parser every time.
        let mut r = map(pars, |(mode, _, key, _, name)| Self {
            mode,
            key,
            name: name.to_string(),
        });

        r(input)
    }

    fn mode_parser(input: &str) -> IResult<&str, Mode> {
        let parser = alt((tag("blob"), tag("tree")));

        map(parser, |parsed_string| match parsed_string {
            "blob" => Mode::Blob,
            "tree" => Mode::Tree,
            _ => panic!(),
        })(input)
    }

    fn key_parser(input: &str) -> IResult<&str, Key> {
        let parser = take(64usize);

        map(parser, |parsed_string| {
            Key::from_str(parsed_string).unwrap()
        })(input)
    }

    fn name_parser(input: &str) -> IResult<&str, &str> {
        take_until("\n")(input)
    }
}

pub struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn write(writer: &mut impl io::Write, input: &Self) -> io::Result<()> {
        for entry in input.entries.iter() {
            TreeEntry::write(writer, entry)?;

            write!(writer, "\n")?;
        }

        Ok(())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parser = separated_list1(line_ending, TreeEntry::parse);

        let mut r = map(parser, |entries| Self { entries });

        r(input)
    }

    pub fn from_reader(reader: &mut impl Read) -> Self {
        let mut buffer = String::default();

        let _ = reader.read_to_string(&mut buffer);

        let k = Self::parse(&buffer).unwrap();

        k.1
    }
}
