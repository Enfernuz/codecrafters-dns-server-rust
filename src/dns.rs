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

    #[derive(Clone, Debug)]
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

    impl fmt::Display for LabelSequence {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let parts: Vec<&str> = self.labels.iter().map(Label::get_content).collect();
            write!(f, "{}", parts.join("."))
        }
    }

    #[derive(Clone, Debug)]
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

    #[derive(Debug)]
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

        pub fn get_ttl(&self) -> u32 {
            self.ttl
        }

        pub fn set_ttl(&mut self, ttl: u32) {
            self.ttl = ttl;
        }

        pub fn get_length(&self) -> u16 {
            self.data.len() as u16
        }

        pub fn get_data(&'_ self) -> &'_ Vec<u8> {
            &self.data
        }

        pub fn set_data(&'_ mut self, data: Vec<u8>) {
            self.data = data;
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
        questions: Rc<Vec<Question>>,
        answers: Vec<Answer>,
    }

    impl Message {
        pub fn new(
            header: &Rc<Header>,
            questions: &Rc<Vec<Question>>,
            answers: Vec<Answer>,
        ) -> Message {
            Message {
                header: Rc::clone(header),
                questions: Rc::clone(questions),
                answers: answers,
            }
        }

        pub fn get_header(&'_ self) -> &'_ Rc<Header> {
            &self.header
        }

        pub fn get_questions(&'_ self) -> &'_ Rc<Vec<Question>> {
            &self.questions
        }

        pub fn get_answers(&'_ self) -> &'_ Vec<Answer> {
            &self.answers
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
                header: Rc::new(header),
                questions: Rc::new(questions),
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
                let mut labels: Vec<Label> = Vec::new();
                while data[index] != b'\0' {
                    if data[index] & 0xC0 == 0xC0 {
                        let offset_index: u16 =
                            ((((data[index] & 0x3F) as u16) << 8) | data[index + 1] as u16) - 12;
                        index = offset_index as usize;
                        continue;
                    } else {
                        let content = String::from_utf8(
                            data[(index + 1)..=(index + data[index] as usize)].to_vec(),
                        )
                        .expect("Failed to read label's content");
                        labels.push(Label { content: content });
                        index += data[index] as usize + 1;
                    }
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

            // Parse the Answer section
            let expected_answers_count = header.get_an_count();
            let mut answers_count: u16 = 0;
            let mut answers: Vec<Answer> = Vec::new();

            while answers_count < expected_answers_count {
                let mut labels: Vec<Label> = Vec::new();
                while data[index] != b'\0' {
                    if data[index] & 0xC0 == 0xC0 {
                        let offset_index: u16 =
                            (((data[index] & 0x3F) as u16) << 8) | data[index + 1] as u16 - 12;
                        index = offset_index as usize;
                        continue;
                    } else {
                        let content = String::from_utf8(
                            data[(index + 1)..=(index + data[index] as usize)].to_vec(),
                        )
                        .expect("Failed to read label's content");
                        labels.push(Label { content: content });
                        index += data[index] as usize + 1;
                    }
                }
                index += 1;
                let r#type = ((data[index] as u16) << 8) | (data[index + 1] as u16);
                index += 2;
                let class = ((data[index] as u16) << 8) | (data[index + 1] as u16);
                index += 2;
                let ttl: u32 = ((data[index] as u32) << 24)
                    | ((data[index + 1] as u32) << 16)
                    | ((data[index + 2] as u32) << 8)
                    | (data[index + 3] as u32);
                index += 4;
                let length = ((data[index] as u16) << 8) | (data[index + 1] as u16);
                index += 2;
                let data: Vec<u8> = data[index..(index + length as usize)].to_vec();
                index += length as usize;

                answers.push(Answer {
                    name: LabelSequence { labels: labels },
                    r#type: r#type,
                    class: class,
                    ttl: ttl,
                    data: data,
                });
                answers_count += 1;
            }

            (questions, answers)
        }
    }

    impl fmt::Display for Message {
        // Example
        // codecrafters.io.     3600    IN      A       76.76.21.21
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let header = &self.header;

            let questions: Vec<String> = self.questions.iter().map(Question::to_string).collect();
            let question_section = format!("QUESTION SECTION:\n;; {}", questions.join("\n;;"));

            let answers: Vec<String> = self.answers.iter().map(Answer::to_string).collect();
            let answer_section = format!("ANSWER SECTION:\n;; {}", answers.join("\n;;"));

            write!(f, "{header}\n;\n;; {question_section}\n;; {answer_section}")
        }
    }
}
