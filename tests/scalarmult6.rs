extern crate tweetnaclrs;

use tweetnaclrs::crypto::scalarmult;

static BOBSK: [u8; 32] = [
    0x5d,0xab,0x08,0x7e,0x62,0x4a,0x8a,0x4b,
    0x79,0xe1,0x7f,0x8b,0x83,0x80,0x0e,0xe6,
    0x6f,0x3b,0xb1,0x29,0x26,0x18,0xb6,0xfd,
    0x1c,0x2f,0x8b,0x27,0xff,0x88,0xe0,0xeb
];

static ALICEPK: [u8; 32] = [
    0x85,0x20,0xf0,0x09,0x89,0x30,0xa7,0x54,
    0x74,0x8b,0x7d,0xdc,0xb4,0x3e,0xf7,0x5a,
    0x0d,0xbf,0x3a,0x0d,0x26,0x38,0x1a,0xf4,
    0xeb,0xa4,0xa9,0x8e,0xaa,0x9b,0x4e,0x6a
];

static EXPECTED: [u8; scalarmult::BYTES] = [
    0x4a,0x5d,0x9d,0x5b,0xa4,0xce,0x2d,0xe1,
    0x72,0x8e,0x3b,0xf4,0x80,0x35,0x0f,0x25,
    0xe0,0x7e,0x21,0xc9,0x47,0xd1,0x9e,0x33,
    0x76,0xf0,0x9b,0x3c,0x1e,0x16,0x17,0x42
];

#[test]
fn scalarmult6() {
    assert_eq!(EXPECTED, scalarmult::scalarmult(&BOBSK, &ALICEPK));
}
