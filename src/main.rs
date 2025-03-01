use std::fmt;

const SBOX: [u8; 16] = [0x9, 0x4, 0xa, 0xb, 0xd, 0x1, 0x8, 0x5, 0x6, 0x2, 0x0, 0x3, 0xc, 0xe, 0xf, 0x7];
// const INV_SBOX: [u8; 16] = [0xa, 0x5, 0x9, 0xb, 0x1, 0x7, 0x8, 0xf, 0x6, 0x0, 0x2, 0x3, 0xc, 0x4, 0xd, 0xe];

#[derive(Clone)]
struct State([[u8; 2]; 2]);

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:x}, {:x}]\n[{:x}, {:x}]", self.0[0][0], self.0[0][1], self.0[1][0], self.0[1][1])
    }
}

impl State {
    fn new(bytes: [u8; 4]) -> Self {
        State([[bytes[0], bytes[1]], [bytes[2], bytes[3]]])
    }

    fn sub_bytes(&mut self) {
        for row in self.0.iter_mut() {
            for byte in row.iter_mut() {
                *byte = SBOX[*byte as usize & 0x0F];
            }
        }
    }

    fn shift_rows(&mut self) {
        self.0[1].swap(0, 1);
    }

    fn mix_columns(&mut self) {
        let a = self.0[0][0];
        let b = self.0[1][0];
        self.0[0][0] = a ^ b;
        self.0[1][0] ^= a;
        let a = self.0[0][1];
        let b = self.0[1][1];
        self.0[0][1] = a ^ b;
        self.0[1][1] ^= a;
    }

    fn add_round_key(&mut self, key: &State) {
        for i in 0..2 {
            for j in 0..2 {
                self.0[i][j] ^= key.0[i][j];
            }
        }
    }
}

fn encrypt(plaintext: [u8; 4], key: [u8; 4]) -> State {
    let mut state = State::new(plaintext);
    let round_key = State::new(key);

    state.add_round_key(&round_key);
    state.sub_bytes();
    state.shift_rows();
    state.mix_columns();
    state.add_round_key(&round_key);

    state
}

fn main() {
    let plaintext = [0x01, 0x02, 0x03, 0x04];
    let key = [0x0a, 0x0b, 0x0c, 0x0d];
    let ciphertext = encrypt(plaintext, key);
    println!("Ciphertext:\n{:?}", ciphertext);
}