pub mod message {
    use std::{fmt, rc::Rc, str, u8};

    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum OpCode {
        #[default]
        Query, // 0
        IQuery,                // 1 (Obsolete)
        Status,                // 2
        Notify,                // 4
        Update,                // 5
        DnsStatefulOperations, // 6
        Unassigned(u8),        //3, 7-15
    }

    impl fmt::Display for OpCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let code: &str = match self {
                Self::Query => "QUERY (0)",
                Self::IQuery => "IQUERY (1)",
                Self::Status => "STATUS (2)",
                Self::Notify => "NOTIFY (4)",
                Self::Update => "UPDATE (5)",
                Self::DnsStatefulOperations => "DSO (6)",
                Self::Unassigned(value) => &format!("UNASSIGNED ({})", value),
            };
            write!(f, "{}", code)
        }
    }

    impl From<&OpCode> for u8 {
        fn from(value: &OpCode) -> Self {
            match value {
                OpCode::Query => 0,
                OpCode::IQuery => 1,
                OpCode::Status => 2,
                OpCode::Notify => 4,
                OpCode::Update => 5,
                OpCode::DnsStatefulOperations => 6,
                OpCode::Unassigned(x) => *x,
            }
        }
    }

    #[derive(Debug)]
    pub struct OpCodeParseError {
        pub message: String,
    }

    impl TryFrom<u8> for OpCode {
        type Error = OpCodeParseError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Query),
                1 => Ok(Self::IQuery),
                2 => Ok(Self::Status),
                3 => Ok(Self::Unassigned(3)),
                4 => Ok(Self::Notify),
                5 => Ok(Self::Update),
                6 => Ok(Self::DnsStatefulOperations),
                7..=15 => Ok(Self::Unassigned(value)),
                16..=u8::MAX => Err(OpCodeParseError {
                    message: format!(
                        "OpCode must be from 0 to 15 (4 bits), but the value is {}.",
                        value
                    ),
                }),
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub enum RCode {
        #[default]
        NoError,
        FormatError,
        ServerError,
        NameError,
        NotImplemented,
        Refused,
        Unassigned(u8),
    }

    #[derive(Debug)]
    pub struct RCodeParseError {
        pub message: String,
    }

    impl From<&RCode> for u8 {
        fn from(value: &RCode) -> Self {
            match value {
                RCode::NoError => 0,
                RCode::FormatError => 1,
                RCode::ServerError => 2,
                RCode::NameError => 3,
                RCode::NotImplemented => 4,
                RCode::Refused => 5,
                RCode::Unassigned(x) => *x,
            }
        }
    }

    impl TryFrom<u8> for RCode {
        type Error = RCodeParseError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::NoError),
                1 => Ok(Self::FormatError),
                2 => Ok(Self::ServerError),
                3 => Ok(Self::NameError),
                4 => Ok(Self::NotImplemented),
                5 => Ok(Self::Refused),
                6..=15 => Ok(Self::Unassigned(value)),
                16..=u8::MAX => Err(RCodeParseError {
                    message: format!(
                        "RCode must be from 0 to 15 (4 bits), but the value is {}.",
                        value
                    ),
                }),
            }
        }
    }

    impl fmt::Display for RCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let code: &str = match self {
                Self::NoError => "NO_ERROR (0)",
                Self::FormatError => "FORMAT_ERROR (1)",
                Self::ServerError => "SERVER_ERROR (2)",
                Self::NameError => "NAME_ERROR (3)",
                Self::NotImplemented => "NOT_IMPLEMENTED (4)",
                Self::Refused => "REFUSED (5)",
                Self::Unassigned(value) => &format!("UNASSIGNED ({})", value),
            };
            write!(f, "{}", code)
        }
    }

    #[derive(Debug, Default)]
    pub struct Header {
        id: u16,
        qr: bool,
        opcode: Rc<OpCode>,
        aa: bool,
        tc: bool,
        rd: bool,
        ra: bool,
        z: u8,
        rcode: Rc<RCode>,
        qd_count: u16,
        an_count: u16,
        ns_count: u16,
        ar_count: u16,
    }

    impl Header {
        // Packet Identifier (ID)
        // 16 bits
        // A random ID assigned to query packets. Response packets must reply with the same ID.
        pub fn get_id(&self) -> u16 {
            self.id
        }

        pub fn set_id(&'_ mut self, id: u16) -> &'_ mut Self {
            self.id = id;
            self
        }

        // Query/Response Indicator (QR)
        // 1 bit
        // 1 for a reply packet, 0 for a question packet.
        pub fn get_qr(&self) -> bool {
            self.qr
        }

        pub fn set_qr(&mut self, qr: bool) -> &'_ mut Self {
            self.qr = qr;
            self
        }

        // Operation Code (OPCODE)
        // 4 bits
        pub fn get_opcode(&'_ self) -> &'_ Rc<OpCode> {
            &self.opcode
        }

        pub fn set_opcode(&mut self, opcode: &Rc<OpCode>) -> &'_ mut Self {
            self.opcode = Rc::clone(opcode);
            self
        }

        // Recursion Desired (RD)
        // 1 bit
        pub fn get_rd(&self) -> bool {
            self.rd
        }

        pub fn set_rd(&mut self, rd: bool) -> &'_ mut Self {
            self.rd = rd;
            self
        }

        pub fn get_rcode(&'_ self) -> &'_ Rc<RCode> {
            &self.rcode
        }

        pub fn set_rcode(&mut self, rcode: &Rc<RCode>) -> &'_ mut Self {
            self.rcode = Rc::clone(rcode);
            self
        }

        // Question Count (QDCOUNT)
        // Number of questions in the Question section.
        pub fn get_qd_count(&self) -> u16 {
            self.qd_count
        }

        pub fn set_qd_count(&mut self, qd_count: u16) -> &'_ mut Self {
            self.qd_count = qd_count;
            self
        }

        // Answer Record Count (ANCOUNT)
        // Number of records in the Answer section.
        pub fn get_an_count(&self) -> u16 {
            self.an_count
        }

        pub fn set_an_count(&mut self, an_count: u16) -> &'_ mut Self {
            self.an_count = an_count;
            self
        }

        pub fn encode(&self) -> [u8; 12] {
            let id: [u8; 2] = self.id.to_be_bytes();
            let qr: u8 = if self.qr { 0x80 } else { 0 };
            let opcode = u8::from(self.opcode.as_ref()) << 3;
            let aa: u8 = if self.aa { 0x04 } else { 0 };
            let tc: u8 = if self.tc { 0x02 } else { 0 };
            let rd: u8 = if self.rd { 0x01 } else { 0 };
            let ra: u8 = if self.ra { 0x01 } else { 0 };
            let z: u8 = self.z << 4;
            let rcode: u8 = u8::from(self.rcode.as_ref());
            let qd_count: [u8; 2] = self.qd_count.to_be_bytes();
            let an_count: [u8; 2] = self.an_count.to_be_bytes();
            let ns_count: [u8; 2] = self.ns_count.to_be_bytes();
            let ar_count: [u8; 2] = self.ar_count.to_be_bytes();
            [
                id[0],
                id[1],
                qr | opcode | aa | tc | rd,
                ra | z | rcode,
                qd_count[0],
                qd_count[1],
                an_count[0],
                an_count[1],
                ns_count[0],
                ns_count[1],
                ar_count[0],
                ar_count[1],
            ]
        }

        pub fn parse_from(data: &[u8; 12]) -> Header {
            let qr_opcode_aa_tc_rd: u8 = data[2];
            let ra_z_rcode: u8 = data[3];
            Header {
                id: u16::from_be_bytes([data[0], data[1]]),
                qr: qr_opcode_aa_tc_rd & 0x80 == 0x80,
                opcode: Rc::new(
                    ((qr_opcode_aa_tc_rd & 0x78) >> 3)
                        .try_into()
                        .expect("Could not parse opcode."),
                ),
                aa: qr_opcode_aa_tc_rd & 0x04 == 0x04,
                tc: qr_opcode_aa_tc_rd & 0x02 == 0x02,
                rd: qr_opcode_aa_tc_rd & 0x01 == 0x01,
                ra: ra_z_rcode & 0x80 == 0x80,
                z: ra_z_rcode & 0x70 >> 4,
                rcode: Rc::new(
                    (ra_z_rcode & 0x0F)
                        .try_into()
                        .expect("Could not parse rcode."),
                ),
                qd_count: u16::from_be_bytes([data[4], data[5]]),
                an_count: u16::from_be_bytes([data[6], data[7]]),
                ns_count: u16::from_be_bytes([data[8], data[9]]),
                ar_count: u16::from_be_bytes([data[10], data[11]]),
            }
        }
    }

    impl fmt::Display for Header {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Example:
            // ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 36383
            // ;; flags: qr rd ra ad; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0

            let mut flags: Vec<&str> = Vec::new();
            if self.qr {
                flags.push("qr");
            }
            if self.aa {
                flags.push("aa");
            }
            if self.tc {
                flags.push("tc");
            }
            if self.rd {
                flags.push("rd");
            }
            if self.ra {
                flags.push("ra");
            }

            let flags = format!(
                "flags: {}; QUERY: {}; ANSWER: {}; AUTHORITY: {}; ADDITIONAL: {}",
                flags.join(" "),
                self.qd_count,
                self.an_count,
                self.ns_count,
                self.ar_count
            );

            let opcode = &self.opcode;
            let rcode = &self.rcode;
            let id = self.id;

            write!(
                f,
                ";; opcode: {opcode}, status: {rcode}, id: {id}\n;; {flags}"
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct Label {
        content: Rc<str>,
    }

    impl Label {
        pub fn new(content: &Rc<str>) -> Label {
            Label {
                content: Rc::clone(content),
            }
        }

        pub fn get_content(&self) -> &Rc<str> {
            &self.content
        }

        pub fn encode(&self) -> Rc<[u8]> {
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
            result.into()
        }
    }

    #[derive(Clone, Debug)]
    pub struct LabelSequence {
        labels: Rc<[Label]>,
    }

    impl LabelSequence {
        pub fn new(labels: &Rc<[Label]>) -> LabelSequence {
            LabelSequence {
                labels: Rc::clone(labels),
            }
        }

        pub fn get_labels(&self) -> &Rc<[Label]> {
            &self.labels
        }

        pub fn encode(&self) -> Rc<[u8]> {
            let mut result: Vec<u8> = Vec::new();
            self.labels
                .iter()
                .for_each(|label| result.extend(label.encode().iter()));
            result.push(b'\0');
            result.into()
        }
    }

    impl fmt::Display for LabelSequence {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let parts: Vec<&str> = self
                .labels
                .iter()
                .map(|label| label.content.as_ref())
                .collect();
            write!(f, "{}", parts.join("."))
        }
    }

    #[derive(Clone, Debug)]
    pub struct Question {
        name: Rc<LabelSequence>,
        r#type: u16,
        class: u16,
    }

    impl Question {
        pub fn new(name: &Rc<LabelSequence>, r#type: u16, class: u16) -> Question {
            Question {
                name: Rc::clone(name),
                r#type: r#type,
                class: class,
            }
        }

        pub fn get_name(&self) -> &Rc<LabelSequence> {
            &self.name
        }

        pub fn get_type(&self) -> u16 {
            self.r#type
        }

        pub fn get_class(&self) -> u16 {
            self.class
        }

        pub fn encode(&self) -> Rc<[u8]> {
            let mut result: Vec<u8> = Vec::new();
            result.extend(self.name.encode().iter());
            result.push(((self.r#type & 0xFF00) >> 8) as u8);
            result.push((self.r#type & 0x00FF) as u8);
            result.push(((self.class & 0xFF00) >> 8) as u8);
            result.push((self.class & 0x00FF) as u8);
            result.into()
        }
    }

    impl fmt::Display for Question {
        // Example:
        // ;codecrafters.io.    IN       A
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = &self.name;
            let _type = self.r#type;
            let class = self.class;
            write!(f, "{name}    {_type}    {class}")
        }
    }

    #[derive(Clone, Debug)]
    pub struct Answer {
        name: Rc<LabelSequence>,
        r#type: u16,
        class: u16,
        ttl: u32,
        data: Rc<[u8]>,
    }

    impl Answer {
        pub fn new(
            name: &Rc<LabelSequence>,
            r#type: u16,
            class: u16,
            ttl: u32,
            data: &Rc<[u8]>,
        ) -> Answer {
            Answer {
                name: Rc::clone(name),
                r#type: r#type,
                class: class,
                ttl: ttl,
                data: Rc::clone(data),
            }
        }

        pub fn get_name(&self) -> &Rc<LabelSequence> {
            &self.name
        }

        pub fn get_type(&self) -> u16 {
            self.r#type
        }

        pub fn get_class(&self) -> u16 {
            self.class
        }

        pub fn get_ttl(&self) -> u32 {
            self.ttl
        }

        pub fn get_data_length(&self) -> u16 {
            self.data.len() as u16
        }

        pub fn get_data(&self) -> &Rc<[u8]> {
            &self.data
        }

        pub fn encode(&self) -> Rc<[u8]> {
            let mut result: Vec<u8> = Vec::new();
            result.extend_from_slice(&self.name.encode());
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
            result.extend_from_slice(&self.data);
            result.into()
        }
    }

    impl fmt::Display for Answer {
        // Example
        // codecrafters.io.     3600    IN      A       76.76.21.21
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = &self.name;
            let ttl = self.ttl;
            let _type = self.r#type;
            let class = self.class;
            let address_parts: Vec<String> = self.data.iter().map(u8::to_string).collect();
            let address = address_parts.join("."); // TODO: IPv6 representation
            write!(f, "{name}    {ttl}    {_type}    {class}    {address}")
        }
    }

    #[derive(Debug)]
    pub struct Message {
        header: Rc<Header>,
        questions: Rc<[Question]>,
        answers: Rc<[Answer]>,
    }

    impl Message {
        pub fn new(
            header: &Rc<Header>,
            questions: &Rc<[Question]>,
            answers: &Rc<[Answer]>,
        ) -> Message {
            Message {
                header: Rc::clone(header),
                questions: questions.clone(),
                answers: answers.clone(),
            }
        }

        pub fn get_header(&self) -> &Rc<Header> {
            &self.header
        }

        pub fn get_questions(&self) -> &Rc<[Question]> {
            &self.questions
        }

        pub fn get_answers(&self) -> &Rc<[Answer]> {
            &self.answers
        }

        pub fn encode(&self) -> Rc<[u8]> {
            let mut result: Vec<u8> = Vec::new();
            result.extend_from_slice(&self.header.encode());
            self.questions
                .iter()
                .for_each(|question| result.extend_from_slice(&question.encode()));
            self.answers.iter().for_each(|answer| {
                result.extend_from_slice(&answer.encode());
            });
            result.into()
        }

        pub fn parse_from(data: &[u8]) -> Message {
            let header: Header =
                Header::parse_from(data.get(..12).and_then(|s| s.try_into().ok()).expect(
                    "data array length is less than 12 (12 bytes is the size of DNS header).",
                ));
            let payload = &data[12..];
            let (questions, answers) = Message::parse_questions_and_answers(payload, &header);

            Message {
                header: Rc::new(header),
                questions: questions,
                answers: answers,
            }
        }

        fn parse_label_sequence(
            data: &[u8],
            label_sequence_start_index: usize,
        ) -> (Rc<LabelSequence>, usize) {
            let mut labels: Vec<Label> = Vec::new();
            let mut compressed_label_index: usize = 0;
            let mut current_index: usize = label_sequence_start_index;
            let mut null_byte_found = false;
            while current_index < data.len() {
                let control_byte: u8 = data[current_index];
                match control_byte {
                    0 => {
                        null_byte_found = true;
                        break;
                    }
                    /* uncompressed label */
                    1..0xC0 => {
                        let label_length: usize = control_byte as usize;
                        let content = String::from_utf8(
                            data[(current_index + 1)..=(current_index + label_length)].to_vec(),
                        )
                        .expect("Failed to read label's content");
                        labels.push(Label {
                            content: content.into(),
                        });
                        current_index += label_length + 1;
                    }
                    /* compressed label */
                    0xC0..=0xFF => {
                        compressed_label_index = current_index;
                        // We have to subtract 12, as the compressed offset is relative to the entire message's byte array,
                        // and 'data' is a slice of it without the header bytes.
                        let offset_index: u16 = ((((control_byte & 0x3F) as u16) << 8)
                            | data[current_index + 1] as u16)
                            - 12;
                        current_index = offset_index as usize;
                    }
                }
            }

            assert!(null_byte_found,
                "Could not parse label sequence starting from index #{}: end of data was reached but no null-byte was found.", 
                label_sequence_start_index);

            let label_sequence_end_index: usize = if compressed_label_index == 0 {
                current_index
            } else {
                compressed_label_index + 1
            };
            let length: usize = (label_sequence_end_index - label_sequence_start_index) + 1;

            (
                Rc::new(LabelSequence {
                    labels: labels.into(),
                }),
                length,
            )
        }

        fn parse_question_section(
            data: &[u8],
            expected_questions_count: u16,
        ) -> (Rc<[Question]>, usize) {
            let mut questions_count: u16 = 0;
            let mut current_index: usize = 0;
            let mut questions: Vec<Question> = Vec::new();
            while current_index < data.len() && questions_count < expected_questions_count {
                let (label_sequence, label_sequence_length) =
                    Message::parse_label_sequence(data, current_index);
                current_index += label_sequence_length;

                let r#type = ((data[current_index] as u16) << 8) | (data[current_index + 1] as u16);
                current_index += 2;

                let class = ((data[current_index] as u16) << 8) | (data[current_index + 1] as u16);
                current_index += 2;

                questions.push(Question {
                    name: label_sequence,
                    r#type: r#type,
                    class: class,
                });
                questions_count += 1;
            }

            assert!(
                questions_count == expected_questions_count,
                "Expected to have {} questions but was able to parse {}.",
                expected_questions_count,
                questions_count
            );

            (questions.into(), current_index)
        }

        fn parse_answer_section(
            data: &[u8],
            section_start_index: usize,
            expected_answers_count: u16,
        ) -> (Rc<[Answer]>, usize) {
            let mut answers_count: u16 = 0;
            let mut current_index: usize = section_start_index;
            let mut answers: Vec<Answer> = Vec::new();
            while current_index < data.len() && answers_count < expected_answers_count {
                let (label_sequence, label_sequence_length) =
                    Message::parse_label_sequence(data, current_index);
                current_index += label_sequence_length;

                let r#type: u16 =
                    ((data[current_index] as u16) << 8) | (data[current_index + 1] as u16);
                current_index += 2;

                let class: u16 =
                    ((data[current_index] as u16) << 8) | (data[current_index + 1] as u16);
                current_index += 2;

                let ttl: u32 = ((data[current_index] as u32) << 24)
                    | ((data[current_index + 1] as u32) << 16)
                    | ((data[current_index + 2] as u32) << 8)
                    | (data[current_index + 3] as u32);
                current_index += 4;

                let data_length: usize = (((data[current_index] as u16) << 8)
                    | (data[current_index + 1] as u16))
                    as usize;
                current_index += 2;

                answers.push(Answer {
                    name: label_sequence,
                    r#type: r#type,
                    class: class,
                    ttl: ttl,
                    data: data[current_index..(current_index + data_length)].into(),
                });
                current_index += data_length;
                answers_count += 1;
            }

            assert!(
                answers_count == expected_answers_count,
                "Expected to have {} answers but was able to parse {}.",
                expected_answers_count,
                answers_count
            );

            (answers.into(), current_index)
        }

        fn parse_questions_and_answers(
            data: &[u8],
            header: &Header,
        ) -> (Rc<[Question]>, Rc<[Answer]>) {
            let (qd, question_section_end_index) =
                Message::parse_question_section(data, header.get_qd_count());
            let (an, _) = Message::parse_answer_section(
                data,
                question_section_end_index,
                header.get_an_count(),
            );
            (qd, an)
        }
    }

    impl fmt::Display for Message {
        // Example
        // codecrafters.io.     3600    IN      A       76.76.21.21
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let header = &self.header;

            let questions: Vec<String> = self
                .questions
                .iter()
                .map(|question| question.to_string())
                .collect();
            let question_section = format!("QUESTION SECTION:\n;; {}", questions.join("\n;;"));

            let answers: Vec<String> = self.answers.iter().map(Answer::to_string).collect();
            let answer_section = format!("ANSWER SECTION:\n;; {}", answers.join("\n;; "));

            write!(f, "{header}\n;\n;; {question_section}\n;; {answer_section}")
        }
    }
}
