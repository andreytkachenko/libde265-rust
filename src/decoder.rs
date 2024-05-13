use std::{any::Any, ffi::c_int, sync::Arc};

use libde265_sys::{de265_acceleration, de265_param};

use crate::{error::Error, image::Image, De265};

pub struct Decoder {
    inner: *mut libde265_sys::de265_decoder_context,
    _sess: Arc<De265>,
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { libde265_sys::de265_free_decoder(self.inner) };
    }
}

impl Decoder {
    pub fn new(_sess: Arc<De265>) -> Self {
        Self {
            inner: unsafe { libde265_sys::de265_new_decoder() },
            _sess,
        }
    }

    pub fn start_worker_threads(&mut self, number_of_threads: u32) -> Result<(), Error> {
        Error::from_res(unsafe {
            libde265_sys::de265_start_worker_threads(self.inner, number_of_threads as _)
        })
    }

    pub fn push_data(
        &mut self,
        data: &[u8],
        pts: i64,
        user_data: Option<Box<dyn Any>>,
    ) -> Result<(), Error> {
        Error::from_res(unsafe {
            libde265_sys::de265_push_data(
                self.inner,
                data.as_ptr() as *const _,
                data.len() as _,
                pts,
                user_data
                    .map(Box::into_raw)
                    .map(|x| x as *mut _)
                    .unwrap_or(std::ptr::null_mut()),
            )
        })
    }

    pub fn push_end_of_nal(&mut self) {
        unsafe { libde265_sys::de265_push_end_of_NAL(self.inner) }
    }

    pub fn push_end_of_frame(&mut self) {
        unsafe { libde265_sys::de265_push_end_of_frame(self.inner) }
    }

    pub fn push_nal(
        &mut self,
        data: &[u8],
        pts: i64,
        user_data: Option<Box<dyn Any>>,
    ) -> Result<(), Error> {
        Error::from_res(unsafe {
            libde265_sys::de265_push_NAL(
                self.inner,
                data.as_ptr() as *const _,
                data.len() as _,
                pts,
                user_data
                    .map(Box::into_raw)
                    .map(|x| x as *mut _)
                    .unwrap_or(std::ptr::null_mut()),
            )
        })
    }

    pub fn flush_data(&mut self) -> Result<(), Error> {
        Error::from_res(unsafe { libde265_sys::de265_flush_data(self.inner) })
    }

    pub fn get_number_of_input_bytes_pending(&self) -> i32 {
        unsafe { libde265_sys::de265_get_number_of_input_bytes_pending(self.inner) }
    }

    pub fn get_number_of_nal_units_pending(&self) -> i32 {
        unsafe { libde265_sys::de265_get_number_of_NAL_units_pending(self.inner) }
    }

    pub fn decode(&mut self) -> Result<(), Error> {
        let mut more: i32 = 0;
        let res = Error::from_res(unsafe { libde265_sys::de265_decode(self.inner, &mut more) });

        match res {
            Ok(_) | Err(Error::WaitingForInputData) => Ok(()),
            Err(other) => Err(other),
        }
    }

    pub fn reset(&mut self) {
        unsafe { libde265_sys::de265_reset(self.inner) }
    }

    pub fn peek_next_picture(&mut self) -> Option<Image> {
        let inner = unsafe { libde265_sys::de265_peek_next_picture(self.inner) };

        if inner.is_null() {
            None
        } else {
            Some(Image { inner })
        }
    }

    pub fn get_next_picture(&mut self) -> Option<Image> {
        let inner = unsafe { libde265_sys::de265_get_next_picture(self.inner) };

        if inner.is_null() {
            None
        } else {
            Some(Image { inner })
        }
    }

    pub fn release_next_picture(&mut self) {
        unsafe { libde265_sys::de265_release_next_picture(self.inner) }
    }

    pub fn get_warning(&self) -> Result<(), Error> {
        Error::from_res(unsafe { libde265_sys::de265_get_warning(self.inner) })
    }

    pub fn get_highest_tid(&mut self) -> i32 {
        unsafe { libde265_sys::de265_get_highest_TID(self.inner) }
    }

    pub fn get_current_tid(&mut self) -> i32 {
        unsafe { libde265_sys::de265_get_current_TID(self.inner) }
    }

    pub fn set_limit_tid(&mut self, max_tid: i32) {
        unsafe { libde265_sys::de265_set_limit_TID(self.inner, max_tid) }
    }

    pub fn set_framerate_ratio(&mut self, percent: i32) {
        unsafe { libde265_sys::de265_set_framerate_ratio(self.inner, percent) }
    }

    pub fn change_framerate(&mut self, more_vs_less: i32) -> i32 {
        unsafe { libde265_sys::de265_change_framerate(self.inner, more_vs_less) }
    }

