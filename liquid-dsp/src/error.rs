use liquid_dsp_sys as sys;
use thiserror::Error;

/// Error values returned from libliquid itself.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ErrorKind {
    /// internal logic error; this is a bug with liquid and should be reported immediately
    #[error("internal logic error; this is a bug with liquid and should be reported immediately")]
    Internal,

    /// invalid object, examples:
    ///  - destroy() method called on NULL pointer
    #[error("invalid object")]
    Object,

    /// invalid parameter, or configuration; examples:
    ///  - setting bandwidth of a filter to a negative number
    ///  - setting FFT size to zero
    ///  - create a spectral periodogram object with window size greater than nfft
    #[error("invalid parameter or configuration")]
    Config,

    /// input out of range; examples:
    ///  - try to take log of -1
    ///  - try to create an FFT plan of size zero
    #[error("input out of range")]
    Input,

    /// invalid vector length or dimension; examples
    ///  - trying to refer to the 17th element of a 2 x 2 matrix
    ///  - trying to multiply two matrices of incompatible dimensions
    #[error("invalid vector length or dimension")]
    Range,

    /// invalid mode; examples:
    ///  - try to create a modem of type 'LIQUID_MODEM_XXX' which does not exit
    #[error("invalid mode")]
    InvalidMode,

    /// unsupported mode (e.g. LIQUID_FEC_CONV_V27 with 'libfec' not installed)
    #[error("unsupported mode")]
    UnsupportedMode,

    /// object has not been created or properly initialized
    ///  - try to run firfilt_crcf_execute(NULL, ...)
    ///  - try to modulate using an arbitrary modem without initializing the constellation
    #[error("object has not been created or properly initialized")]
    NoInit,

    /// not enough memory allocated for operation; examples:
    ///  - try to factor 100 = 2*2*5*5 but only give 3 spaces for factors
    #[error("not enough memory allocated for operation")]
    Memory,

    /// file input/output; examples:
    ///  - could not open a file for writing because of insufficient permissions
    ///  - could not open a file for reading because it does not exist
    ///  - try to read more data than a file has space for
    ///  - could not parse line in file (improper formatting)
    #[error("file IO error")]
    Io,
}

#[allow(dead_code)]
pub(crate) struct PassThrough(pub u32);

impl TryFrom<i32> for PassThrough {
    type Error = ErrorKind;

    fn try_from(err_code: i32) -> Result<PassThrough, ErrorKind> {
        match err_code as u32 {
            sys::liquid_error_code_LIQUID_EINT => Err(ErrorKind::Internal),
            sys::liquid_error_code_LIQUID_EIOBJ => Err(ErrorKind::Object),
            sys::liquid_error_code_LIQUID_EICONFIG => Err(ErrorKind::Config),
            sys::liquid_error_code_LIQUID_EIVAL => Err(ErrorKind::Input),
            sys::liquid_error_code_LIQUID_EIRANGE => Err(ErrorKind::Range),
            sys::liquid_error_code_LIQUID_EIMODE => Err(ErrorKind::InvalidMode),
            sys::liquid_error_code_LIQUID_EUMODE => Err(ErrorKind::UnsupportedMode),
            sys::liquid_error_code_LIQUID_ENOINIT => Err(ErrorKind::NoInit),
            sys::liquid_error_code_LIQUID_EIMEM => Err(ErrorKind::Memory),
            sys::liquid_error_code_LIQUID_EIO => Err(ErrorKind::Io),
            other => Ok(PassThrough(other)),
        }
    }
}
