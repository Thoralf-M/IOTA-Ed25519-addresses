use bs58;
use byteorder::{ByteOrder, LittleEndian};
use ed25519_dalek::PublicKey;
use multihash::Blake2b256;
use rand_core::{OsRng, RngCore};

pub struct Wallet {
  seed: Vec<u8>,
}

impl Wallet {
  pub fn new() -> Wallet {
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    Wallet {
      seed: seed.to_vec(),
    }
  }
  pub fn from_base58(seed: &str) -> Wallet {
    let decoded = bs58::decode(seed).into_vec().unwrap();
    Wallet { seed: decoded }
  }
  pub fn sub_seed(&self, index: u64) -> Vec<u8> {
    let mut index_bytes = [0; 8];
    LittleEndian::write_u64(&mut index_bytes, index);
    let res = Blake2b256::digest(&index_bytes);
    // first 4 bytes aren't needed
    let blakehash = &res[4..];
    // XOR
    let sub_seed: Vec<u8> = self
      .seed
      .iter()
      .zip(blakehash.iter())
      .map(|(&x1, &x2)| x1 ^ x2)
      .collect();
    sub_seed
  }
  pub fn address(&self, index: usize) -> String {
    let subseed = self.sub_seed(index as u64);
    let private_key = ed25519_dalek::SecretKey::from_bytes(&subseed).unwrap();
    let public_key: PublicKey = PublicKey::from(&private_key);
    let address_bytes = Blake2b256::digest(&public_key.to_bytes());
    bs58::encode(&[&[1], &address_bytes[4..]].concat()).into_string()
  }
  pub fn get_base58_seed(&self) -> String {
    bs58::encode(&self.seed).into_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn correct_addresses() {
    let wallet = Wallet::from_base58("Gnp4f3nn8RFgEMpjsr58DEWKQHTfpGP5wvic5e8aeSBp");
    assert_eq!(
      wallet.address(0),
      "QbmLnx2zrQQJ1U5JyR54qrzWgQvqiD4wNjoRKYqCjo8q"
    );
    assert_eq!(
      wallet.address(1),
      "LbNeQyMtf2HF1D6oQWabsrd6wPX1CUhgacz8htoN6vJs"
    );
  }
}
