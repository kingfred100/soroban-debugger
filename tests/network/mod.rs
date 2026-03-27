use std::net::TcpListener;

/// Checks if the current environment allows binding to a local port.
/// Returns true if loopback networking is available.
pub fn can_bind_loopback() -> bool {
    match TcpListener::bind("127.0.0.1:0") {
        Ok(_) => true,
        Err(e) => {
            eprintln!("⚠️ Loopback networking restricted: {}", e);
            false
        }
    }
}
