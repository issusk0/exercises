



pub fn ex_21(packet: &[u8]){
    //si es ipv4 se parte desde 0 sino desde el 14
    let start_index = check_header(packet);
    

    if start_index > 2500 {
        println!("error en el paquete");
    }
    
    is_ipv4(packet, start_index);

}



pub fn data() -> Vec<u8>{
    let mut packet: Vec<u8> = vec![
        // =====================
        // Ethernet II (14 bytes)
        // =====================
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // Dest MAC
        0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, // Src MAC
        0x08, 0x00,                         // EtherType = IPv4

        // =====================
        // IPv4 HEADER (20 bytes)
        // =====================
        0x45, 0x00, 0x00, 0x50, // Version/IHL, TOS, Total Length = 80 bytes
        0xab, 0xcd, 0x40, 0x00, // Identification, Flags
        0x40, 0x06, 0x00, 0x00, // TTL, Protocol TCP
        0xc0, 0xa8, 0x01, 0x02, // Src IP: 192.168.1.2
        0xc0, 0xa8, 0x01, 0x01, // Dst IP: 192.168.1.1

        // =====================
        // TCP HEADER (20 bytes)
        // =====================
        0x30, 0x39, 0x00, 0x50, // Src Port 12345, Dst Port 80
        0x00, 0x00, 0x00, 0x01, // Seq
        0x00, 0x00, 0x00, 0x00, // Ack
        0x50, 0x18, 0x20, 0x00, // DataOffset=5 (20 bytes), Flags PSH+ACK
        0x00, 0x00, 0x00, 0x00, // Checksum, Urgent

        // =====================
        // HTTP PAYLOAD
        // =====================
        0x47, 0x45, 0x54, 0x20, // "GET "
        0x2f, 0x20,             // "/ "
        0x48, 0x54, 0x54, 0x50, // "HTTP"
        0x2f, 0x31, 0x2e, 0x31, // "/1.1"
        0x0d, 0x0a,             // CRLF
        0x48, 0x6f, 0x73, 0x74, // "Host"
        0x3a, 0x20,
        0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d,
        0x0d, 0x0a,
        0x0d, 0x0a,
    ];
    packet.resize(1024, 0x00);
    packet



}



fn check_header(packet: &[u8]) -> usize{
    //tiene tcp pero no tampoco cumple con el minimo para ser http, es tcp invalido
    if packet.len() <20{return 2000;}
    

    //para ver si es ipv4
    

    if packet[0] >> 4 == 4{
        return 0;

    }

    //para checkar si es ethernet
    if packet.len()>=34{
         let byte_1 = packet[12];
         let byte_2 = packet[13];
         let ether_type = ((byte_1 as u16) << 8)|(byte_2 as u16);
         let version_after_eth = packet[14] >> 4;
         if ether_type == 0x0800 && version_after_eth == 4{
            return 14
         }

    }
    
    //no se pudo concretar ni eth o ip
    return 2000

}
fn is_ipv4(packet: &[u8],index: usize){
    let ihl = packet[index] & 0x0F;
    let ip_header_len = (ihl as usize) * 4;


    let tcp = &packet[index + ip_header_len..];
    let data_offset = tcp[12] >> 4;
    let tcp_header_len = (data_offset as usize) * 4;

    let http = &tcp[tcp_header_len..];
    let s = std::str::from_utf8(http).unwrap();

    println!("{}", s);


}