    #[inline]
    pub fn set_check_sei_hash(&mut self, check_hash: bool) {
        self.set_parameter_bool(
            de265_param::DE265_DECODER_PARAM_BOOL_SEI_CHECK_HASH,
            check_hash,
        )
    }

    #[inline]
    pub fn is_sei_hash_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_BOOL_SEI_CHECK_HASH)
    }

    #[inline]
    pub fn set_dump_sps_headers(&mut self, dump_sps: bool) {
        self.set_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_SPS_HEADERS, dump_sps);
    }

    #[inline]
    pub fn is_dump_sps_headers_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_SPS_HEADERS)
    }

    #[inline]
    pub fn set_dump_pps_headers(&mut self, dump_pps: bool) {
        self.set_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_PPS_HEADERS, dump_pps)
    }

    #[inline]
    pub fn is_dump_pps_headers_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_PPS_HEADERS)
    }

    #[inline]
    pub fn set_dump_vps_headers(&mut self, dump_vps: bool) {
        self.set_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_VPS_HEADERS, dump_vps)
    }

    #[inline]
    pub fn is_dump_vps_headers_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_VPS_HEADERS)
    }

    #[inline]
    pub fn set_dump_slice_headers(&mut self, dump_slices: bool) {
        self.set_parameter_bool(
            de265_param::DE265_DECODER_PARAM_DUMP_SLICE_HEADERS,
            dump_slices,
        )
    }

    #[inline]
    pub fn is_dump_slice_headers_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DUMP_SLICE_HEADERS)
    }

    #[inline]
    pub fn set_acceleration_type(&mut self, accel: AccelerationType) {
        let accel: de265_acceleration::Type = accel.into();
        self.set_parameter_int(
            de265_param::DE265_DECODER_PARAM_ACCELERATION_CODE,
            accel as _,
        )
    }

    #[inline]
    pub fn set_suppress_faulty_pictures(&mut self, suppress: bool) {
        self.set_parameter_bool(
            de265_param::DE265_DECODER_PARAM_SUPPRESS_FAULTY_PICTURES,
            suppress,
        )
    }

    #[inline]
    pub fn is_suppress_faulty_pictures_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_SUPPRESS_FAULTY_PICTURES)
    }

    #[inline]
    pub fn set_disable_deblocking(&mut self, disable_deblocking: bool) {
        self.set_parameter_bool(
            de265_param::DE265_DECODER_PARAM_DISABLE_DEBLOCKING,
            disable_deblocking,
        )
    }

    #[inline]
    pub fn is_deblocking_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DISABLE_DEBLOCKING)
    }

    #[inline]
    pub fn set_disable_sao(&mut self, disable_sao: bool) {
        self.set_parameter_bool(de265_param::DE265_DECODER_PARAM_DISABLE_SAO, disable_sao)
    }

    #[inline]
    pub fn is_sao_enabled(&self) -> bool {
        self.get_parameter_bool(de265_param::DE265_DECODER_PARAM_DISABLE_SAO)
    }

    fn get_parameter_bool(&self, decoder_param: de265_param::Type) -> bool {
        !matches!(
            unsafe { libde265_sys::de265_get_parameter_bool(self.inner, decoder_param) },
            0
        )
    }

    fn set_parameter_bool(&mut self, decoder_param: de265_param::Type, value: bool) {
        unsafe {
            libde265_sys::de265_set_parameter_bool(
                self.inner,
                decoder_param,
                if value { 1 } else { 0 },
            )
        };
    }

    fn set_parameter_int(&mut self, decoder_param: de265_param::Type, value: c_int) {
        unsafe { libde265_sys::de265_set_parameter_int(self.inner, decoder_param, value) };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AccelerationType {
    Auto,
    Scalar,
    Mmx,
    Sse,
    Sse2,
    Sse4,
    Avx,
    Avx2,
    Arm,
    Neon,
}

impl From<AccelerationType> for de265_acceleration::Type {
    fn from(val: AccelerationType) -> Self {
        match val {
            AccelerationType::Auto => de265_acceleration::de265_acceleration_AUTO,
            AccelerationType::Scalar => de265_acceleration::de265_acceleration_SCALAR,
            AccelerationType::Mmx => de265_acceleration::de265_acceleration_MMX,
            AccelerationType::Sse => de265_acceleration::de265_acceleration_SSE,
            AccelerationType::Sse2 => de265_acceleration::de265_acceleration_SSE2,
            AccelerationType::Sse4 => de265_acceleration::de265_acceleration_SSE4,
            AccelerationType::Avx => de265_acceleration::de265_acceleration_AVX,
            AccelerationType::Avx2 => de265_acceleration::de265_acceleration_AVX2,
            AccelerationType::Arm => de265_acceleration::de265_acceleration_ARM,
            AccelerationType::Neon => de265_acceleration::de265_acceleration_NEON,
        }
    }
}
