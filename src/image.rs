use std::any::Any;

use crate::ChromaFormat;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct de265_image_spec {
//     pub format: de265_image_format,
//     pub width: ::std::os::raw::c_int,
//     pub height: ::std::os::raw::c_int,
//     pub alignment: ::std::os::raw::c_int,
//     pub crop_left: ::std::os::raw::c_int,
//     pub crop_right: ::std::os::raw::c_int,
//     pub crop_top: ::std::os::raw::c_int,
//     pub croibp_bottom: ::std::os::raw::c_int,
//     pub visible_width: ::std::os::raw::c_int,
//     pub visible_height: ::std::os::raw::c_int,
// }

pub struct Image {
    pub(crate) inner: *const libde265_sys::de265_image,
}

impl Image {
    #[inline]
    pub fn get_image_width(&self, channel: i32) -> u32 {
        unsafe { libde265_sys::de265_get_image_width(self.inner, channel) as u32 }
    }

    #[inline]
    pub fn get_image_height(&self, channel: i32) -> u32 {
        unsafe { libde265_sys::de265_get_image_height(self.inner, channel) as u32 }
    }

    #[inline]
    pub fn get_chroma_format(&self) -> ChromaFormat {
        ChromaFormat::try_from(unsafe { libde265_sys::de265_get_chroma_format(self.inner) })
            .unwrap()
    }

    #[inline]
    pub fn get_bits_per_pixel(&self, channel: i32) -> i32 {
        unsafe { libde265_sys::de265_get_bits_per_pixel(self.inner, channel) }
    }

    #[inline]
    pub fn get_image_plane(&self, channel: i32) -> (&[u8], usize) {
        let mut stride = 0i32;
        let x = unsafe { libde265_sys::de265_get_image_plane(self.inner, channel, &mut stride) };

        let size = self.get_image_width(channel) * self.get_image_height(channel);

        (
            unsafe { std::slice::from_raw_parts(x, size as usize) },
            stride as usize,
        )
    }

    #[inline]
    pub fn get_image_plane_user_data(&self, channel: i32) -> Option<&dyn Any> {
        let user_data =
            unsafe { libde265_sys::de265_get_image_plane_user_data(self.inner, channel) };

        if user_data.is_null() {
            None
        } else {
            Some(unsafe { &mut *user_data })
        }
    }

    #[inline]
    pub fn get_image_plane_user_data_mut(&mut self, channel: i32) -> Option<&mut dyn Any> {
        let user_data =
            unsafe { libde265_sys::de265_get_image_plane_user_data(self.inner, channel) };

        if user_data.is_null() {
            None
        } else {
            Some(unsafe { &mut *user_data })
        }
    }

    #[inline]
    pub fn get_image_user_data(&self) -> Option<&dyn Any> {
        let user_data = unsafe { libde265_sys::de265_get_image_user_data(self.inner) };

        if user_data.is_null() {
            None
        } else {
            Some(unsafe { &*user_data })
        }
    }

    #[inline]
    pub fn get_image_user_data_mut(&mut self) -> Option<&mut dyn Any> {
        let user_data = unsafe { libde265_sys::de265_get_image_user_data(self.inner) };

        if user_data.is_null() {
            None
        } else {
            Some(unsafe { &mut *user_data })
        }
    }

    // #[inline]
    // pub fn set_image_user_data(&mut self, user_data: *mut ::std::os::raw::c_void) {
    //     let user_data = unsafe { libde265_sys::de265_set_image_user_data(self.inner, user_data) };
    // }

    // #[inline]
    // pub fn set_image_plane(
    //     &mut self,
    //     chroma_idx: ::std::os::raw::c_int,
    //     mem: *mut ::std::os::raw::c_void,
    //     stride: ::std::os::raw::c_int,
    //     userdata: *mut ::std::os::raw::c_void,
    // ) {
    //     let user_data = unsafe { libde265_sys::de265_set_image_plane(self.inner, user_data) };
    // }

    #[inline]
    pub fn get_image_pts(&self) -> i64 {
        unsafe { libde265_sys::de265_get_image_PTS(self.inner) }
    }

    #[inline]
    pub fn get_image_nal_header(&self) {
        let mut nal_unit_type: i32 = 0;
        let mut nal_unit_name: *const i8 = std::ptr::null_mut();
        let mut nuh_layer_id: i32 = 0;
        let mut nuh_temporal_id: i32 = 0;

        unsafe {
            libde265_sys::de265_get_image_NAL_header(
                self.inner,
                &mut nal_unit_type,
                &mut nal_unit_name,
                &mut nuh_layer_id,
                &mut nuh_temporal_id,
            )
        };
    }

    #[inline]
    pub fn get_image_full_range_flag(&self) -> i32 {
        unsafe { libde265_sys::de265_get_image_full_range_flag(self.inner) }
    }

    #[inline]
    pub fn get_image_colour_primaries(&self) -> i32 {
        unsafe { libde265_sys::de265_get_image_colour_primaries(self.inner) }
    }

    #[inline]
    pub fn get_image_transfer_characteristics(&self) -> i32 {
        unsafe { libde265_sys::de265_get_image_transfer_characteristics(self.inner) }
    }

    #[inline]
    pub fn get_image_matrix_coefficients(&self) -> i32 {
        unsafe { libde265_sys::de265_get_image_matrix_coefficients(self.inner) }
    }
}
