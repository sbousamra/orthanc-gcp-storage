use libc::{c_char, c_int};

use std::ffi::{c_void, CStr, CString};

#[repr(C)]
pub struct OrthancPluginContext {
  plugins_manager: c_void,
  orthanc_version: *const c_char,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OrthancPluginErrorCode {
  InternalError = -1,
  Success = 0,
  Plugin = 1,
  NotImplemented = 2,
  ParameterOutOfRange = 3,
  NotEnoughMemory = 4,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum OrthancPluginContentType {
  Unknown = 0,
  Dicom = 1,
  DicomAsJson = 2,
  INTERNAL = 0x7fffffff,
}

pub type OrthancPluginStorageCreate = unsafe extern "C" fn(
  *const c_char,
  *const c_void,
  i64,
  OrthancPluginContentType,
) -> OrthancPluginErrorCode;

pub type OrthancPluginStorageRead = unsafe extern "C" fn(
  *mut *mut c_void,
  *mut i64,
  *const c_char,
  OrthancPluginContentType,
) -> OrthancPluginErrorCode;

pub type OrthancPluginStorageRemove =
  unsafe extern "C" fn(*const c_char, OrthancPluginContentType) -> OrthancPluginErrorCode;

#[allow(improper_ctypes, dead_code)]
extern "C" {
  fn OrthancPluginLogErrorWrapped(context: *mut OrthancPluginContext, message: *const c_char);
  fn OrthancPluginLogWarningWrapped(context: *mut OrthancPluginContext, message: *const c_char);
  fn OrthancPluginLogInfoWrapped(context: *mut OrthancPluginContext, message: *const c_char);
  fn OrthancPluginCheckVersionWrapped(context: *mut OrthancPluginContext) -> c_int;
  fn OrthancPluginSetDescriptionWrapped(
    context: *mut OrthancPluginContext,
    description: *const c_char,
  );
  fn OrthancPluginGetConfigurationWrapped(context: *mut OrthancPluginContext) -> *mut c_char;
  fn OrthancPluginRegisterStorageAreaWrapped(
    context: *mut OrthancPluginContext,
    create: OrthancPluginStorageCreate,
    read: OrthancPluginStorageRead,
    remove: OrthancPluginStorageRemove,
  );

}

#[allow(dead_code)]
pub mod orthanc {

  use super::*;

  pub unsafe fn log_error(context: *mut OrthancPluginContext, message: String) -> () {
    OrthancPluginLogErrorWrapped(context, CString::new(message).unwrap().into_raw())
  }

  pub unsafe fn log_warning(context: *mut OrthancPluginContext, message: String) -> () {
    OrthancPluginLogWarningWrapped(context, CString::new(message).unwrap().into_raw())
  }

  pub unsafe fn log_info(context: *mut OrthancPluginContext, message: String) -> () {
    OrthancPluginLogInfoWrapped(context, CString::new(message).unwrap().into_raw())
  }

  pub unsafe fn check_version(context: *mut OrthancPluginContext) -> i32 {
    OrthancPluginCheckVersionWrapped(context)
  }

  pub unsafe fn set_description(context: *mut OrthancPluginContext, description: String) -> () {
    OrthancPluginSetDescriptionWrapped(context, CString::new(description).unwrap().into_raw())
  }

  pub unsafe fn get_configuration(context: *mut OrthancPluginContext) -> String {
    return CStr::from_ptr(OrthancPluginGetConfigurationWrapped(context))
      .to_string_lossy()
      .into_owned();
  }

  pub unsafe fn register_storage_area(
    context: *mut OrthancPluginContext,
    create: OrthancPluginStorageCreate,
    read: OrthancPluginStorageRead,
    remove: OrthancPluginStorageRemove,
  ) -> () {
    return OrthancPluginRegisterStorageAreaWrapped(context, create, read, remove);
  }

  pub unsafe fn get_version(context: *mut OrthancPluginContext) -> String {
    return CStr::from_ptr((*context).orthanc_version)
      .to_string_lossy()
      .into_owned();
  }
}
