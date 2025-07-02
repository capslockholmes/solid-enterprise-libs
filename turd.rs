struct Turd<T: Butthole> {
  source: Weak<T>,
}

unsafe impl<#[may_dangle] T> Drop for Turd<T> {
  fn drop(&mut self) {
    if source.is_hairy() {
      // maybe this works
      mem::forget( unsafe { *(self as *mut _) });
    }
  }
}
