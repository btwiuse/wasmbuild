/// add two u64 numbers
#[no_mangle]
pub fn add(a: u64, b: u64) -> u64 {
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
