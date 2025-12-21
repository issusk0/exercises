pub fn ex_20(packet: &[u8])-> usize{

    let first_byte = packet[0];
    let version = first_byte >> 4;
    let ihl = first_byte & 0x0F;
    let iphl = (ihl as usize)* 4;
    

    let tcp = &packet[iphl..];
    let data_offset = tcp[0] >> 4;
    let offset_header = (data_offset as usize) * 4;
    println!("iphl: {}, tcp header len: {}", iphl, offset_header);
    offset_header


}


pub fn data() -> Vec<u8>{
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
