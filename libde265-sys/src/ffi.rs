/* automatically generated by rust-bindgen 0.69.2 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
extern "C" {
    pub fn de265_get_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn de265_get_version_number() -> u32;
}
extern "C" {
    pub fn de265_get_version_number_major() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_version_number_minor() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_version_number_maintenance() -> ::std::os::raw::c_int;
}
pub mod de265_error {
    pub type Type = ::std::os::raw::c_uint;
    pub const DE265_OK: Type = 0;
    pub const DE265_ERROR_NO_SUCH_FILE: Type = 1;
    pub const DE265_ERROR_COEFFICIENT_OUT_OF_IMAGE_BOUNDS: Type = 4;
    pub const DE265_ERROR_CHECKSUM_MISMATCH: Type = 5;
    pub const DE265_ERROR_CTB_OUTSIDE_IMAGE_AREA: Type = 6;
    pub const DE265_ERROR_OUT_OF_MEMORY: Type = 7;
    pub const DE265_ERROR_CODED_PARAMETER_OUT_OF_RANGE: Type = 8;
    pub const DE265_ERROR_IMAGE_BUFFER_FULL: Type = 9;
    pub const DE265_ERROR_CANNOT_START_THREADPOOL: Type = 10;
    pub const DE265_ERROR_LIBRARY_INITIALIZATION_FAILED: Type = 11;
    pub const DE265_ERROR_LIBRARY_NOT_INITIALIZED: Type = 12;
    pub const DE265_ERROR_WAITING_FOR_INPUT_DATA: Type = 13;
    pub const DE265_ERROR_CANNOT_PROCESS_SEI: Type = 14;
    pub const DE265_ERROR_PARAMETER_PARSING: Type = 15;
    pub const DE265_ERROR_NO_INITIAL_SLICE_HEADER: Type = 16;
    pub const DE265_ERROR_PREMATURE_END_OF_SLICE: Type = 17;
    pub const DE265_ERROR_UNSPECIFIED_DECODING_ERROR: Type = 18;
    pub const DE265_ERROR_NOT_IMPLEMENTED_YET: Type = 502;
    pub const DE265_WARNING_NO_WPP_CANNOT_USE_MULTITHREADING: Type = 1000;
    pub const DE265_WARNING_WARNING_BUFFER_FULL: Type = 1001;
    pub const DE265_WARNING_PREMATURE_END_OF_SLICE_SEGMENT: Type = 1002;
    pub const DE265_WARNING_INCORRECT_ENTRY_POINT_OFFSET: Type = 1003;
    pub const DE265_WARNING_CTB_OUTSIDE_IMAGE_AREA: Type = 1004;
    pub const DE265_WARNING_SPS_HEADER_INVALID: Type = 1005;
    pub const DE265_WARNING_PPS_HEADER_INVALID: Type = 1006;
    pub const DE265_WARNING_SLICEHEADER_INVALID: Type = 1007;
    pub const DE265_WARNING_INCORRECT_MOTION_VECTOR_SCALING: Type = 1008;
    pub const DE265_WARNING_NONEXISTING_PPS_REFERENCED: Type = 1009;
    pub const DE265_WARNING_NONEXISTING_SPS_REFERENCED: Type = 1010;
    pub const DE265_WARNING_BOTH_PREDFLAGS_ZERO: Type = 1011;
    pub const DE265_WARNING_NONEXISTING_REFERENCE_PICTURE_ACCESSED: Type = 1012;
    pub const DE265_WARNING_NUMMVP_NOT_EQUAL_TO_NUMMVQ: Type = 1013;
    pub const DE265_WARNING_NUMBER_OF_SHORT_TERM_REF_PIC_SETS_OUT_OF_RANGE: Type = 1014;
    pub const DE265_WARNING_SHORT_TERM_REF_PIC_SET_OUT_OF_RANGE: Type = 1015;
    pub const DE265_WARNING_FAULTY_REFERENCE_PICTURE_LIST: Type = 1016;
    pub const DE265_WARNING_EOSS_BIT_NOT_SET: Type = 1017;
    pub const DE265_WARNING_MAX_NUM_REF_PICS_EXCEEDED: Type = 1018;
    pub const DE265_WARNING_INVALID_CHROMA_FORMAT: Type = 1019;
    pub const DE265_WARNING_SLICE_SEGMENT_ADDRESS_INVALID: Type = 1020;
    pub const DE265_WARNING_DEPENDENT_SLICE_WITH_ADDRESS_ZERO: Type = 1021;
    pub const DE265_WARNING_NUMBER_OF_THREADS_LIMITED_TO_MAXIMUM: Type = 1022;
    pub const DE265_NON_EXISTING_LT_REFERENCE_CANDIDATE_IN_SLICE_HEADER: Type = 1023;
    pub const DE265_WARNING_CANNOT_APPLY_SAO_OUT_OF_MEMORY: Type = 1024;
    pub const DE265_WARNING_SPS_MISSING_CANNOT_DECODE_SEI: Type = 1025;
    pub const DE265_WARNING_COLLOCATED_MOTION_VECTOR_OUTSIDE_IMAGE_AREA: Type = 1026;
    pub const DE265_WARNING_PCM_BITDEPTH_TOO_LARGE: Type = 1027;
    pub const DE265_WARNING_REFERENCE_IMAGE_BIT_DEPTH_DOES_NOT_MATCH: Type = 1028;
    pub const DE265_WARNING_REFERENCE_IMAGE_SIZE_DOES_NOT_MATCH_SPS: Type = 1029;
    pub const DE265_WARNING_CHROMA_OF_CURRENT_IMAGE_DOES_NOT_MATCH_SPS: Type = 1030;
    pub const DE265_WARNING_BIT_DEPTH_OF_CURRENT_IMAGE_DOES_NOT_MATCH_SPS: Type = 1031;
    pub const DE265_WARNING_REFERENCE_IMAGE_CHROMA_FORMAT_DOES_NOT_MATCH: Type = 1032;
    pub const DE265_WARNING_INVALID_SLICE_HEADER_INDEX_ACCESS: Type = 1033;
}
extern "C" {
    pub fn de265_get_error_text(err: de265_error::Type) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn de265_isOK(err: de265_error::Type) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_disable_logging();
}
extern "C" {
    pub fn de265_set_verbosity(level: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct de265_image {
    _unused: [u8; 0],
}
pub mod de265_chroma {
    pub type Type = ::std::os::raw::c_uint;
    pub const de265_chroma_mono: Type = 0;
    pub const de265_chroma_420: Type = 1;
    pub const de265_chroma_422: Type = 2;
    pub const de265_chroma_444: Type = 3;
}
pub type de265_PTS = i64;
extern "C" {
    pub fn de265_get_image_width(
        arg1: *const de265_image,
        channel: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_image_height(
        arg1: *const de265_image,
        channel: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_chroma_format(arg1: *const de265_image) -> de265_chroma::Type;
}
extern "C" {
    pub fn de265_get_bits_per_pixel(
        arg1: *const de265_image,
        channel: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_image_plane(
        arg1: *const de265_image,
        channel: ::std::os::raw::c_int,
        out_stride: *mut ::std::os::raw::c_int,
    ) -> *const u8;
}
extern "C" {
    pub fn de265_get_image_plane_user_data(
        arg1: *const de265_image,
        channel: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn de265_get_image_PTS(arg1: *const de265_image) -> de265_PTS;
}
extern "C" {
    pub fn de265_get_image_user_data(arg1: *const de265_image) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn de265_set_image_user_data(
        arg1: *mut de265_image,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn de265_get_image_NAL_header(
        arg1: *const de265_image,
        nal_unit_type: *mut ::std::os::raw::c_int,
        nal_unit_name: *mut *const ::std::os::raw::c_char,
        nuh_layer_id: *mut ::std::os::raw::c_int,
        nuh_temporal_id: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn de265_get_image_full_range_flag(arg1: *const de265_image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_image_colour_primaries(arg1: *const de265_image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_image_transfer_characteristics(
        arg1: *const de265_image,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_image_matrix_coefficients(arg1: *const de265_image) -> ::std::os::raw::c_int;
}
pub type de265_decoder_context = ::std::os::raw::c_void;
extern "C" {
    pub fn de265_new_decoder() -> *mut de265_decoder_context;
}
extern "C" {
    pub fn de265_start_worker_threads(
        arg1: *mut de265_decoder_context,
        number_of_threads: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn de265_free_decoder(arg1: *mut de265_decoder_context) -> de265_error::Type;
}
extern "C" {
    pub fn de265_decode_data(
        arg1: *mut de265_decoder_context,
        data: *const ::std::os::raw::c_void,
        length: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn de265_push_data(
        arg1: *mut de265_decoder_context,
        data: *const ::std::os::raw::c_void,
        length: ::std::os::raw::c_int,
        pts: de265_PTS,
        user_data: *mut ::std::os::raw::c_void,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn de265_push_end_of_NAL(arg1: *mut de265_decoder_context);
}
extern "C" {
    pub fn de265_push_end_of_frame(arg1: *mut de265_decoder_context);
}
extern "C" {
    pub fn de265_push_NAL(
        arg1: *mut de265_decoder_context,
        data: *const ::std::os::raw::c_void,
        length: ::std::os::raw::c_int,
        pts: de265_PTS,
        user_data: *mut ::std::os::raw::c_void,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn de265_flush_data(arg1: *mut de265_decoder_context) -> de265_error::Type;
}
extern "C" {
    pub fn de265_get_number_of_input_bytes_pending(
        arg1: *mut de265_decoder_context,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_number_of_NAL_units_pending(
        arg1: *mut de265_decoder_context,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_decode(
        arg1: *mut de265_decoder_context,
        more: *mut ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn de265_reset(arg1: *mut de265_decoder_context);
}
extern "C" {
    pub fn de265_peek_next_picture(arg1: *mut de265_decoder_context) -> *const de265_image;
}
extern "C" {
    pub fn de265_get_next_picture(arg1: *mut de265_decoder_context) -> *const de265_image;
}
extern "C" {
    pub fn de265_release_next_picture(arg1: *mut de265_decoder_context);
}
extern "C" {
    pub fn de265_get_warning(arg1: *mut de265_decoder_context) -> de265_error::Type;
}
pub mod de265_image_format {
    pub type Type = ::std::os::raw::c_uint;
    pub const de265_image_format_mono8: Type = 1;
    pub const de265_image_format_YUV420P8: Type = 2;
    pub const de265_image_format_YUV422P8: Type = 3;
    pub const de265_image_format_YUV444P8: Type = 4;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct de265_image_spec {
    pub format: de265_image_format::Type,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub alignment: ::std::os::raw::c_int,
    pub crop_left: ::std::os::raw::c_int,
    pub crop_right: ::std::os::raw::c_int,
    pub crop_top: ::std::os::raw::c_int,
    pub crop_bottom: ::std::os::raw::c_int,
    pub visible_width: ::std::os::raw::c_int,
    pub visible_height: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct de265_image_allocation {
    pub get_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut de265_decoder_context,
            spec: *mut de265_image_spec,
            img: *mut de265_image,
            userdata: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub release_buffer: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut de265_decoder_context,
            img: *mut de265_image,
            userdata: *mut ::std::os::raw::c_void,
        ),
    >,
}
extern "C" {
    pub fn de265_set_image_allocation_functions(
        arg1: *mut de265_decoder_context,
        arg2: *mut de265_image_allocation,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn de265_get_default_image_allocation_functions() -> *const de265_image_allocation;
}
extern "C" {
    pub fn de265_set_image_plane(
        img: *mut de265_image,
        cIdx: ::std::os::raw::c_int,
        mem: *mut ::std::os::raw::c_void,
        stride: ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn de265_get_highest_TID(arg1: *mut de265_decoder_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_get_current_TID(arg1: *mut de265_decoder_context) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_set_limit_TID(arg1: *mut de265_decoder_context, max_tid: ::std::os::raw::c_int);
}
extern "C" {
    pub fn de265_set_framerate_ratio(
        arg1: *mut de265_decoder_context,
        percent: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn de265_change_framerate(
        arg1: *mut de265_decoder_context,
        more_vs_less: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub mod de265_param {
    pub type Type = ::std::os::raw::c_uint;
    pub const DE265_DECODER_PARAM_BOOL_SEI_CHECK_HASH: Type = 0;
    pub const DE265_DECODER_PARAM_DUMP_SPS_HEADERS: Type = 1;
    pub const DE265_DECODER_PARAM_DUMP_VPS_HEADERS: Type = 2;
    pub const DE265_DECODER_PARAM_DUMP_PPS_HEADERS: Type = 3;
    pub const DE265_DECODER_PARAM_DUMP_SLICE_HEADERS: Type = 4;
    pub const DE265_DECODER_PARAM_ACCELERATION_CODE: Type = 5;
    pub const DE265_DECODER_PARAM_SUPPRESS_FAULTY_PICTURES: Type = 6;
    pub const DE265_DECODER_PARAM_DISABLE_DEBLOCKING: Type = 7;
    pub const DE265_DECODER_PARAM_DISABLE_SAO: Type = 8;
}
pub mod de265_acceleration {
    pub type Type = ::std::os::raw::c_uint;
    pub const de265_acceleration_SCALAR: Type = 0;
    pub const de265_acceleration_MMX: Type = 10;
    pub const de265_acceleration_SSE: Type = 20;
    pub const de265_acceleration_SSE2: Type = 30;
    pub const de265_acceleration_SSE4: Type = 40;
    pub const de265_acceleration_AVX: Type = 50;
    pub const de265_acceleration_AVX2: Type = 60;
    pub const de265_acceleration_ARM: Type = 70;
    pub const de265_acceleration_NEON: Type = 80;
    pub const de265_acceleration_AUTO: Type = 10000;
}
extern "C" {
    pub fn de265_set_parameter_bool(
        arg1: *mut de265_decoder_context,
        param: de265_param::Type,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn de265_set_parameter_int(
        arg1: *mut de265_decoder_context,
        param: de265_param::Type,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn de265_get_parameter_bool(
        arg1: *mut de265_decoder_context,
        param: de265_param::Type,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn de265_init() -> de265_error::Type;
}
extern "C" {
    pub fn de265_free() -> de265_error::Type;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct en265_encoder_context {
    _unused: [u8; 0],
}
extern "C" {
    pub fn en265_new_encoder() -> *mut en265_encoder_context;
}
extern "C" {
    pub fn en265_free_encoder(arg1: *mut en265_encoder_context) -> de265_error::Type;
}
extern "C" {
    pub fn en265_set_parameter_bool(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_set_parameter_int(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_set_parameter_string(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_set_parameter_choice(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_list_parameters(
        arg1: *mut en265_encoder_context,
    ) -> *mut *const ::std::os::raw::c_char;
}
pub mod en265_parameter_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const en265_parameter_bool: Type = 0;
    pub const en265_parameter_int: Type = 1;
    pub const en265_parameter_string: Type = 2;
    pub const en265_parameter_choice: Type = 3;
}
extern "C" {
    pub fn en265_get_parameter_type(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
    ) -> en265_parameter_type::Type;
}
extern "C" {
    pub fn en265_list_parameter_choices(
        arg1: *mut en265_encoder_context,
        parametername: *const ::std::os::raw::c_char,
    ) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn en265_parse_command_line_parameters(
        arg1: *mut en265_encoder_context,
        argc: *mut ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_show_parameters(arg1: *mut en265_encoder_context);
}
extern "C" {
    pub fn en265_start_encoder(
        arg1: *mut en265_encoder_context,
        number_of_threads: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_allocate_image(
        arg1: *mut en265_encoder_context,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        chroma: de265_chroma::Type,
        pts: de265_PTS,
        image_userdata: *mut ::std::os::raw::c_void,
    ) -> *mut de265_image;
}
extern "C" {
    pub fn de265_alloc_image_plane(
        img: *mut de265_image,
        cIdx: ::std::os::raw::c_int,
        inputdata: *mut ::std::os::raw::c_void,
        inputstride: ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn de265_free_image_plane(img: *mut de265_image, cIdx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn en265_get_image_spec(
        arg1: *mut en265_encoder_context,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        chroma: de265_chroma::Type,
        out_spec: *mut de265_image_spec,
    );
}
extern "C" {
    pub fn en265_push_image(
        arg1: *mut en265_encoder_context,
        arg2: *mut de265_image,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_push_eof(arg1: *mut en265_encoder_context) -> de265_error::Type;
}
extern "C" {
    pub fn en265_block_on_input_queue_length(
        arg1: *mut en265_encoder_context,
        max_pending_images: ::std::os::raw::c_int,
        timeout_ms: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_trim_input_queue(
        arg1: *mut en265_encoder_context,
        max_pending_images: ::std::os::raw::c_int,
    ) -> de265_error::Type;
}
extern "C" {
    pub fn en265_current_input_queue_length(
        arg1: *mut en265_encoder_context,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn en265_encode(arg1: *mut en265_encoder_context) -> de265_error::Type;
}
pub mod en265_encoder_state {
    pub type Type = ::std::os::raw::c_uint;
    pub const EN265_STATE_IDLE: Type = 0;
    pub const EN265_STATE_WAITING_FOR_INPUT: Type = 1;
    pub const EN265_STATE_WORKING: Type = 2;
    pub const EN265_STATE_OUTPUT_QUEUE_FULL: Type = 3;
    pub const EN265_STATE_EOS: Type = 4;
}
extern "C" {
    pub fn en265_get_encoder_state(arg1: *mut en265_encoder_context) -> en265_encoder_state::Type;
}
pub mod en265_packet_content_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const EN265_PACKET_VPS: Type = 0;
    pub const EN265_PACKET_SPS: Type = 1;
    pub const EN265_PACKET_PPS: Type = 2;
    pub const EN265_PACKET_SEI: Type = 3;
    pub const EN265_PACKET_SLICE: Type = 4;
    pub const EN265_PACKET_SKIPPED_IMAGE: Type = 5;
}
pub mod en265_nal_unit_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const EN265_NUT_TRAIL_N: Type = 0;
    pub const EN265_NUT_TRAIL_R: Type = 1;
    pub const EN265_NUT_TSA_N: Type = 2;
    pub const EN265_NUT_TSA_R: Type = 3;
    pub const EN265_NUT_STSA_N: Type = 4;
    pub const EN265_NUT_STSA_R: Type = 5;
    pub const EN265_NUT_RADL_N: Type = 6;
    pub const EN265_NUT_RADL_R: Type = 7;
    pub const EN265_NUT_RASL_N: Type = 8;
    pub const EN265_NUT_RASL_R: Type = 9;
    pub const EN265_NUT_BLA_W_LP: Type = 16;
    pub const EN265_NUT_BLA_W_RADL: Type = 17;
    pub const EN265_NUT_BLA_N_LP: Type = 18;
    pub const EN265_NUT_IDR_W_RADL: Type = 19;
    pub const EN265_NUT_IDR_N_LP: Type = 20;
    pub const EN265_NUT_CRA: Type = 21;
    pub const EN265_NUT_VPS: Type = 32;
    pub const EN265_NUT_SPS: Type = 33;
    pub const EN265_NUT_PPS: Type = 34;
    pub const EN265_NUT_AUD: Type = 35;
    pub const EN265_NUT_EOS: Type = 36;
    pub const EN265_NUT_EOB: Type = 37;
    pub const EN265_NUT_FD: Type = 38;
    pub const EN265_NUT_PREFIX_SEI: Type = 39;
    pub const EN265_NUT_SUFFIX_SEI: Type = 40;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct en265_packet {
    pub version: ::std::os::raw::c_int,
    pub data: *const u8,
    pub length: ::std::os::raw::c_int,
    pub frame_number: ::std::os::raw::c_int,
    pub content_type: en265_packet_content_type::Type,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub nal_unit_type: en265_nal_unit_type::Type,
    pub nuh_layer_id: ::std::os::raw::c_uchar,
    pub nuh_temporal_id: ::std::os::raw::c_uchar,
    pub encoder_context: *mut en265_encoder_context,
    pub input_image: *const de265_image,
    pub reconstruction: *const de265_image,
}
impl en265_packet {
    #[inline]
    pub fn complete_picture(&self) -> ::std::os::raw::c_char {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_complete_picture(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn final_slice(&self) -> ::std::os::raw::c_char {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_final_slice(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn dependent_slice(&self) -> ::std::os::raw::c_char {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_dependent_slice(&mut self, val: ::std::os::raw::c_char) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        complete_picture: ::std::os::raw::c_char,
        final_slice: ::std::os::raw::c_char,
        dependent_slice: ::std::os::raw::c_char,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let complete_picture: u8 = unsafe { ::std::mem::transmute(complete_picture) };
            complete_picture as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let final_slice: u8 = unsafe { ::std::mem::transmute(final_slice) };
            final_slice as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let dependent_slice: u8 = unsafe { ::std::mem::transmute(dependent_slice) };
            dependent_slice as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn en265_get_packet(
        arg1: *mut en265_encoder_context,
        timeout_ms: ::std::os::raw::c_int,
    ) -> *mut en265_packet;
}
extern "C" {
    pub fn en265_free_packet(arg1: *mut en265_encoder_context, arg2: *mut en265_packet);
}
extern "C" {
    pub fn en265_number_of_queued_packets(
        arg1: *mut en265_encoder_context,
    ) -> ::std::os::raw::c_int;
}
