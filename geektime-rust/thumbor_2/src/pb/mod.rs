mod abi;
pub use abi::*;
use base64::{Engine, engine::{self, general_purpose}, alphabet};
use prost::Message;
use photon_rs::transform::SamplingFilter;
use std::convert::TryFrom;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);


// 让ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let mut buf = String::new();
        let data = image_spec.encode_to_vec();
        CUSTOM_ENGINE.encode_string(data, &mut buf);
        buf
    }
}

// 让ImageSpec可以通过一个字符串创建
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = CUSTOM_ENGINE.decode(value).unwrap();
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photon_rs 相应的方法里需要字符串
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Marine => Some("marine"),
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
        }
    }
}

// 在我们定义的SampleFilter和photon_rs的SamplingFilter间转换
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::Undefined => SamplingFilter::Nearest,
            resize::SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
            resize::SampleFilter::Triangle => SamplingFilter::Triangle,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
        }
    }
}

// 提供一些辅助函数，让创建一个spec的过程简单一些
impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            })),
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            })),
        }
    }
    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;

    #[test]
    fn encode_spec_could_be_decode() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
