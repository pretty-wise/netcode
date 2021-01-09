pub mod socketio {
    use std::net::{SocketAddr, UdpSocket};
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use std::time;

    use mpsc::TryRecvError;

    pub struct Packet {
        pub recv_time: time::Instant,
        pub nbytes: usize,
        pub buffer: Vec<u8>,
    }

    pub struct Context {
        recv_thread: Option<thread::JoinHandle<()>>, // "the option dance"
        read_rx: Receiver<Packet>,
        socket: UdpSocket,
        local_addr: SocketAddr, // used to send message to self to unblock the thread
    }

    impl Context {
        pub fn new(local_addr: SocketAddr) -> Context {
            let socket = UdpSocket::bind(local_addr).unwrap();
            let recv_socket = socket.try_clone().unwrap();

            let (tx, rx): (Sender<Packet>, Receiver<Packet>) = mpsc::channel();

            let thread = thread::spawn(move || {
                loop {
                    let mut buffer = vec![0; 1500];
                    match recv_socket.recv_from(&mut buffer.as_mut_slice()) {
                        Ok((nbytes, src_addr)) => {
                            let recv_time = time::Instant::now();
                            if nbytes == 1 {
                                break; // see a note in client_destroy.
                            }
                            buffer.resize(nbytes, 0);
                            println!("received {} bytes from {}", nbytes, src_addr);
                            tx.send(Packet {
                                nbytes,
                                recv_time,
                                buffer,
                            })
                            .unwrap();
                        }
                        Err(_) => break,
                    }
                }
            });

            Context {
                recv_thread: Some(thread),
                read_rx: rx,
                socket,
                local_addr,
            }
        }

        pub fn send(&self, buf: &[u8], dest: SocketAddr) -> std::io::Result<usize> {
            self.socket.send_to(buf, dest)
        }

        pub fn try_recv(&self) -> Result<Packet, TryRecvError> {
            self.read_rx.try_recv()
        }
    }

    impl Drop for Context {
        fn drop(&mut self) {
            // note(kstasik):
            // 1 bytes is sent from the main thread when terminating library as rust exposes no way to close a socket.
            let empty = [0; 1];
            self.send(&empty, self.local_addr).unwrap();
            self.recv_thread.take().unwrap().join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::socketio;
    use core::panic;
    use std::{net::SocketAddr, str::FromStr};

    #[test]
    fn lifetime() {
        let addr = SocketAddr::from_str("127.0.0.1:8888").unwrap();
        let _context = socketio::Context::new(addr);
    }

    #[test]
    fn messaging() {
        let addr = SocketAddr::from_str("127.0.0.1:9999").unwrap();
        let context = socketio::Context::new(addr);

        let msg = "test";
        let size = context.send(msg.as_bytes(), addr).unwrap();
        assert_eq!(size, msg.len());

        loop {
            match context.try_recv() {
                Ok(packet) => {
                    assert_eq!(packet.buffer.len(), msg.len());
                    break;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => continue,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("socket failure?"),
            }
        }
    }
}
