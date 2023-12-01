use wasm_bindgen::prelude::*;

/// add two u128 numbers
#[wasm_bindgen]
pub fn add(a: u128, b: u128) -> u128 {
  return a + b;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }
}
