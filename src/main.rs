#[cfg(windows)] extern crate winapi;
use std::io::Error;
//use std::ffi::CString;
use std::io::ErrorKind;
use std::ffi::{OsString, OsStr};
use std::ptr::null_mut;
use std::os::windows::prelude::*;
use std::iter::once;

/*
#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

fn to_c_string(os_str_buf: &[u8]) -> Result<CString,Box<std::error::Error>> {    
    match os_str_buf.iter().position(|&x| x == 0 ) {
        Some(i) => { 
            match CString::new(os_str_buf[0..i].to_vec()) {
                Ok(c) => Ok(c),
                Err(why) => Err(Box::new(why)),
            }            
        },
        None => return Err(Box::new(Error::from(ErrorKind::InvalidInput)))
    }
}
#[cfg(windows)]
fn to_os_string(os_str_buf: &[u16]) -> Result<OsString,Box<std::error::Error>> {            
    match os_str_buf.iter().position(|&x| x == 0 ) {        
        Some(i) => Ok(OsString::from_wide(&os_str_buf[0..i])),
        None => return Err(Box::new(Error::from(ErrorKind::InvalidInput)))
    }
}
*/


/*
fn clip(os_str_buf: &[u16],start: usize,end: usize) -> Result<String,Box<std::error::Error>> {            
    match os_str_buf.iter().position(|&x| x == 0 ) {                
        Some(i) => {
            if i <= (start + end) {
                Ok(String::new())
            } else {
                Ok(String::from_utf16_lossy(&os_str_buf[start..(i - end)]))
            }
        },
        None => return Err(Box::new(Error::from(ErrorKind::InvalidInput)))
    }
}
*/



#[cfg(not(windows))]
fn main() {
    println!("this program is meant to run on windows OS ");
}

#[cfg(windows)]
fn main() {
    enumerate_volumes().unwrap();
}