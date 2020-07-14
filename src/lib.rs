use bs58;
use byteorder::{ByteOrder, LittleEndian};
use ed25519_dalek::PublicKey;
use multihash::Blake2b256;

pub struct Wallet {
  seed: Vec<u8>,
}

impl Wallet {
  //base58 String
  pub fn new(seed: &str) -> Wallet {
    // println!("{:?}", seed.to_string().into_bytes());
    let decoded = bs58::decode(seed).into_vec().unwrap();
    // println!("{:?}", decoded);
    Wallet { seed: decoded }
  }
  pub fn address(&self, index: usize) -> String {
    let subseed = self.sub_seed(index as u64);
    let private_key = ed25519_dalek::SecretKey::from_bytes(&subseed).unwrap();
    // println!("Private Key:{:?}", private_key);
    // let public_key: PublicKey = (private_key).into();
    let public_key: PublicKey = PublicKey::from(&private_key);
    // println!("Public Key:{:?}", public_key);

    let address = Blake2b256::digest(&public_key.to_bytes());
    // println!("addressblake {:?}", &address[4..]);
    let pubkeybase58 = bs58::encode(&[&[1], &address[4..]].concat()).into_string();
    // println!("pubkeybase58{:?}", pubkeybase58);
    // println!("Index: {}", index);
    pubkeybase58
  }
  pub fn sub_seed(&self, index: u64) -> Vec<u8> {
    let mut index_bytes = [0; 8];
    LittleEndian::write_u64(&mut index_bytes, index);
    let res = Blake2b256::digest(&index_bytes);
    // println!("hashOfIndexBytes {:?}", &res[4..]);
    // first 4 bytes aren't needed
    let blakehash = &res[4..];
    // XOR
    let sub_seed: Vec<u8> = self
      .seed
      .iter()
      .zip(blakehash.iter())
      .map(|(&x1, &x2)| x1 ^ x2)
      .collect();
    // println!("sub_seed{:?}", sub_seed);
    sub_seed
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn correct_addresses() {
    let wallet = Wallet::new("Gnp4f3nn8RFgEMpjsr58DEWKQHTfpGP5wvic5e8aeSBp");
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
