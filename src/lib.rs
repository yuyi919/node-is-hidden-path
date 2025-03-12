#![deny(clippy::all)]

use std::ffi::OsStr;
use std::fs;
use std::os::windows::fs::MetadataExt;

use std::os::windows::ffi::OsStrExt;

use napi_derive::napi;
use windows::Win32::Foundation::{
  GetLastError, ERROR_ACCESS_DENIED, ERROR_SHARING_VIOLATION, INVALID_HANDLE_VALUE,
};
use windows::Win32::Storage::FileSystem::{
  FindFirstFileW, GetFileAttributesW, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM,
  INVALID_FILE_ATTRIBUTES, WIN32_FIND_DATAW,
};

fn is_hidden_file_attributes(attributes: u32) -> bool {
  (attributes & (FILE_ATTRIBUTE_HIDDEN.0 | FILE_ATTRIBUTE_SYSTEM.0)) != 0
}

/**
 * @deprecated
 */
#[napi]
pub fn is_hidden_file(file_path: String) -> bool {
  let metadata = fs::metadata(file_path).expect("msg");

  // 获取文件属性
  let attributes = metadata.file_attributes();
  if attributes == INVALID_FILE_ATTRIBUTES {
    // 文件不存在或路径无效
    eprintln!("Error: Invalid file path or file not found");
    return false;
  }
  // 检查隐藏属性
  is_hidden_file_attributes(attributes)
}

#[napi]
pub fn is_hidden_file_win32(file_path: String) -> bool {
  let path_str = file_path.as_str();
  let path: &std::path::Path = std::path::Path::new(path_str);
  let (_wide_path, ptr): (Vec<u16>, windows::core::PCWSTR) = get_ptr(path.as_os_str());
  let attributes = unsafe { GetFileAttributesW(ptr) };

  if attributes == INVALID_FILE_ATTRIBUTES {
    // 获取错误代码
    let error_code = unsafe { GetLastError() };

    // 检查是否为访问被拒绝
    if error_code == ERROR_ACCESS_DENIED || error_code == ERROR_SHARING_VIOLATION {
      let mut find_data = WIN32_FIND_DATAW::default();
      let handle = unsafe { FindFirstFileW(ptr, &mut find_data) }.expect(path_str);

      if handle == INVALID_HANDLE_VALUE {
        println!("INVALID_HANDLE_VALUE {:?}", path);
        return false;
      }
      return is_hidden_file_attributes(find_data.dwFileAttributes);
    }
    // 文件不存在或路径无效
    eprintln!("Invalid {:?} {:?}", path_str, error_code,);
    return false;
  }
  // 检查隐藏属性
  is_hidden_file_attributes(attributes)
}

fn get_wide_path(ostr: &OsStr) -> Vec<u16> {
  let mut wide_path: Vec<u16> = ostr.encode_wide().collect();

  if wide_path.last() == Some(&47u16) {
    wide_path.pop();
  }
  wide_path.push(0); // 添加 null 终止符

  wide_path
}

fn get_ptr(ostr: &OsStr) -> (Vec<u16>, windows::core::PCWSTR) {
  let wide_path: Vec<u16> = get_wide_path(ostr);
  let ptr: windows::core::PCWSTR = windows::core::PCWSTR(wide_path.as_ptr() as *const u16);
  (wide_path, ptr)
}

/**
 * @deprecated
 */
#[napi]
pub fn is_hidden_file_win32_2(file_path: String) -> bool {
  let path_str = file_path.as_str();
  let path: &std::path::Path = std::path::Path::new(path_str);
  let (_wide_path, ptr): (Vec<u16>, windows::core::PCWSTR) = get_ptr(path.as_os_str());

  let mut find_data = WIN32_FIND_DATAW::default();

  let handle = unsafe { FindFirstFileW(ptr, &mut find_data) }.expect(path_str);

  if handle == INVALID_HANDLE_VALUE {
    println!("INVALID_HANDLE_VALUE {:?}", path);
    return false;
  }
  return is_hidden_file_attributes(find_data.dwFileAttributes);
}

#[cfg(test)]
mod tests {
  use std::ffi::OsStr;
  use std::os::windows::ffi::OsStrExt;

  use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
  use windows::Win32::Storage::FileSystem::{
    FindFirstFileW, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM, WIN32_FIND_DATAW,
  };
  fn os_str_to_pcwstr(ostr: &OsStr) -> windows::core::PCWSTR {
    let mut wide_path: Vec<u16> = ostr.encode_wide().collect();
    if wide_path.last().unwrap() == &47u16 {
      // 移除路径末尾的"/"
      wide_path.pop();
    }
    wide_path.push(0); // 添加 null 终止符
    windows::core::PCWSTR(wide_path.as_ptr() as *const u16)
  }

  fn is_hidden_file<P: AsRef<std::path::Path>>(path: P) -> bool {
    let ostr = std::ffi::OsStr::new(path.as_ref().as_os_str());
    let ptr: windows::core::PCWSTR = os_str_to_pcwstr(ostr);
    let mut find_data = WIN32_FIND_DATAW::default();
    let handle = unsafe { FindFirstFileW(ptr, &mut find_data).expect("msg") };

    if handle == INVALID_HANDLE_VALUE {
      return false;
    }
    return find_data.dwFileAttributes & (FILE_ATTRIBUTE_SYSTEM.0 | FILE_ATTRIBUTE_HIDDEN.0) != 0;
  }
  #[test]
  fn main() {
    assert!(!is_hidden_file("D:\\D.zip"));
    assert!(is_hidden_file("D:\\AlphaDiscLog.txt"));
    assert!(!is_hidden_file("D:/workspace"));
    assert!(!is_hidden_file("D:/素材/"));
    assert!(is_hidden_file("C:/Documents and Settings/"));
    assert!(is_hidden_file("C:/System Volume Information"));
    assert!(is_hidden_file("C:/hiberfil.sys"));
    assert!(is_hidden_file("C:/DumpStack.log.tmp"));
  }
}
