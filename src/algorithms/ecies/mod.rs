use crate::curve::{Point, Curve};
use crate::modulo::modulo;
use sha2::{Sha256, Digest};
use rand::Rng;

pub fn encrypt(message: &[u8], curve: Curve, pk: Point) -> (Point, Vec<u8>) {
  let ord = curve.order;

  // Select random nonce
  let mut rng = rand::thread_rng();
  let random_number: i128 = rng.gen();
  let k = modulo(random_number.abs(), ord);

  // Compute mask
  let mask = curve.multiply(&pk, k);
  let key = curve.multiply(&curve.gen, k);
  
  // Hash mask
  let mut hasher = Sha256::new();
  
  let mask_string = match mask {
    Point::Identity => panic!("Mask must be different than identity"),
    // This processing of the mask is not necessarily "good enough".
    // But it's just a way to get a hash.
    Point::Coords(x, y) => (x + y).to_string(),
  };

  hasher.update(mask_string);
  let mask_bytes = hasher.finalize();

  // Mask the original message with the generated bytes

  let message = message.to_vec();
  let mut mask_bytes = mask_bytes.to_vec();

  if message.len() < mask_bytes.len() {
    mask_bytes = mask_bytes[0..message.len()].to_vec()
  }

  let masked: Vec<u8> = message.to_vec()
    .iter()
    .zip(mask_bytes.iter())
    .map(|(&x1, &x2)| x1 ^ x2)
    .collect();

  (key.clone(), masked.to_vec())
}

pub fn decrypt(cyphertext: &[u8], curve: Curve, key: &Point, sk: i128) -> Vec<u8> {
  let mask = curve.multiply(key, sk);

  // Hash mask
  let mut hasher = Sha256::new();
  
  let mask_string = match mask {
    Point::Identity => panic!("Mask must be different than identity"),
    Point::Coords(x, y) => (x + y).to_string(),
  };

  hasher.update(mask_string);
  let mask_bytes = hasher.finalize();

  let cyphertext = cyphertext.to_vec();
  let mut mask_bytes = mask_bytes.to_vec();

  if cyphertext.len() < mask_bytes.len() {
    mask_bytes = mask_bytes[0..cyphertext.len()].to_vec()
  }

  let unmasked: Vec<u8> = cyphertext.to_vec()
    .iter()
    .zip(mask_bytes.iter())
    .map(|(&x1, &x2)| x1 ^ x2)
    .collect();

  unmasked.to_vec()
}