use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_one() {
        assert_eq!(add_one(3), 4);
    }
}
