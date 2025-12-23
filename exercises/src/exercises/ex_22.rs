pub fn split_fragment(packet: &[u8]) -> &[u8] {
    let ihl = packet[0] & 0x0F;
    let iphl = (ihl as usize) * 4;
    &packet[iphl..]
}

struct TcpStreamReassembler {
    isn: Option<u32>,
    buffer: Vec<u8>,
    next_seq: u32,
    recieve_mask: Vec<bool>,
}

impl TcpStreamReassembler {
    pub fn new() -> Self {
        TcpStreamReassembler {
            isn: None,
            buffer: Vec::new(),
            next_seq: 0,
            recieve_mask: Vec::new(),
        }
    }

    pub fn process_segment(&mut self, packet: &[u8]) {
        let tcp = split_fragment(packet);
        let bytes: [u8; 4] = [tcp[4], tcp[5], tcp[6], tcp[7]];
        let current_seq = u32::from_be_bytes(bytes);
        // inicializar ISN
        if self.isn.is_none() || current_seq < self.isn.unwrap() {
            self.isn = Some(current_seq);
            self.next_seq = current_seq;
        }

        if let Some(initial_seq) = self.isn {
            let bytes: [u8; 4] = [tcp[4], tcp[5], tcp[6], tcp[7]];
            let current_seq = u32::from_be_bytes(bytes);
            let relative_offset = current_seq.wrapping_sub(initial_seq) as usize;

            let data_offset_len = ((tcp[12] >> 4) as usize) * 4;
            let payload = &tcp[data_offset_len..];

            if payload.is_empty() {
            }

            match current_seq {
                // paquete esperado
                _ if current_seq == self.next_seq => {
                    self.next_seq = current_seq.wrapping_add(payload.len() as u32);

                    let start = relative_offset;
                    let end = start + payload.len();
                    if end > self.buffer.len() {
                        self.buffer.resize(end, 0);
                    }
                    if end > self.recieve_mask.len() {
                        self.recieve_mask.resize(end, false);
                    }
                    self.buffer[start..end].copy_from_slice(payload);
                    self.recieve_mask[start..end].fill(true);
                    self.checkin_data(self.isn.unwrap_or(0));
                }

                // paquete antiguo
                _ if current_seq < self.next_seq => {
                    let bytes_to_skip = (self.next_seq - current_seq) as usize;
                    if payload.len() > bytes_to_skip {
                        let new_bytes = &payload[bytes_to_skip..];
                        let start =
                            self.next_seq.wrapping_sub(initial_seq) as usize;
                        let end = start + new_bytes.len();

                        if end > self.buffer.len() {
                            self.buffer.resize(end, 0);
                        }
                        if end > self.recieve_mask.len() {
                            self.recieve_mask.resize(end, false);
                        }
                        self.buffer[start..end].copy_from_slice(new_bytes);
                        self.recieve_mask[start..end].fill(true);
                        self.next_seq =
                            current_seq.wrapping_add(payload.len() as u32);
                        self.checkin_data(self.isn.unwrap_or(0));
                    }
                }

                // paquete futuro
                _ if current_seq > self.next_seq => {
                    let start = relative_offset;
                    let end = start + payload.len();

                    if end > self.buffer.len() {
                        self.buffer.resize(end, 0);
                    }
                    if end > self.recieve_mask.len() {
                        self.recieve_mask.resize(end, false);
                    }
                    self.buffer[start..end].copy_from_slice(payload);
                    self.recieve_mask[start..end].fill(true);
                    self.checkin_data(self.isn.unwrap_or(0));
                }

                _ => {}
            }
        }
    }

    pub fn checkin_data(&mut self, initial_seq: u32) {
        if self.next_seq < initial_seq {
            return;
        }
        let start = (self.next_seq.wrapping_sub(initial_seq)) as usize;
        if start < self.recieve_mask.len() {
            for state in &self.recieve_mask[start..] {
                if !*state {
                    break;
                }
                self.next_seq += 1;
            }
        }
    }
}






pub fn ex_22() {
    let mut reassembler = TcpStreamReassembler::new();
    let pkt1: Vec<u8> = vec![
        0x45,0x00, 0x00,0x3c,  
        0x00,0x01, 0x40,0x00,
        0x40, 0x06,
        0x00,0x00,
        192,168,1,10,
        192,168,1,20,
        0x30,0x39,            
        0x01,0xbb,            
        0x00,0x00,0x03,0xE8,  
        0x00,0x00,0x00,0x00,  
        0x50,0x18,            
        0x72,0x10,
        0x00,0x00,
        0x00,0x00,
        0x16,0x03,0x03,0x00,0x05,
        0x01,0xAA,0xBB,0xCC,0xDD,
    ];
    let pkt2: Vec<u8> = vec![
        0x45,0x00, 0x00,0x3c,
        0x00,0x02, 0x40,0x00,
        0x40, 0x06,
        0x00,0x00,
        192,168,1,10,
        192,168,1,20,
        0x30,0x39,
        0x01,0xbb,
        0x00,0x00,0x03,0xF2,  
        0x00,0x00,0x00,0x00,
        0x50,0x18,
        0x72,0x10,
        0x00,0x00,
        0x00,0x00,
        0xEE,0xFF,0x11,0x22,0x33,
        0x44,0x55,0x66,0x77,0x88,
    ];
    reassembler.process_segment(&pkt2);
    reassembler.process_segment(&pkt1);


    // Buscamos el primer 0x16 (inicio de TLS) y mostramos los siguientes 20 bytes
    if let Some(start) = reassembler.buffer.iter().position(|&x| x != 0) {
        let end = (start + 20).min(reassembler.buffer.len());
        println!("Flujo TLS Completo: {:X?}", &reassembler.buffer[start..end]);
    }else{
        println!("El buffer esta vacío")
        
    } //asignamos el valor absoluto si el x es distinto de 0   
    

}

