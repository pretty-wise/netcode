mod actor_ids;
mod cmd_buffer;
mod control;
mod simulation;
mod world;

use crate::shared::socketio;
use std::{mem::transmute, net::SocketAddr, slice, str::FromStr, time};

pub struct NetcodeServer {
    io: socketio::Context,
    simulation: simulation::Simulation,
}

#[no_mangle]
pub extern "C" fn server_create() -> *mut NetcodeServer {
    let local_addr = SocketAddr::from_str("127.0.0.1:0").unwrap();
    let (socket_io, _port) = socketio::Context::new(local_addr);
    let simulation = simulation::Simulation::start(0, time::Duration::from_millis(16), 8);

    let context = Box::new(NetcodeServer {
        io: socket_io,
        simulation,
    });
    unsafe { transmute(context) }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn server_destroy(context: *mut NetcodeServer) {
    let _dropped: Box<NetcodeServer> = transmute(context);
    _dropped.simulation.stop();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn server_update(context: *mut NetcodeServer) {
    let server = &mut *context;

    // tick server loop
    const UPDATE_DELTA: time::Duration = time::Duration::from_millis(16);
    server.simulation.update(UPDATE_DELTA);

    while let Ok(data) = server.io.try_recv() {
        println!(
            "server read {}({}) on main. time since recv: {}ms",
            data.nbytes,
            data.buffer.len(),
            data.recv_time.elapsed().as_millis()
        );
        continue;
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn server_read(
    context: *mut NetcodeServer,
    buffer: *const u8,
    nbytes: usize,
) {
    let server = &mut *context;
    server
        .simulation
        .read(slice::from_raw_parts(buffer, nbytes));
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
        unsafe { server_update(instance) };
        unsafe { server_destroy(instance) };
    }
}
