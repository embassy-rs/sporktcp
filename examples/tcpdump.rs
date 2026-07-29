use std::env;
use std::os::unix::io::AsRawFd;
use xarxa::phy::wait as phy_wait;
use xarxa::phy::{Device, RawSocket, RxToken};
use xarxa::wire::{EthernetFrame, PrettyPrinter};

fn main() {
    let ifname = env::args().nth(1).unwrap();
    let mut socket = RawSocket::new(ifname.as_ref(), xarxa::phy::DriverMedium::Ethernet).unwrap();
    loop {
        phy_wait(socket.as_raw_fd(), None).unwrap();
        let (rx_token, _) = socket.receive().unwrap();
        rx_token.consume(|buffer| {
            println!(
                "{}",
                PrettyPrinter::<EthernetFrame<&[u8]>>::new("", &buffer)
            );
        })
    }
}
