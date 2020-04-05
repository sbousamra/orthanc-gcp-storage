use std::ffi::CString;

pub struct OrthancPluginContext {}

#[no_mangle]
pub extern "C" fn OrthancPluginInitialize(_context: OrthancPluginContext) -> i32 {
  println!("To be implemented");
  return 0;
}

#[no_mangle]
pub extern "C" fn OrthancPluginFinalize() -> () {
  println!("To be implemented");
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetName() -> *const char {
  CString::new("gcp-storage").unwrap().into_raw() as *const char
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetVersion() -> *const char {
  CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw() as *const char
}
