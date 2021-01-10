pub mod client;
pub mod server;
pub mod simserver;
pub mod simshared;
pub mod socketio;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
