
mod packet_parsing;
mod handler;

use crate::handler::*;

fn main() -> std::io::Result<()> {
    let mut client = UdpClient::new();

    loop {
        let packets = client.poll_for_updates();
        for packet in packets {
            client.receive_packet(packet);
        }

        println!("{:?}", client.get_rotation(0));
    }

    Ok(())
}