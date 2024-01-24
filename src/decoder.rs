use std::{any::Any, sync::Arc};

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
}
