use orthanc::OrthancPluginContext;

#[derive(Clone, Copy)]
pub struct GCPStorage {
  pub context: *mut OrthancPluginContext,
}

impl GCPStorage {
  pub fn from_context(context: *mut OrthancPluginContext) -> GCPStorage {
    GCPStorage { context }
  }

  pub fn upload_file(&self) -> () {
    unimplemented!();
  }

  pub fn download_file(&self) -> () {
    unimplemented!();
  }

  pub fn delete_file(&self) -> () {
    unimplemented!();
  }
}
