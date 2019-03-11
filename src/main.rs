#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::ffi::CString;
use std::io::ErrorKind;


#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
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
fn enumerate_volumes() -> Result<i32, Error> {    
    use winapi::um::handleapi::{INVALID_HANDLE_VALUE, CloseHandle};
    use winapi::um::winnt::{FILE_SHARE_READ, GENERIC_READ};        
    use winapi::um::fileapi::{CreateFileW, OPEN_EXISTING};        
    use winapi::um::winbase::{FindFirstVolumeW, FindNextVolumeW, FindVolumeClose};
    use std::ptr::null_mut;

    const BUFFER_SIZE: usize = 1024;
    let mut buffer: [u16;BUFFER_SIZE] = [0; BUFFER_SIZE];
    
    println!("calling FindFirstVolumeW");

    let h_search = unsafe {
        FindFirstVolumeW(buffer.as_mut_ptr(), BUFFER_SIZE as u32)
    };

    if h_search == INVALID_HANDLE_VALUE {
        println!("got invalid handle enumerating volumes");
        return Err(Error::last_os_error());
    } else {        
        let os_string = OsString::from_wide(&buffer);        
        println!("got volume: {:?}",os_string);        
        loop {
            
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
            } else {
                let os_string = OsString::from_wide(&buffer);
                println!("got volume: {:?}",os_string);
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