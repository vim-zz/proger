use anyhow::Result;
use proger_core::protocol::model::PageModel;
use proger_core::protocol::request::{DeleteStepsPage, NewStepsPage, SetStepsPage};
use rand::{distributions::Alphanumeric, Rng};
use sha3::{Digest, Sha3_256};
use thiserror::Error;
use tokio::runtime::Runtime;

const PASSWORD_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 30;

/// The create session message
#[derive(Debug)]
pub enum StorageCmd {
    CreateStepsPage(NewStepsPage),
    UpdateStepsPage(String, SetStepsPage),
    DeleteStepsPage(String, DeleteStepsPage),
    GetStepsPage(String),
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Wrong password when trying to update page")]
    WrongPassword,
    #[error("The link `{0}` does not exists")]
    LinkNotExists(String),
    #[error("The page descriptor schema doesn't match")]
    CorruptedItem,
}

/// Trait to allow different database backend
pub trait StorageDriver: 'static + Unpin {
    fn connect(&self) -> Result<()>;
    fn execute(&self, rt: &mut Runtime, cmd: StorageCmd) -> Result<PageModel>;
}

pub fn generate_link() -> String {
    let link: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(60)
        .collect();
    link
}

pub fn generate_secret() -> String {
    let mut rng = rand::thread_rng();
    let secret: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, PASSWORD_CHARSET.len());
            PASSWORD_CHARSET[idx] as char
        })
        .collect();
    secret
}

pub fn hash_secret(secret: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(secret.as_bytes());
    let mut hash = "".to_string();
    for byte in hasher.result() {
        hash.push_str(&format!("{:x}", byte));
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link() {
        let link1 = generate_link();
        let link2 = generate_link();
        println!("link1: {}", link1);
        println!("link2: {}", link2);
        assert_ne!(link1, link2);
    }

    #[test]
    fn secret() {
        let secret1 = generate_secret();
        let secret2 = generate_secret();
        println!("secret1: {}", secret1);
        println!("secret2: {}", secret2);
        assert_ne!(secret1, secret2);
    }

    #[test]
    fn hash() {
        let secret = generate_secret();
        let hash1 = hash_secret(&secret);
        let hash2 = hash_secret(&secret);
        println!("hash1: {}", hash1);
        println!("hash2: {}", hash2);
        assert_eq!(hash1, hash2);
    }
}
