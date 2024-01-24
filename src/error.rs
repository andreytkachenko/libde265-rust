use std::ffi::CStr;

use libde265_sys::de265_error;

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum Error {
    #[error("no such file")]
    NoSuchFile,

    #[error("checksum mismatch")]
    ChecksumMismatch,

    #[error("ctb outside image area")]
    CtbOutsideImageArea,

    #[error("out of memory")]
    OutOfMemory,

    #[error("coded parameter out of range")]
    CodedParameterOutOfRange,

    #[error("image buffer full")]
    ImageBufferFull,

    #[error("cannot start threadpool")]
    CannotStartThreadpool,

    #[error("Library Initialization Failed")]
    LibraryInitializationFailed,

    #[error("coefficient out of image bounds")]
    CoefficientOutOfImageBounds,

    #[error("library not initialized")]
    LibraryNotInitialized,

    #[error("waiting for input data")]
    WaitingForInputData,

    #[error("cannot process sei")]
    CannotProcessSei,

    #[error("parameter parsing")]
    ParameterParsing,

    #[error("noinitial slice header")]
    NoInitialSliceHeader,

    #[error("premature end of slice")]
    PrematureEndOfSlice,

    #[error("unspecified decoding error")]
    UnspecifiedDecodingError,

    #[error("not yet implemented")]
    NotImplementedYet,

    #[error("unknown error {0}")]
    UnknownError(u32),
}

impl Error {
    pub fn from_res(kind: de265_error::Type) -> Result<(), Self> {
        match kind {
            de265_error::DE265_OK => Ok(()),
            de265_error::DE265_ERROR_NO_SUCH_FILE => Err(Error::NoSuchFile),
            de265_error::DE265_ERROR_COEFFICIENT_OUT_OF_IMAGE_BOUNDS => {
                Err(Error::CoefficientOutOfImageBounds)
            }
            de265_error::DE265_ERROR_CHECKSUM_MISMATCH => Err(Error::ChecksumMismatch),
            de265_error::DE265_ERROR_CTB_OUTSIDE_IMAGE_AREA => Err(Error::CtbOutsideImageArea),
            de265_error::DE265_ERROR_OUT_OF_MEMORY => Err(Error::OutOfMemory),
            de265_error::DE265_ERROR_CODED_PARAMETER_OUT_OF_RANGE => {
                Err(Error::CodedParameterOutOfRange)
            }
            de265_error::DE265_ERROR_IMAGE_BUFFER_FULL => Err(Error::ImageBufferFull),
            de265_error::DE265_ERROR_CANNOT_START_THREADPOOL => Err(Error::CannotStartThreadpool),
            de265_error::DE265_ERROR_LIBRARY_INITIALIZATION_FAILED => {
                Err(Error::LibraryInitializationFailed)
            }
            de265_error::DE265_ERROR_LIBRARY_NOT_INITIALIZED => Err(Error::LibraryNotInitialized),
            de265_error::DE265_ERROR_WAITING_FOR_INPUT_DATA => Err(Error::WaitingForInputData),
            de265_error::DE265_ERROR_CANNOT_PROCESS_SEI => Err(Error::CannotProcessSei),
            de265_error::DE265_ERROR_PARAMETER_PARSING => Err(Error::ParameterParsing),
            de265_error::DE265_ERROR_NO_INITIAL_SLICE_HEADER => Err(Error::NoInitialSliceHeader),
            de265_error::DE265_ERROR_PREMATURE_END_OF_SLICE => Err(Error::PrematureEndOfSlice),
            de265_error::DE265_ERROR_UNSPECIFIED_DECODING_ERROR => {
                Err(Error::UnspecifiedDecodingError)
            }
            de265_error::DE265_ERROR_NOT_IMPLEMENTED_YET => Err(Error::NotImplementedYet),

            warning @ 1000.. => {
                let msg = unsafe { CStr::from_ptr(libde265_sys::de265_get_error_text(warning)) };
                log::warn!("{}", msg.to_string_lossy());
                Ok(())
            }
            code => Err(Self::UnknownError(code)),
        }
    }
}
