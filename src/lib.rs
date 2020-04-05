pub struct OrthancPluginContext {}

#[no_mangle]
pub extern "C" fn OrthancPluginInitialize(context: OrthancPluginContext) -> i32 {
  println!("To be implemented");
  return 0;
}

#[no_mangle]
pub extern "C" fn OrthancPluginFinalize() -> () {
  println!("To be implemented");
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetName() -> *const char {
  println!("To be implemented");
  return &'c';
}

#[no_mangle]
pub extern "C" fn OrthancPluginGetVersion() -> *const char {
  println!("To be implemented");
  return &'c';
}
