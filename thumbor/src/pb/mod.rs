use base64::engine::general_purpose::URL_SAFE_NO_PAD;

mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self{specs}
    }
}

impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
    }
}