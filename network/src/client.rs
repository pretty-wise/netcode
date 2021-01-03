use std::mem::transmute;

//
// client
//

pub struct NetcodeClient {
    test: i32,
}

#[no_mangle]
pub extern "C" fn client_create() -> *mut NetcodeClient {
    let context = Box::new(NetcodeClient { test: 2 });
    unsafe { transmute(context) }
}

#[no_mangle]
pub extern "C" fn client_destroy(context: *mut NetcodeClient) {
    let _dropped: Box<NetcodeClient> = unsafe { transmute(context) };
}

#[no_mangle]
pub extern "C" fn client_update(context: *mut NetcodeClient) {
    unsafe {
        (*context).test += 1;
    }
}
