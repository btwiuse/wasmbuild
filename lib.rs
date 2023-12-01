use wasm_bindgen::prelude::*;

/// add two u32 numbers
#[wasm_bindgen]
pub fn add(a: String, b: String) -> String {
  let a_num = a.parse::<u32>().unwrap();
  let b_num = b.parse::<u32>().unwrap();
  (a_num + b_num).to_string()
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
