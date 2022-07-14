use wasm_bindgen::prelude::*;

/// add two i32 numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

/// get string length
#[wasm_bindgen]
pub fn strlen(s: &str) -> usize {
  println!("{}", "haha");
  return s.len();
}

mod oneblock {
  mod solution1 {
    use super::super::*;

    #[derive(Debug)]
    enum TrafficLight {
      Red,
      Green,
      Yellow,
    }

    trait Light {
      fn duration(&self) -> i32;
    }

    impl Light for TrafficLight {
      fn duration(&self) -> i32 {
        return match self {
          TrafficLight::Red => 30,
          TrafficLight::Green => 5,
          TrafficLight::Yellow => 45,
        };
      }
    }

    fn display_light<T>(light: T) -> String
    where
      T: Light + std::fmt::Debug,
    {
      format!("{:?} light lasts for {}s", light, light.duration())
    }

    macro_rules! _display_light {
      ($( $light:expr),* $(,)?) => {{
        $( println!("{:?} light lasts for {}s", $light, $light.duration()); )*
      }};
    }

    /// show traffic lights
    #[wasm_bindgen]
    pub fn solution1() -> String {
      [
        display_light(TrafficLight::Red),
        display_light(TrafficLight::Green),
        display_light(TrafficLight::Yellow),
      ]
      .join("\n")
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(1, 2);
    assert_eq!(result, 3);
  }

  #[test]
  fn strlen_works() {
    let result = strlen("btwiuse/wasmbuild");
    assert_eq!(result, 17);
  }
}
