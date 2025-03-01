# Toy AES in Rust

A simplified implementation of the Advanced Encryption Standard (AES) in Rust, designed for educational purposes. This toy version uses a 4-byte (2x2 matrix) block size and a single round, mimicking AES's core transformations: SubBytes, ShiftRows, MixColumns, and AddRoundKey.

## Features
- **Block Size:** 4 bytes (2x2 matrix) instead of AES's 16 bytes.
- **Key Size:** 4 bytes, expanded implicitly via AddRoundKey.
- **Rounds:** 1 main round + initial key addition (real AES uses 10+ rounds).
- **Output:** Encrypts a 4-byte plaintext into a 4-byte ciphertext.

## Example
```plaintext
Plaintext: [0x01, 0x02, 0x03, 0x04]
Key: [0x0a, 0x0b, 0x0c, 0x0d]
Ciphertext:
[0xb, 0xe]
[0xd, 0x8]
```

## Prerequisites
- **Rust:** Install via `rustup` (requires `rustc` and `cargo`).
- **No external crates needed**—uses only the Rust standard library.

## Setup
### Clone or Create the Project:
```bash
cargo new toy-aes
cd toy-aes
```

### Add the Code:
Replace `src/main.rs` with the implementation below.

### Build:
```bash
cargo build
```

## Usage
### Run the Program:
```bash
cargo run
```
Outputs the ciphertext in a 2x2 matrix format (hex values).

### Modify Inputs:
Edit plaintext and key in `main()` to test different values:
```rust
let plaintext = [0x05, 0x06, 0x07, 0x08];
let key = [0x01, 0x02, 0x03, 0x04];
```
Re-run with `cargo run`.

## Implementation Details
- **State:** A 2x2 matrix (`[[u8; 2]; 2]`) representing 4 bytes.
- **Transformations:**
  - **SubBytes:** Uses a simplified 16-entry S-box.
  - **ShiftRows:** Swaps bytes in the second row.
  - **MixColumns:** Basic XOR-based mixing (not full GF(2^8)).
  - **AddRoundKey:** XORs state with the key.

### Limitations:
- Not cryptographically secure—use `aes` crate for real-world needs.
- Single round, no key expansion, tiny block size.


## Future Improvements
- Add decryption using `INV_SBOX`.
- Expand to 128-bit blocks and multiple rounds.
- Implement proper GF(2^8) arithmetic for MixColumns.
- Add tests with `cargo test`.

## License
This project is for educational use and has no formal license. Feel free to adapt and share!

## Acknowledgments
- Inspired by AES (Rijndael) specifications.

