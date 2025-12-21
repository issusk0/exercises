

pub fn ex_18(packet: &[u8]) -> usize{
    let first_byte = packet[0];
    let version = first_byte >> 4;
    let ihl = first_byte & 0x0F;
    let ip_header_len = (ihl as usize) * 4;
    println!("Version: {}, IHL: {}, Ip Header Len: {}", version, ihl, ip_header_len);
    ip_header_len

    

    
    
    
}



pub fn data() -> Vec<u8> {

    let packet: Vec<u8> = vec![
    0x45, 0x00, 0x00, 0x34,
    0x12, 0x34, 0x40, 0x00,
    0x40, 0x06, 0x00, 0x00,
    192, 168, 1, 10,
    192, 168, 1, 20,
    // TCP header empieza aquí (byte 20)
    0x50, 0x02, 0x20, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
    packet
    
}
