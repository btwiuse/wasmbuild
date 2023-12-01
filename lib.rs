use wasm_bindgen::prelude::*;

/// add two u32 numbers
#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
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
