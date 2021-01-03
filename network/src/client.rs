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

#[cfg(test)]
mod tests {
    use super::client_create;
    use super::client_destroy;
    use super::client_update;
    #[test]
    fn instatiation() {
        let instance = client_create();
        assert!(!instance.is_null());
        client_update(instance);
        client_destroy(instance);
    }
}
