use wasm_bindgen::prelude::*;

/// add two u128 numbers
#[wasm_bindgen]
pub fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
  let mut a_slice = [0u8; 16];
  let mut b_slice = [0u8; 16];
  a_slice.copy_from_slice(a);
  b_slice.copy_from_slice(b);
  let a_num = u128::from_le_bytes(a_slice);
  let b_num = u128::from_le_bytes(b_slice);
  (a_num + b_num).to_le_bytes().to_vec()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let a = 1u128.to_le_bytes();
    let b = 2u128.to_le_bytes();
    let result = add(&a, &b);
    assert_eq!(result, 3u128.to_le_bytes().to_vec());
  }
}
