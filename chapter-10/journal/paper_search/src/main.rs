use std::env;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::raw::{c_void, c_int};
use std::mem;
use std::ffi::{CStr, c_char, CString};

#[no_mangle]
pub fn memory_search(term: *mut c_char) -> *mut c_char {
      let t = unsafe { CStr::from_ptr(term).to_bytes().to_vec() }; 
      let mut output = t.to_vec();
      let search_term: String = String::from_utf8(output).unwrap();
      let res_string = search(search_term, 0, 1).unwrap();
      let mut res: Vec<u8> = res_string.into_iter().nth(0).unwrap().into();
      res.resize(1024, 0); 
      unsafe { CString::from_vec_unchecked(res)}.into_raw()
}

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void { 
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer); 

    pointer as *mut c_void
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    args.reverse();
    let term = args.pop().unwrap_or("rust".to_string());

    let res: Vec<String> = search(term, 0, 10).unwrap();
    for entry in res.iter() {
        println!("{:?}", entry);
    }    
    Ok(())
}

#[no_mangle]
pub fn static_search(
    page: isize,
    offset: isize,
) {
    
    let res: Vec<String> = search("rust".to_string(), page, offset).unwrap();
    for entry in res.iter() { 
        println!("{:?}", entry);
    } 
}


pub fn search(
    term: String,
    page: isize,
    max_results: isize,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let feed: paper_search_lib::Feed = rt.block_on(async {paper_search_lib::search(term, page, max_results) 
        .await}).unwrap();
    let res = feed
        .entry
        .into_iter()
        .map(|e| format!("{} {}", e.title, e.id))
        .collect::<Vec<String>>();
    return Ok::<Vec<String>, Box<dyn std::error::Error>>(res);
}