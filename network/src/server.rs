use std::mem::transmute;

//
// server
//

pub struct NetcodeServer {
    test: i32,
}

#[no_mangle]
pub extern "C" fn server_create() -> *mut NetcodeServer {
    let context = Box::new(NetcodeServer { test: 3 });
    unsafe { transmute(context) }
}

#[no_mangle]
pub extern "C" fn server_destroy(context: *mut NetcodeServer) {
    let _dropped: Box<NetcodeServer> = unsafe { transmute(context) };
}

#[no_mangle]
pub extern "C" fn server_update(context: *mut NetcodeServer) {
    unsafe {
        (*context).test += 1;
    }
}
