use std::io;
fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("thunder0", tun_tap::Mode::Tun).expect("Failed to create TUN device");
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf).expect("Failed to read from TUN device");
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        if proto != 0x0800 {
            // no ipv4 packet, ignore
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(header) => {
                println!("Source: {}, Destination: {}", header.source_addr(), header.destination_addr());
                println!("Read {} bytes: {:x?}, flags are {:x?} and the proto is {:x?}", nbytes, header, flags, proto);
            }
            Err(_) => {
                println!("Failed to parse IPv4 header");
            }
        }
    }      
    Ok(())
}
