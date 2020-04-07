extern crate libc;

mod orthanc_plugin;
use orthanc_plugin::*;

use libc::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::sync::Once;

static mut CONTEXT: Option<*mut OrthancPluginContext> = None;
static INITIALIZED: Once = Once::new();

unsafe fn get_context() -> *mut OrthancPluginContext {
  return CONTEXT.unwrap();
}

unsafe extern "C" fn create(
  uuid: *const c_char,
  _content: *const c_void,
  size: i64,
  content_type: OrthancPluginContentType,
) -> OrthancPluginErrorCode {
  let uuid = CStr::from_ptr(uuid).to_string_lossy().into_owned();

  orthanc::log_info(
    get_context(),
    format!(
      "Creating file: UUID={} size={} contentType={:?}",
      uuid, size, content_type
    ),
  );

  return OrthancPluginErrorCode::Success;
}
unsafe extern "C" fn read(
  _content: *mut *mut c_void,
  size: *mut i64,
  uuid: *const c_char,
  content_type: OrthancPluginContentType,
) -> OrthancPluginErrorCode {
  let uuid = CStr::from_ptr(uuid).to_string_lossy().into_owned();

  orthanc::log_info(
    get_context(),
    format!(
      "Reading file: UUID={} size={} contentType={:?}",
      uuid, &*size, content_type
    ),
  );
  return OrthancPluginErrorCode::Success;
}

unsafe extern "C" fn remove(
  uuid: *const c_char,
  content_type: OrthancPluginContentType,
) -> OrthancPluginErrorCode {
  let uuid = CStr::from_ptr(uuid).to_string_lossy().into_owned();

  orthanc::log_info(
    get_context(),
    format!(
      "Removing file: UUID={} contentType={:?}",
      uuid, content_type
    ),
  );

  return OrthancPluginErrorCode::Success;
}

#[no_mangle]
unsafe extern "C" fn OrthancPluginInitialize(context: *mut OrthancPluginContext) -> i32 {
  orthanc::log_info(context, "[GCP] Storage plugin is initializing".to_string());

  let version_check = orthanc::check_version(context);
  let version = orthanc::get_version(context);

  if version_check == 0 {
    orthanc::log_error(
      context,
      format!(
        "Your version of Orthanc ({}) must be above 1.4.0 to run this plugin.",
        version
      ),
    );
    return -1;
  }

  orthanc::set_description(context, "Implementation of GCP Storage.".to_string());

  INITIALIZED.call_once(|| {
    CONTEXT = Some(context);
  });

  orthanc::register_storage_area(context, create, read, remove);

  return 0;
}

#[no_mangle]
pub extern "C" fn OrthancPluginFinalize() -> () {
  println!("OrthancPluginFinalize");
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetName() -> *const char {
  CString::new("gcp-storage").unwrap().into_raw() as *const char
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetVersion() -> *const char {
  CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw() as *const char
}
