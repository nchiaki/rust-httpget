
// ENCODE

const BASE64_ENCODE_TABLE : &[u8] = concat!(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwxyz",
    "0123456789+/",
).as_bytes();

const PADDING: u8 = b'=';


fn into4bit6(byte1: u8, byte2: u8, byte3:u8) -> (u8,u8,u8,u8)
{
    let bit32 = u32::from_be_bytes([0,byte1,byte2,byte3]);
    (
        ((bit32 >> 18) & LOW_6_BITS) as u8,
        ((bit32 >> 12) & LOW_6_BITS) as u8,
        ((bit32 >> 6) & LOW_6_BITS) as u8,
        ((bit32 >> 0) & LOW_6_BITS) as u8,
    )
}

fn into3bit6(byte1: u8, byte2: u8) -> (u8,u8,u8)
{
    let bit32 = u32::from_be_bytes([byte1,byte2,0,0]);
    (
        ((bit32 >> 26) & LOW_6_BITS) as u8,
        ((bit32 >> 20) & LOW_6_BITS) as u8,
        ((bit32 >> 14) & LOW_6_BITS) as u8,
    )
}

fn into2bit6(byte1: u8) -> (u8,u8)
{
    let bit32 = u32::from_be_bytes([byte1,0,0,0]);
    (
        ((bit32 >> 26) & LOW_6_BITS) as u8,
        ((bit32 >> 20) & LOW_6_BITS) as u8,
    )
}

fn into_bit6s(bytes: &[u8]) -> Vec<u8>
{
    let mut bit6s = Vec::new();
    let mut rest = bytes;
    loop
    {
        match rest.len()
        {
            0 => {return bit6s;},
            1 => {
                let (bit61, bit62) = into2bit6(rest[0]);
                bit6s.push(bit61);
                bit6s.push(bit62);
                return bit6s;
            },
            2 => {
                let (bit61, bit62, bit63) = into3bit6(rest[0],rest[1]);
                bit6s.push(bit61);
                bit6s.push(bit62);
                bit6s.push(bit63);
                return bit6s;
            },
            _ => {
                let (bit61, bit62, bit63, bit64) = into4bit6(rest[0],rest[1],rest[2]);
                bit6s.push(bit61);
                bit6s.push(bit62);
                bit6s.push(bit63);
                bit6s.push(bit64);
            },
        }
        rest = &rest[3..];
    }
}

pub fn encode<T: AsRef<[u8]>>(input: T) -> String
{
    let bit6s = into_bit6s(input.as_ref());
    let mut encode: Vec<_> = bit6s.into_iter().map(|bit6| {BASE64_ENCODE_TABLE[bit6 as usize]}).collect();

    while encode.len() % 4 != 0
    {encode.push(PADDING);}

    String::from_utf8(encode).unwrap()
}

// DECODE

const BASE64_DECODE_TABLE: [u8; 256] = generate_decode_table_from(&BASE64_ENCODE_TABLE);

const INVALID_VALUE: u8 = 0xff;
const LOW_6_BITS: u32 = 0x3f;

const fn generate_decode_table_from(encode_table: &[u8]) -> [u8; 256]
{
    let mut decode_table = [INVALID_VALUE; 256];
    let mut index = 0;

    while index < 64
    {
        decode_table[encode_table[index] as usize] = index as u8;
        index += 1;
    }
    decode_table
}

fn into3byte(bit61: u8,bit62: u8,bit63: u8,bit64: u8) -> (u8,u8,u8)
{
    (
        (bit61 << 2) + (bit62 >> 4),
        (bit62 << 4) + (bit63 >> 2),
        (bit63 << 6) + (bit64 >> 0),
    )
}
fn into2byte(bit61: u8,bit62: u8,bit63: u8) -> (u8,u8)
{
    (
        (bit61 << 2) + (bit62 >> 4),
        (bit62 << 4) + (bit63 >> 2),
    )
}
fn into1byte(bit61: u8,bit62: u8) -> u8
{
    (bit61 << 2) + (bit62 >> 4)
}

fn into_bytes(bit6s: &[u8]) -> Vec<u8>
{
    if bit6s.len() % 4 == 1
    {panic!("Invalid 6-bits length. {}:{}", file!(), line!());}

    let mut bytes = Vec::new();
    let mut rest = bit6s;

    loop
    {
        match rest.len()
        {
            0 => {return bytes;},
            2 => {
                let byte = into1byte(rest[0],rest[1]);
                bytes.push(byte);
                return bytes;
            },
            3 => {
                let (byte1, byte2) = into2byte(rest[0],rest[1],rest[2]);
                bytes.push(byte1);
                bytes.push(byte2);
                return bytes;
            },
            _ => {
                let (byte1, byte2, byte3) = into3byte(rest[0],rest[1],rest[2],rest[3]);
                bytes.push(byte1);
                bytes.push(byte2);
                bytes.push(byte3);
            },
        }
        rest = &rest[4..];
    }
}

pub fn decode<T:AsRef<[u8]>>(input:T) -> Result<Vec<u8>, DecodeError>
{
    let input = input.as_ref();

    if input.is_empty()
    {return Ok(vec!());}

    if let Err(e) = validate_decoding_target(input)
    {return Err(e);}

    let padding = count_padding(input);
    let bit6s: Vec<_> = input[..input.len() - padding].iter().map(|symbol| {BASE64_DECODE_TABLE[*symbol as usize]}).collect();
    let decoded = into_bytes(bit6s.as_ref());

    Ok(decoded)
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    InvalidLength,
    InvalidByte(usize, u8),
    InvalidLastSymbol(usize, u8),
}

fn validate_decoding_target(input: &[u8]) -> Result<(), DecodeError>
{
    // 空のデータは対象外
    if input.is_empty()
    {return Ok(());}

    // 入力長は４の倍数以外はエラー
    if input.len() % 4 != 0
    {return Err(DecodeError::InvalidLength);}

    // 適切なascii文字か、適切なパディング文字か
    let padding = count_padding(input);
    let invalid_value = input[..input.len()-padding].into_iter().zip(0..input.len()).filter(|(value, _)| {BASE64_DECODE_TABLE[**value as usize] == INVALID_VALUE}).nth(0);
    if let Some((value, index)) = invalid_value {return Err(DecodeError::InvalidByte(index, *value));}

    // パディング有効時、最終ビットの０パディングを確認する
    let last_non_pad_index = input.len() - padding - 1;
    let last_non_pad_elem = input[last_non_pad_index];
    let mask = match padding {
        2 => 0b0000_1111,
        1 => 0b0000_0011,
        _ => 0b0000_0000,
    };
    if BASE64_DECODE_TABLE[last_non_pad_elem as usize] & mask != 0
    {return Err(DecodeError::InvalidLastSymbol(last_non_pad_index, last_non_pad_elem));}

    Ok(())
}

fn count_padding(input: &[u8]) -> usize
{
    let (last, last2) = (input[input.len()-1], input[input.len()-2]);

    if last == PADDING && last2 == PADDING {2}
    else if last == PADDING {1}
    else {0}
}
