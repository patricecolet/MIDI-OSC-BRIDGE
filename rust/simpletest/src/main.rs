use rosc::{encoder, OscMessage, OscPacket, OscType, OscBundle, OscTime};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available port
    let target_addr = "127.0.0.1:9000"; // Replace with your target IP and port

    // MIDI messages (using raw MIDI bytes)
    // Note On (Channel 1, A4 = 69, Velocity 100)
    let note_on = vec![0x90, 69, 100];  // 0x90 = Note On for Channel 1 (0x9 + channel 0)
    
    // Note Off (Channel 1, A4 = 69, Velocity 0)
    let note_off = vec![0x80, 69, 0];   // 0x80 = Note Off for Channel 1

    // Control Change (Channel 2, Controller 1, Value 12)
    let control_change = vec![0xB1, 1, 12]; // 0xB1 = Control Change for Channel 2 (0xB + channel 1)
    
    // Wrap MIDI data in OSC messages
    let osc_note_on = OscMessage {
        addr: "/midi".to_string(),
        args: vec![OscType::Blob(note_on.into())],
    };

    let osc_note_off = OscMessage {
        addr: "/midi".to_string(),
        args: vec![OscType::Blob(note_off.into())],
    };

    let osc_control_change = OscMessage {
        addr: "/midi".to_string(),
        args: vec![OscType::Blob(control_change.into())],
    };
    
    // Bundle OSC messages
    let packet = OscPacket::Bundle(OscBundle {
        timetag: OscTime {
            seconds: 0,
            fractional: 0,
        },
        content: vec![
            OscPacket::Message(osc_note_on),
            OscPacket::Message(osc_note_off),
            OscPacket::Message(osc_control_change),
        ],
    });

    // Encode OSC bundle to bytes
    let buf = encoder::encode(&packet).expect("Failed to encode OSC packet");
    
    // Send the buffer over UDP
    socket.send_to(&buf, target_addr)?;

    println!("OSC bundle sent to {}", target_addr);
    
    Ok(())
}