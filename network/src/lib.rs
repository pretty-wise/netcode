use std::mem::transmute;

pub struct NetcodeClient {
    test: i32,
}

#[no_mangle]
pub extern "C" fn client_create() -> *mut NetcodeClient {
    let value = Box::new(NetcodeClient { test: 2 });
    unsafe { transmute(value) }
}

#[no_mangle]
pub extern "C" fn client_destroy(client: *mut NetcodeClient) {
    let _dropped: Box<NetcodeClient> = unsafe { transmute(client) };
}

#[no_mangle]
pub extern "C" fn client_update(client: *mut NetcodeClient) {
    unsafe {
        (*client).test += 3;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
