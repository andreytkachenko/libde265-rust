mod decoder;
#[cfg(encoder)]
mod encoder;
mod error;
mod image;

use std::{ffi::CStr, sync::Arc};

pub use decoder::Decoder;

#[cfg(encoder)]
pub use encoder::Encoder;
pub use error::Error;

#[derive(Debug, Clone, Copy)]
pub enum ChromaFormat {
    Mono,
    Chroma420,
    Chroma422,
    Chroma444,
}

impl TryFrom<libde265_sys::de265_chroma::Type> for ChromaFormat {
    type Error = ();

    fn try_from(value: libde265_sys::de265_chroma::Type) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChromaFormat::Mono),
            1 => Ok(ChromaFormat::Chroma420),
            2 => Ok(ChromaFormat::Chroma422),
            3 => Ok(ChromaFormat::Chroma444),
            _ => Err(()),
        }
    }
}

pub enum ImageFormat {
    Mono8,
    Yuv420P8,
    Yuv422P8,
    Yuv444P8,
}

impl TryFrom<libde265_sys::de265_image_format::Type> for ImageFormat {
    type Error = ();

    fn try_from(value: libde265_sys::de265_image_format::Type) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ImageFormat::Mono8),
            2 => Ok(ImageFormat::Yuv420P8),
            3 => Ok(ImageFormat::Yuv422P8),
            4 => Ok(ImageFormat::Yuv444P8),
            _ => Err(()),
        }
    }
}

// pub const de265_param_DE265_DECODER_PARAM_BOOL_SEI_CHECK_HASH: de265_param = 0;
// pub const de265_param_DE265_DECODER_PARAM_DUMP_SPS_HEADERS: de265_param = 1;
// pub const de265_param_DE265_DECODER_PARAM_DUMP_VPS_HEADERS: de265_param = 2;
// pub const de265_param_DE265_DECODER_PARAM_DUMP_PPS_HEADERS: de265_param = 3;
// pub const de265_param_DE265_DECODER_PARAM_DUMP_SLICE_HEADERS: de265_param = 4;
// pub const de265_param_DE265_DECODER_PARAM_ACCELERATION_CODE: de265_param = 5;
// pub const de265_param_DE265_DECODER_PARAM_SUPPRESS_FAULTY_PICTURES: de265_param = 6;
// pub const de265_param_DE265_DECODER_PARAM_DISABLE_DEBLOCKING: de265_param = 7;
// pub const de265_param_DE265_DECODER_PARAM_DISABLE_SAO: de265_param = 8;
// pub type de265_param = ::std::os::raw::c_uint;
// pub const de265_acceleration_de265_acceleration_SCALAR: de265_acceleration = 0;
// pub const de265_acceleration_de265_acceleration_MMX: de265_acceleration = 10;
// pub const de265_acceleration_de265_acceleration_SSE: de265_acceleration = 20;
// pub const de265_acceleration_de265_acceleration_SSE2: de265_acceleration = 30;
// pub const de265_acceleration_de265_acceleration_SSE4: de265_acceleration = 40;
// pub const de265_acceleration_de265_acceleration_AVX: de265_acceleration = 50;
// pub const de265_acceleration_de265_acceleration_AVX2: de265_acceleration = 60;
// pub const de265_acceleration_de265_acceleration_ARM: de265_acceleration = 70;
// pub const de265_acceleration_de265_acceleration_NEON: de265_acceleration = 80;
// pub const de265_acceleration_de265_acceleration_AUTO: de265_acceleration = 10000;
// pub type de265_acceleration = ::std::os::raw::c_uint;

// pub fn de265_set_parameter_bool(
//     arg1: *mut de265_decoder_context,
//     param: de265_param,
//     value: ::std::os::raw::c_int,
// );
// pub fn de265_set_parameter_int(
//     arg1: *mut de265_decoder_context,
//     param: de265_param,
//     value: ::std::os::raw::c_int,
// );

// pub fn de265_get_parameter_bool(
//     arg1: *mut de265_decoder_context,
//     param: de265_param,
// ) -> ::std::os::raw::c_int;

pub struct De265;

impl Drop for De265 {
    fn drop(&mut self) {
        unsafe { libde265_sys::de265_free() };
    }
}

impl De265 {
    pub fn new() -> Result<Arc<Self>, Error> {
        Error::from_res(unsafe { libde265_sys::de265_init() })?;

        Ok(Arc::new(Self))
    }

    pub fn get_version(&self) -> &'static str {
        let version = unsafe { CStr::from_ptr(libde265_sys::de265_get_version()) };

        version.to_str().unwrap()
    }

    pub fn de265_get_version_number(&self) -> u32 {
        unsafe { libde265_sys::de265_get_version_number() }
    }

    pub fn de265_get_version_number_major(&self) -> i32 {
        unsafe { libde265_sys::de265_get_version_number_major() }
    }

    pub fn de265_get_version_number_minor(&self) -> i32 {
        unsafe { libde265_sys::de265_get_version_number_minor() }
    }

    pub fn de265_get_version_number_maintenance(&self) -> i32 {
        unsafe { libde265_sys::de265_get_version_number_maintenance() }
    }

    pub fn disable_logging(&self) {
        unsafe { libde265_sys::de265_disable_logging() };
    }

    pub fn de265_set_verbosity(&self, level: i32) {
        unsafe { libde265_sys::de265_set_verbosity(level) };
    }
}
