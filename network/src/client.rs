use crate::socketio;
use std::mem::transmute;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct NetcodeClient {
    test: i32,
    io: socketio::Context,
}

#[no_mangle]
pub extern "C" fn client_create() -> *mut NetcodeClient {
    let local_addr = SocketAddr::from_str("127.0.0.1:0").unwrap();
    let (socket_io, _port) = socketio::Context::new(local_addr);

    let context = Box::new(NetcodeClient {
        test: 2,
        io: socket_io,
    });

    unsafe { transmute(context) }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_destroy(context: *mut NetcodeClient) {
    let _dropped: Box<NetcodeClient> = transmute(context);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_update(context: *mut NetcodeClient) {
    let client = &mut *context;

    client.test += 1;

    while let Ok(data) = client.io.try_recv() {
        println!(
            "client read {}({}) on main. time since recv: {}ms",
            data.nbytes,
            data.buffer.len(),
            data.recv_time.elapsed().as_millis()
        );
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
        unsafe { client_update(instance) };
        unsafe { client_destroy(instance) };
    }
}
