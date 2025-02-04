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
        // 16 bits
        // A random ID assigned to query packets. Response packets must reply with the same ID.
        pub fn get_id(&self) -> u16 {
            ((self.id_high as u16) << 8) + self.id_low as u16
        }

        pub fn set_id(&mut self, value: u16) {
            self.id_high = ((value & 0xFF00) >> 8) as u8;
            self.id_low = (value & 0x00FF) as u8;
        }

        // Query/Response Indicator (QR)
        // 1 bit
        // 1 for a reply packet, 0 for a question packet.
        pub fn get_qr(&self) -> bool {
            self.qr_opcode_aa_tc_rd & 0x80 == 0x80
        }

        pub fn set_qr(&mut self, value: bool) {
            let t = self.qr_opcode_aa_tc_rd & 0x7F;
            self.qr_opcode_aa_tc_rd = if value { t | 0x80 } else { t };
        }

        // Operation Code (OPCODE)
        // 4 bits
        pub fn get_opcode(&self) -> u8 {
            (self.qr_opcode_aa_tc_rd & 0x78) >> 3
        }

        pub fn set_opcode(&mut self, value: u8) {
            let v = (value & 0xF) << 3;
            let t = self.qr_opcode_aa_tc_rd & 0x87;
            self.qr_opcode_aa_tc_rd = t | v;

            self.set_rcode(if value == 0 { 0 } else { 4 });
        }

        // Recursion Desired (RD)
        // 1 bit
        pub fn get_rd(&self) -> bool {
            self.qr_opcode_aa_tc_rd & 0x1 == 0x1
        }

        pub fn set_rd(&mut self, value: bool) {
            let t = self.qr_opcode_aa_tc_rd & 0xFE;
            self.qr_opcode_aa_tc_rd = if value { t | 0x1 } else { t };
        }

        pub fn get_rcode(&self) -> u8 {
            self.ra_z_rcode & 0x0F
        }

        fn set_rcode(&mut self, value: u8) {
            let v = value & 0xF;
            let t = self.ra_z_rcode & 0xF0;
            self.ra_z_rcode = t | v;
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

        // Answer Record Count (ANCOUNT)
        // Number of records in the Answer section.
        pub fn get_an_count(&self) -> u16 {
            ((self.an_count_high as u16) << 8) + self.an_count_low as u16
        }

        pub fn set_an_count(&mut self, value: u16) {
            self.an_count_high = ((value & 0xFF00) >> 8) as u8;
            self.an_count_low = (value & 0x00FF) as u8;
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

        pub fn parse_from(data: &[u8; 12]) -> Header {
            Header {
                id_high: data[0],
                id_low: data[1],
                qr_opcode_aa_tc_rd: data[2],
                ra_z_rcode: data[3],
                qd_count_high: data[4],
                qd_count_low: data[5],
                an_count_high: data[6],
                an_count_low: data[7],
                ns_count_high: data[8],
                ns_count_low: data[9],
                ar_count_high: data[10],
                ar_count_low: data[11],
            }
        }
    }

    #[derive(Clone)]
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

    #[derive(Clone)]
    pub struct LabelSequence {
        labels: Vec<Label>,
    }

    impl LabelSequence {
        pub fn new(labels: Vec<Label>) -> LabelSequence {
            LabelSequence { labels: labels }
        }

        pub fn get_labels(&'_ mut self) -> &'_ Vec<Label> {
            &self.labels
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

    #[derive(Clone)]
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

        pub fn get_name(&'_ self) -> &'_ LabelSequence {
            &self.name
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

    pub struct Answer {
        name: LabelSequence,
        r#type: u16,
        class: u16,
        ttl: u32,
        data: Vec<u8>,
    }

    impl Answer {
        pub fn new() -> Answer {
            Answer {
                name: LabelSequence::new(Vec::new()),
                r#type: 0,
                class: 0,
                ttl: 0,
                data: Vec::new(),
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

        pub fn get_ttl(&self) -> u32 {
            self.ttl
        }

        pub fn set_ttl(&mut self, ttl: u32) {
            self.ttl = ttl;
        }

        pub fn get_length(&self) -> u16 {
            self.data.len() as u16
        }

        pub fn get_data(&'_ mut self) -> &'_ mut Vec<u8> {
            &mut self.data
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut result: Vec<u8> = Vec::new();
            result.extend(self.name.encode().iter());
            result.push(((self.r#type & 0xFF00) >> 8) as u8);
            result.push((self.r#type & 0x00FF) as u8);
            result.push(((self.class & 0xFF00) >> 8) as u8);
            result.push((self.class & 0x00FF) as u8);
            result.push(((self.ttl & 0xFF000000) >> 24) as u8);
            result.push(((self.ttl & 0x00FF0000) >> 16) as u8);
            result.push(((self.ttl & 0x0000FF00) >> 8) as u8);
            result.push((self.ttl & 0x000000FF) as u8);
            let length = self.data.len() as u16;
            result.push(((length & 0xFF00) >> 8) as u8);
            result.push((length & 0x00FF) as u8);
            self.data.iter().for_each(|el| result.push(*el));
            result
        }
    }

    pub struct Message {
        header: Header,
        questions: Vec<Question>,
        answers: Vec<Answer>,
    }

    impl Message {
        pub fn new(header: Header, questions: Vec<Question>, answers: Vec<Answer>) -> Message {
            Message {
                header: header,
                questions: questions,
                answers: answers,
            }
        }

        pub fn get_header(&'_ self) -> &'_ Header {
            &self.header
        }

        pub fn get_questions(&'_ self) -> &'_ Vec<Question> {
            &self.questions
        }

        pub fn encode(&self) -> Vec<u8> {
            let mut result: Vec<u8> = Vec::new();
            result.extend_from_slice(&self.header.encode());
            self.questions
                .iter()
                .for_each(|question| result.extend(&question.encode()));
            self.answers
                .iter()
                .for_each(|answer| result.extend(&answer.encode()));
            result
        }

        pub fn parse_from(data: &[u8]) -> Message {
            let header: Header =
                Header::parse_from(data.get(..12).and_then(|s| s.try_into().ok()).expect(
                    "data array length is less than 12 (12 bytes is the size of DNS header).",
                ));
            let payload = &data[12..];
            let (questions, answers) = Message::parse_questions_and_answers(payload, &header);

            Message {
                header: header,
                questions: questions,
                answers: answers,
            }
        }

        fn parse_questions_and_answers(
            data: &[u8],
            header: &Header,
        ) -> (Vec<Question>, Vec<Answer>) {
            let expected_questions_count = header.get_qd_count();
            let mut questions_count: u16 = 0;
            let mut questions: Vec<Question> = Vec::new();
            let mut index: usize = 0;

            while questions_count < expected_questions_count {
                let mut control_byte: u8 = data[index];
                let mut labels: Vec<Label> = Vec::new();
                while control_byte != b'\0' {
                    let content = String::from_utf8(
                        data[(index + 1)..=(index + control_byte as usize)].to_vec(),
                    )
                    .expect("Failed to read label's content");
                    dbg!("Content {}:", &content);
                    labels.push(Label { content: content });
                    index += control_byte as usize + 1;
                    control_byte = data[index];
                }
                index += 1;
                let r#type = ((data[index] as u16) << 8) | (data[index + 1] as u16);
                index += 2;
                let class = ((data[index] as u16) << 8) | (data[index + 1] as u16);
                index += 2;
                questions.push(Question {
                    name: LabelSequence { labels: labels },
                    r#type: r#type,
                    class: class,
                });
                questions_count += 1;
            }

            (questions, Vec::new())
        }
    }
}
