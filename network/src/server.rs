use crate::socketio;
use std::{mem::transmute, net::SocketAddr, str::FromStr};

pub struct NetcodeServer {
    io: socketio::Context,
}

#[no_mangle]
pub extern "C" fn server_create() -> *mut NetcodeServer {
    let local_addr = SocketAddr::from_str("127.0.0.1:0").unwrap();
    let (socket_io, _port) = socketio::Context::new(local_addr);

    let context = Box::new(NetcodeServer { io: socket_io });
    unsafe { transmute(context) }
}

#[no_mangle]
pub extern "C" fn server_destroy(context: *mut NetcodeServer) {
    let _dropped: Box<NetcodeServer> = unsafe { transmute(context) };
}

#[no_mangle]
pub extern "C" fn server_update(context: *mut NetcodeServer) {
    let server = unsafe { &mut *context };

    // tick server loop

    loop {
        match server.io.try_recv() {
            Ok(data) => {
                println!(
                    "server read {}({}) on main. time since recv: {}ms",
                    data.nbytes,
                    data.buffer.len(),
                    data.recv_time.elapsed().as_millis()
                );
                continue;
            }
            Err(_) => break,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::server_create;
    use super::server_destroy;
    use super::server_update;
    #[test]
    fn instatiation() {
        let instance = server_create();
        assert!(!instance.is_null());
        server_update(instance);
        server_destroy(instance);
    }
}
