#[derive(Debug)]
struct Packet {
    version: u8,
    data: PacketData
}

impl Packet {
    pub fn parse(bits: &[u8]) -> Packet {
        let version = to_usize(&bits[0..3]) as u8;
        let data = match PacketTypeId::from_byte(to_usize(&bits[3..6]) as u8) {
            PacketTypeId::LITERAL => PacketData::LITERAL(LiteralPacket::parse(&bits[6..])),
            PacketTypeId::OPERATOR => PacketData::OPERATOR(OperatorPacket::parse(&bits[6..])),
        };
        return Packet { version: version, data: data};
    }

    pub fn len(&self) -> usize {
        return 6 + self.data.len();
    }
}

#[derive(Debug, PartialEq)]
enum PacketTypeId {
    LITERAL,
    OPERATOR,
}

impl PacketTypeId {
    pub fn from_byte(byte: u8) -> PacketTypeId {
        match byte {
            4 => PacketTypeId::LITERAL,
            _ => PacketTypeId::OPERATOR,
        }
    }
}

#[derive(Debug)]
enum PacketData {
    LITERAL(LiteralPacket),
    OPERATOR(OperatorPacket),
}

impl PacketData {
    pub fn len(&self) -> usize {
        return match self {
            PacketData::LITERAL(packet) => packet.len(),
            PacketData::OPERATOR(packet) => packet.len(),
        }
    }
}

#[derive(Debug)]
struct LiteralPacket {
    value: u16,
    len: usize,
}

#[derive(Debug)]
struct OperatorPacket {
    header_len: usize,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    pub fn parse(bits: &[u8]) -> OperatorPacket {
        let mut subpackets: Vec<Packet> = vec!();
        let mut packet: Packet;
        let mut offset: usize;
        let header_len: usize;

        if bits[0] == 0 {
            header_len = 16;
            offset = header_len;
            let len_subpackets = to_usize(&bits[1..16]);
            while offset < len_subpackets + 16 {
                packet = Packet::parse(&bits[offset..]);
                offset += packet.len();
                subpackets.push(packet);
            }
        } else {
            if bits[0] != 1 {
                panic!("Bad Operator Type Length");
            }
            header_len = 12;
            offset = header_len;
            let subpacket_count = to_usize(&bits[1..12]);
            for _ in 0..subpacket_count {
                packet = Packet::parse(&bits[offset..]);
                offset += packet.len();
                subpackets.push(packet);
            }
        }
        return OperatorPacket {subpackets: subpackets, header_len: header_len};
    }

    pub fn len(&self) -> usize {
        return self.header_len + self.subpackets.iter().map(|x| x.len()).sum::<usize>();
    }
}

impl LiteralPacket {
    pub fn parse(bits: &[u8]) -> LiteralPacket {
        let mut idx: usize = 0;
        let mut value: u16 = 0;
        let mut len: usize = 0;

        loop {
            len += 5;
            value <<= 4;
            value += to_usize(&bits[idx+1..idx+5]) as u16;
            if bits[idx] == 0 { break; }
            idx += 5;
        } 
        return LiteralPacket {value: value, len: len};
    }

    pub fn len(&self) -> usize {
        return self.len;
    }
}

fn to_bits(byte: &u8) -> [u8; 4] {
    let mut remain = *byte;
    let mut bits: [u8; 4] = [0; 4];

    for idx in (0..4).rev() {
        bits[idx] = remain % 2;
        remain /= 2;
        if remain == 0 { break; }
    }
    return bits;
}

fn version_sum(packet: &Packet) -> usize {
    let sum: usize = packet.version as _;
    return match &packet.data {
        PacketData::LITERAL(_) => sum,
        PacketData::OPERATOR(operator) => {
            sum + (operator
            .subpackets
            .iter()
            .map(|x| version_sum(x))
            .sum::<usize>())
        }
    };
}

fn run(input: &'static str) -> usize {
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len() * 4);
    for chr in input.chars() {
        let value = u8::from_str_radix(&String::from(chr), 16).unwrap();
        bytes.append(&mut to_bits(&value).to_vec());
    }

    let packet = Packet::parse(bytes.as_slice());
    
    return version_sum(&packet);
}

fn to_usize(bits: &[u8]) -> usize {
    return bits
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, bit)| bit > &&0)
        .map(|(idx, _)| 2usize.pow(idx as _) as usize)
        .sum()
}

fn main() {
    println!("{}", run(include_str!("../input.txt").lines().next().unwrap()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(run(include_str!("../test.txt").lines().next().unwrap()), 16);
    }

    #[test]
    fn test2() {
        assert_eq!(run(include_str!("../test.txt").lines().skip(1).next().unwrap()), 12);
    }

    #[test]
    fn test3() {
        assert_eq!(run(include_str!("../test.txt").lines().skip(2).next().unwrap()), 23);
    }

    #[test]
    fn test4() {
        assert_eq!(run(include_str!("../test.txt").lines().skip(3).next().unwrap()), 31);
    }
}