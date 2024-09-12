use protos::gen::ffi::EngineException as NativeEngineException;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type EngineResult<A> = Result<A, EngineException>;

#[repr(C)]
pub struct ScalaResult {
    data: *mut u8,
    len: usize,
    is_successful: bool,
}

impl ScalaResult {
    fn buffer_to_result(data: Vec<u8>, is_successful: bool) -> Self {
        let mut buf = data.into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);
        Self {
            data,
            len,
            is_successful,
        }
    }

    pub fn success(data: Vec<u8>) -> Self {
        Self::buffer_to_result(data, true)
    }

    pub fn error(data: Vec<u8>) -> Self {
        Self::buffer_to_result(data, false)
    }
}

#[no_mangle]
pub extern "C" fn free_scala_result(scala_result: ScalaResult) {
    let s = unsafe { std::slice::from_raw_parts_mut(scala_result.data, scala_result.len) };
    let s = s.as_mut_ptr();
    unsafe {
        let _ = Box::from_raw(s);
    }
}

struct EngineException {
    exception_type: EngineExceptionType,
    msg: String,
}

impl EngineException {
    pub fn to_native(self) -> NativeEngineException {
        NativeEngineException {
            name: self.exception_type.to_string(),
            message: self.msg,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EngineExceptionType {
    AddressParseException,
    AssertionError,
    BrokenPipeError,
    InvalidAddressError,
    InvalidFilename,
    InvalidParametersError,
    InvalidSpecPathError,
    InvalidTargetNameError,
    KeyboardInterruptError,
    SciondClientException,
    SciondConnectionException,
    UnsupportedWildcardError,
}

impl EngineExceptionType {
    fn new_err(self, msg: &str) -> EngineException {
        EngineException {
            exception_type: self,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for EngineExceptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl BinaryConvertible for EngineException {
    fn to_binary_result(self) -> Vec<u8> {
        todo!()
    }
}

pub trait BinaryConvertible {
    fn to_binary_result(self) -> Vec<u8>;
}
