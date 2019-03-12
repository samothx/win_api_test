#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::ffi::CString;
use std::io::ErrorKind;
use std::ffi::{OsString, OsStr};
use std::ptr::null_mut;
use std::os::windows::prelude::*;
use std::iter::once;

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

fn to_string(os_str_buf: &[u16]) -> Result<String,Box<std::error::Error>> {            
    match os_str_buf.iter().position(|&x| x == 0 ) {        
        Some(i) => Ok(String::from_utf16_lossy(&os_str_buf[0..i])),
        None => return Err(Box::new(Error::from(ErrorKind::InvalidInput)))
    }
}


fn to_string_list(os_str_buf: &[u16]) -> Result<Vec<String>,Box<std::error::Error>> {            
    let mut str_list: Vec<String> = Vec::new();
    let mut start: usize = 0;
    for curr in os_str_buf.iter().enumerate() {
        if *curr.1 == 0 {
            if  start < curr.0 {
                match to_string(&os_str_buf[start .. curr.0 + 1]) {
                    Ok(s) =>  { 
                        str_list.push(s);
                        start = curr.0 + 1;
                    },
                    Err(why) => return Err(why),
                }                
            } else {
                // TODO: might be better to allways terminate                
                break;
            }            
        }
    }
    Ok(str_list)
}


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


#[cfg(windows)]
fn enumerate_volumes() -> Result<i32, Error> {    

    use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
    use winapi::um::winnt::{FILE_SHARE_READ, GENERIC_READ};        
    use winapi::um::fileapi::{CreateFileW, FindFirstVolumeW, FindNextVolumeW, FindVolumeClose, QueryDosDeviceW, OPEN_EXISTING};        
    // use winapi::um::winbase::{FindFirstVolumeA, FindNextVolumeA};
    

    const BUFFER_SIZE: usize = 2048;
    let mut buffer: [u16;BUFFER_SIZE] = [0; BUFFER_SIZE];
    
    // println!("calling FindFirstVolumeW");

    let h_search = unsafe {
        FindFirstVolumeW(buffer.as_mut_ptr(), BUFFER_SIZE as u32)
    };

    if h_search == INVALID_HANDLE_VALUE {
        println!("got invalid handle enumerating volumes");
        return Err(Error::last_os_error());
    } else {        
        loop {
            let vol_name = to_string(&buffer).unwrap();        
            println!("got volume: {}",vol_name);

            let dev_name = if vol_name.starts_with("\\\\?\\") && vol_name.ends_with("\\") {
                clip(&buffer, 4, 1).unwrap()
            } else { 
                vol_name.clone() 
            };

            println!("got dev_name: {}",dev_name);

            let dev_path: Vec<u16> = OsStr::new(&dev_name).encode_wide().chain(once(0)).collect();

            let ret = unsafe { QueryDosDeviceW(dev_path.as_ptr(),buffer.as_mut_ptr(),BUFFER_SIZE as u32) } ;
            if ret != 0 {
                for device in to_string_list(&buffer).unwrap().iter() {
                    println!("got device name: {}",device);
                }
            } else {
                println!("QueryDosDeviceW returned : {}",ret);
            }
            
            /*
            let h_file = unsafe { CreateFileA(
                c_string.clone().into_raw(),
                GENERIC_READ,
                FILE_SHARE_READ,
                null_mut(), 
                OPEN_EXISTING,                
                0,
                null_mut()) };

                if h_file == INVALID_HANDLE_VALUE {
                    println!("failed to open volume: {:?}", c_string);
                    println!("last OS error: {:?}", Error::last_os_error());
                } else {
                    println!("succeeded to open volume: {:?}", c_string);
                    unsafe { CloseHandle(h_file) };
                }
            */   

            let ret = unsafe { FindNextVolumeW(h_search, buffer.as_mut_ptr(), BUFFER_SIZE as u32) };
            if ret == 0 {
                break;                
            } 
        }

        unsafe { FindVolumeClose(h_search) };
    }
    
    
    Ok(0)
}


#[cfg(not(windows))]
fn print_message() -> Result<(), Error> {
    println!("not on windows");
    Ok(())
}

fn main() {
    enumerate_volumes().unwrap();
}