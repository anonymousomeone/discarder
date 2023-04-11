use winapi::um::{dpapi, wincrypt::CRYPTOAPI_BLOB};
use winapi::um::winbase::LocalFree;

use std::ptr;
use std::slice;

pub fn decrypt(data: &mut [u8]) -> Option<Vec<u8>> {

    let mut data_in = CRYPTOAPI_BLOB { 
        cbData: data.len() as u32,
        pbData: data.as_mut_ptr()
    };
    
    let mut data_out = CRYPTOAPI_BLOB {
        cbData: 0, 
        pbData: ptr::null_mut()
    };

    let pin = &mut data_in;
    let pout = &mut data_out;

    unsafe {
        let _result = dpapi::CryptUnprotectData(
                pin,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                pout
            );

        if !data_out.pbData.is_null() {
            // Construct a slice from the returned data
            let output = slice::from_raw_parts(data_out.pbData, data_out.cbData as _);
            // Cleanup
            LocalFree(data_out.pbData as _);

            return Some(output.to_vec());
        }

        None
    }
}