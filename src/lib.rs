#![deny(clippy::all)]

use std::ffi::CString;

use napi_derive::napi;
#[napi]
pub fn is_hidden_file(file_path: String) -> bool {
  let c_str = CString::new(file_path).expect("Failed to convert to CString");

  let p = c_str.as_ptr() as *const u8;
  // 调用 GetFileAttributesA
  unsafe {
    let attributes = GetFileAttributesA(windows::core::PCSTR(p));
    (attributes & FILE_ATTRIBUTE_HIDDEN.0) != 0
  }
}

use windows::Win32::Storage::FileSystem::{GetFileAttributesA, FILE_ATTRIBUTE_HIDDEN};


#[cfg(test)]
mod tests {
  use windows::Win32::Storage::FileSystem::{GetFileAttributesA, FILE_ATTRIBUTE_HIDDEN};
  fn is_hidden_file(file_path: &str) -> bool {
    let c_str = std::ffi::CString::new(file_path).expect("CString conversion failed");
    // 调用 GetFileAttributesA
    unsafe {
      let attributes = GetFileAttributesA(windows::core::PCSTR(c_str.as_ptr() as *const u8));
      // if attributes == -1 {
      //   // 文件不存在或路径无效
      //   eprintln!("Error: Invalid file path or file not found");
      //   return false;
      // }
      // 检查隐藏属性
      (attributes & FILE_ATTRIBUTE_HIDDEN.0) != 0
    }
  }
  #[test]
  fn main() {
    // assert!(!is_hidden_file("D:\\D.zip"));
    // assert!(is_hidden_file("D:\\AlphaDiscLog.txt"));
    assert!(!is_hidden_file("D:/workspace"));
  }
}
