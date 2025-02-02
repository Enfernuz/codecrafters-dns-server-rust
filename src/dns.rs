pub mod message {

    #[derive(Default)]
    pub struct Header {
        id_high: u8,
        id_low: u8,
        qr_opcode_aa_tc_rd: u8,
        ra_z_rcode: u8,
        qd_count_high: u8,
        qd_count_low: u8,
        an_count_high: u8,
        an_count_low: u8,
        ns_count_high: u8,
        ns_count_low: u8,
        ar_count_high: u8,
        ar_count_low: u8,
    }

    impl Header {
        pub fn get_id(&self) -> u16 {
            ((self.id_high as u16) << 8) + self.id_low as u16
        }

        pub fn set_id(&mut self, value: u16) {
            self.id_high = ((value & 0xFF00) >> 8) as u8;
            self.id_low = (value & 0x00FF) as u8;
        }

        pub fn get_qr(&self) -> bool {
            self.qr_opcode_aa_tc_rd & 0x80 == 0x80
        }

        pub fn set_qr(&mut self, value: bool) {
            let t = self.qr_opcode_aa_tc_rd & 0x7F;
            self.qr_opcode_aa_tc_rd = if value { t | 0x80 } else { t };
        }

        pub fn as_bytes(&self) -> [u8; 12] {
            [
                self.id_high,
                self.id_low,
                self.qr_opcode_aa_tc_rd,
                self.ra_z_rcode,
                self.qd_count_high,
                self.qd_count_low,
                self.an_count_high,
                self.an_count_low,
                self.ns_count_high,
                self.ns_count_low,
                self.ar_count_high,
                self.ar_count_low,
            ]
        }
    }
}
