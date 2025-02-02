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
        // Packet Identifier (ID)
        // A random ID assigned to query packets. Response packets must reply with the same ID.
        pub fn get_id(&self) -> u16 {
            ((self.id_high as u16) << 8) + self.id_low as u16
        }

        pub fn set_id(&mut self, value: u16) {
            self.id_high = ((value & 0xFF00) >> 8) as u8;
            self.id_low = (value & 0x00FF) as u8;
        }

        // Query/Response Indicator (QR)
        // 1 for a reply packet, 0 for a question packet.
        pub fn get_qr(&self) -> bool {
            self.qr_opcode_aa_tc_rd & 0x80 == 0x80
        }

        pub fn set_qr(&mut self, value: bool) {
            let t = self.qr_opcode_aa_tc_rd & 0x7F;
            self.qr_opcode_aa_tc_rd = if value { t | 0x80 } else { t };
        }

        // Question Count (QDCOUNT)
        // Number of questions in the Question section.
        pub fn get_qd_count(&self) -> u16 {
            ((self.qd_count_high as u16) << 8) + self.qd_count_low as u16
        }

        pub fn set_qd_count(&mut self, value: u16) {
            self.qd_count_high = ((value & 0xFF00) >> 8) as u8;
            self.qd_count_low = (value & 0x00FF) as u8;
        }

        pub fn encode(&self) -> [u8; 12] {
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

    pub struct Label {
        content: String,
    }

    impl Label {
        pub fn new(content: &str) -> Label {
            Label {
                content: content.to_owned(),
            }
        }

        pub fn get_content(&'_ self) -> &'_ str {
            &self.content
        }

        pub fn encode(&self) -> Vec<u8> {
            let length = self.content.len();
            assert!(
                length <= u8::MAX as usize,
                "Label content's length {} is too big (should be less than or equal to {}).",
                length,
                u8::MAX
            );
            let mut result: Vec<u8> = Vec::new();
            result.push(length as u8);
            result.extend_from_slice(self.content.as_bytes());
            result
        }
    }

    pub struct LabelSequence {
        labels: Vec<Label>,
    }

    impl LabelSequence {
        pub fn new(labels: Vec<Label>) -> LabelSequence {
            LabelSequence { labels: labels }
        }

        pub fn get_labels(&'_ mut self) -> &'_ mut Vec<Label> {
            &mut self.labels
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut result: Vec<u8> = Vec::new();
            self.labels
                .iter()
                .for_each(|label| result.extend(label.encode().iter()));
            result.push(b'\0');
            result
        }
    }

    pub struct Question {
        name: LabelSequence,
        r#type: u16,
        class: u16,
    }

    impl Question {
        pub fn new() -> Question {
            Question {
                name: LabelSequence::new(Vec::new()),
                r#type: 0,
                class: 0,
            }
        }

        pub fn get_name(&'_ mut self) -> &'_ mut LabelSequence {
            &mut self.name
        }

        pub fn set_name(&mut self, name: LabelSequence) {
            self.name = name;
        }

        pub fn get_type(&self) -> u16 {
            self.r#type
        }

        pub fn set_type(&mut self, r#type: u16) {
            self.r#type = r#type;
        }

        pub fn get_class(&self) -> u16 {
            self.class
        }

        pub fn set_class(&mut self, class: u16) {
            self.class = class;
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut result: Vec<u8> = Vec::new();
            result.extend(self.name.encode().iter());
            result.push(((self.r#type & 0xFF00) >> 8) as u8);
            result.push((self.r#type & 0x00FF) as u8);
            result.push(((self.class & 0xFF00) >> 8) as u8);
            result.push((self.class & 0x00FF) as u8);
            result
        }
    }

    pub struct Message {
        header: Header,
        questions: Vec<Question>,
    }

    impl Message {
        pub fn new(header: Header, questions: Vec<Question>) -> Message {
            Message {
                header: header,
                questions: questions,
            }
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut result: Vec<u8> = Vec::new();
            result.extend_from_slice(&self.header.encode());
            self.questions
                .iter()
                .for_each(|question| result.extend(&question.encode()));
            result
        }
    }
}
