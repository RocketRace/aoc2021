// Why not
type Bit = bool;

const MAX_PACKET_DEPTH: usize = 32;
const MAX_CHILD_COUNT: usize = 64;

#[derive(Clone)]
struct BitStream {
    inner: Vec<Bit>,
    position: usize,
    // Stack
    packet_lengths: [usize; MAX_PACKET_DEPTH],
    packet_depth: usize
}

impl BitStream {
    fn new(stream: Vec<Bit>) -> Self {
        Self {
            inner: stream,
            position: 0,
            packet_lengths: [0; MAX_PACKET_DEPTH],
            packet_depth: 1
        }
    }
    fn read(&mut self, count: usize) -> &[Bit] {
        self.consume(count as usize);
        let bits = &self.inner[self.position..self.position + count];
        self.position += count;
        bits
    }

    fn read_bit(&mut self) -> Bit {
        self.read(1)[0]
    }

    fn read_int(&mut self, count: usize) -> usize {
        self.read(count)
            .iter()
            .rev()
            .enumerate()
            .fold(0usize, |n, (i, &bit)| n | (bit as usize) << i as usize)
    }

    fn initialize_packet(&mut self) {
        self.packet_depth += 1;
    }
    
    fn consume(&mut self, bits: usize) {
        self.packet_lengths[self.packet_depth - 1] += bits;
    }
    
    fn consumed(&self) -> usize {
        self.packet_lengths[self.packet_depth - 1]
    }
    
    fn finalize_packet(&mut self) {
        let consumed = self.packet_lengths[self.packet_depth - 1];
        self.packet_lengths[self.packet_depth - 1] = 0;
        self.packet_depth -= 1;
        self.consume(consumed);
    }

    fn close(&mut self) {
        let all_trailing_zeroes = self.inner[self.position..].iter().all(|&b| !b);
        assert!(all_trailing_zeroes);
        self.position = self.inner.len();
    }
}

#[aoc_generator(day16)]
fn generator(input: &str) -> BitStream {
    let stream = input
        .trim()
        .as_bytes()
        .iter()
        .map(|&c| match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'F' => c - b'A' + 10,
            b'a'..=b'f' => c - b'a' + 10,
            _ => unreachable!()
        })
        .flat_map(|c| 
            [(c & 8) >> 3, (c & 4) >> 2, (c & 2) >> 1, c & 1] 
        )
        .map(|bit| bit == 1)
        .collect();
    
    BitStream::new(stream)
}

impl BitStream {
    fn version_packet(&mut self) -> usize {
        self.initialize_packet();
        let mut version_sum = self.read_int(3);
        let type_id = self.read_int(3);

        match type_id {
            // Literal
            4 => {
                while self.read_bit() {
                    // 1-prefixed group
                    self.read_int(4);
                }
                // 0-prefixed group
                self.read_int(4);
            }
            // Operator packets
            _ => {
                if self.read_bit() {
                    let packet_count = self.read_int(11);
                    for _ in 0..packet_count {
                        version_sum += self.version_packet();
                    }
                }
                else {
                    // self.consumed() always returns 22 here
                    let packet_length = self.read_int(15) + self.consumed();
                    while self.consumed() < packet_length {
                        version_sum += self.version_packet();
                    }
                }
            }
        }
        self.finalize_packet();
        version_sum
    }

    fn value_packet(&mut self) -> usize {
        self.initialize_packet();
        self.read_int(3); // version
        let type_id = self.read_int(3);
    
        let value = if type_id == 4 {
            // Literal
            let mut int = 0;
            while self.read_bit() {
                // 1-prefixed group
                let group = self.read_int(4);
                int <<= 4;
                int |= group;
            }
            // 0-prefixed group
            let group = self.read_int(4);
            int <<= 4;
            int |= group;
            int
        }
        // Operator packets
        else {
            let mut values = [0; MAX_CHILD_COUNT];
            let mut length = 0;
            // length type ID
            if self.read_bit() {
                length = self.read_int(11);
                for i in 0..length {
                    values[i] = self.value_packet();
                }
            }
            else {
                // self.consumed() always returns 22 here
                let packet_length = self.read_int(15) + self.consumed();
                // Special case, only 2 packets required
                if type_id >= 5 {
                    values[0] = self.value_packet();
                    values[1] = self.value_packet();
                }
                else {
                    while self.consumed() < packet_length {
                        values[length] = self.value_packet();
                        length += 1;
                    }
                }
            }
            match type_id {
                0 => values[..length].iter().sum(),
                1 => values[..length].iter().product(),
                2 => *values[..length].iter().min().unwrap(),
                3 => *values[..length].iter().max().unwrap(),
                4 => unreachable!(), // literal
                5 => (values[0] > values[1]) as usize,
                6 => (values[0] < values[1]) as usize,
                7 => (values[0] == values[1]) as usize,
                _ => unreachable!()
            }
        };
    
        self.finalize_packet();
        value
    }
}

#[aoc(day16, part1)]
fn version_sums(input: &BitStream) -> usize {
    let mut stream = input.to_owned();
    let ver = stream.version_packet();
    stream.close();
    ver
}

#[aoc(day16, part2)]
fn decode_packet(input: &BitStream) -> usize {
    let mut stream = input.to_owned();
    let value = stream.value_packet();
    stream.close();
    value
}

