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

    pub fn process_segment(&mut self, packet: &[u8]) -> i32 {
        let tcp = split_fragment(packet);

        // inicializar ISN
        if self.isn.is_none() {
            let bytes: [u8; 4] = [tcp[4], tcp[5], tcp[6], tcp[7]];
            let current_seq = u32::from_be_bytes(bytes);
            self.isn = Some(current_seq);
            self.next_seq = current_seq;
            return 0;
        }

        if let Some(initial_seq) = self.isn {
            let bytes: [u8; 4] = [tcp[4], tcp[5], tcp[6], tcp[7]];
            let current_seq = u32::from_be_bytes(bytes);
            let relative_offset = current_seq.wrapping_sub(initial_seq) as usize;

            let data_offset_len = ((tcp[12] >> 4) as usize) * 4;
            let payload = &tcp[data_offset_len..];

            if payload.is_empty() {
                return 1;
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
                }

                _ => return 1,
            }
        }

        0
    }

    pub fn checkin_data(&mut self, initial_seq: u32) {
        let start = (self.next_seq - initial_seq) as usize;
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
    // 1. Inicializar el reensamblador
    let mut reassembler = TcpStreamReassembler::new();

    // 2. Definir paquetes de prueba (Simplificados)
    // Formato: [IP Header (20b)] + [TCP Header (20b)] + [Payload]
    
    // Paquete 1: Establece el ISN (Secuencia 1000)
    let mut pkt1 = vec![0u8; 40]; 
    pkt1[0] = 0x45; // IP IHL = 5 (20 bytes)
    pkt1[24] = 0x03; pkt1[25] = 0xE8; // TCP Seq: 1000 (0x03E8)
    pkt1[32] = 0x50; // TCP Offset = 5 (20 bytes)
    // (Este primer paquete solo inicializa el ISN en tu código actual)

    // Paquete 2: Datos en orden (Secuencia 1000, "Hola")
    let mut pkt2 = pkt1.clone();
    let payload2 = b"Hola";
    pkt2.extend_from_slice(payload2);

    // Paquete 3: Datos del futuro (Secuencia 1010, "Mundo") - Deja un hueco
    let mut pkt3 = vec![0u8; 40];
    pkt3[0] = 0x45;
    pkt3[24] = 0x03; pkt3[25] = 0xF2; // TCP Seq: 1010
    pkt3[32] = 0x50;
    pkt3.extend_from_slice(b"Mundo");

    // 3. Procesar los segmentos
    println!("--- Procesando Paquete 1 (ISN) ---");
    reassembler.process_segment(&pkt1);

    println!("--- Procesando Paquete 3 (FUTURO) ---");
    reassembler.process_segment(&pkt3); 
    // Aquí verás en el buffer que "Mundo" está lejos y hay un hueco de ceros

    println!("--- Procesando Paquete 2 (ORDEN) ---");
    reassembler.process_segment(&pkt2); 
    // Aquí el checkin_data debería detectar que el hueco se llenó

    // 4. Ver el resultado final
    println!("Buffer final: {:?}", reassembler.buffer);
    println!("Contenido: {}", String::from_utf8_lossy(&reassembler.buffer));
}

