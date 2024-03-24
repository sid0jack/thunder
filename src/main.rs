use std::io;
fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("thunder0", tun_tap::Mode::Tun).expect("Failed to create TUN device");
    let mut buf = [0u8; 1504];
    let nbytes = nic.recv(&mut buf).expect("Failed to read from TUN device");
    eprint!("Read {} bytes from TUN device\n", nbytes.to_string());        
    Ok(())
}
