#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::io::ErrorKind;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::*;
use std::ptr::null_mut;
use log::{error,warn, trace};

const MODULE:&str = "test_win_api";


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
                break;
            }            
        }
    }
    Ok(str_list)
}

fn clip<'a>(clip_str: &'a str, clip_start: Option<&str>, clip_end: Option<&str>) -> &'a str {            
    let mut work_str = clip_str;

    if let Some(s) = clip_start {
        if !s.is_empty() && work_str.starts_with(s) {        
            work_str = &work_str[s.len()..];
        }
    }

    if let Some(s) = clip_end {
        if !s.is_empty() && work_str.ends_with(s) {
            work_str = &work_str[0..(work_str.len()- s.len())];
        }
    }

    work_str
}

#[cfg(windows)]
fn get_volumes() -> Result<Vec<String>,Box<std::error::Error>> {
    trace!("{}::get_volumes: entered", MODULE);
    use winapi::um::handleapi::{INVALID_HANDLE_VALUE};
    use winapi::um::fileapi::{FindFirstVolumeW, FindNextVolumeW, FindVolumeClose};        
    const BUFFER_SIZE: usize = 2048;
    let mut buffer: [u16;BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut vol_list: Vec<String> = Vec::new();

    let h_search = unsafe {
        FindFirstVolumeW(buffer.as_mut_ptr(), BUFFER_SIZE as u32)
    };
    
    if h_search == INVALID_HANDLE_VALUE {        
        return Err(Box::new(Error::last_os_error()));
    }

    vol_list.push(to_string(&buffer)?);

    loop {
        let ret = unsafe { FindNextVolumeW(h_search, buffer.as_mut_ptr(), BUFFER_SIZE as u32) };
        if ret == 0 {
            unsafe { FindVolumeClose(h_search) };
            return Ok(vol_list);
        }
        vol_list.push(to_string(&buffer)?);
    }
}


#[cfg(windows)]
fn query_dos_device(dev_name: Option<&str>) -> Result<Vec<String>,Box<std::error::Error>> {
    trace!("{}::query_dos_device: entered with {:?}" , MODULE, dev_name);
    use winapi::um::fileapi::{ QueryDosDeviceW};        
    const BUFFER_SIZE: usize = 131072;
    let mut buffer: [u16;BUFFER_SIZE] = [0; BUFFER_SIZE];
    let num_tchar = match dev_name {
        Some(s) => {
            let dev_path: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
            unsafe { QueryDosDeviceW(dev_path.as_ptr(),buffer.as_mut_ptr(),BUFFER_SIZE as u32) } 
        },
        None => unsafe { QueryDosDeviceW(null_mut(),buffer.as_mut_ptr(),BUFFER_SIZE as u32) }
    };
    
    
    if num_tchar > 0 {
        trace!("{}::query_dos_device: success", MODULE);
        Ok(to_string_list(&buffer)?)
    } else {
       let os_err = Error::last_os_error();
       warn!("{}::query_dos_device: returned {}, last os error: {:?} ", MODULE, num_tchar, os_err);
       return Err(Box::new(os_err));        
    }
}

#[cfg(windows)]
pub fn enumerate_volumes() -> Result<i32, Box<std::error::Error>> {    
    
    // use winapi::um::winbase::{FindFirstVolumeA, FindNextVolumeA};
    
    match query_dos_device(None) { 
        Ok(sl) => {
            for device in sl {
                println!("got device name: {}",device);
            }
        },
        Err(why) => {
            println!("query_dos_device retured error: {:?}", why);
        }
    };

    
    for vol_name in get_volumes()? {
        let dev_name = clip(&vol_name,Some("\\\\?\\"), Some("\\"));

        println!("got dev_name: {}",dev_name);

        for device in query_dos_device(Some(dev_name))? {
            println!("  got dev_name: {}",device);
        }
    }    
    
    Ok(0)
}

