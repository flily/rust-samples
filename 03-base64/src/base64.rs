
static BASE64_CHARS: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub fn base64_encode(input: &[u8]) -> Vec<u8> {
    let input_length: usize = input.len();
    let output_blocks: usize = input_length / 3;
    let output_remain: usize = input_length % 3;
    let output_length: usize = if output_remain == 0 {
        output_blocks * 4
    } else {
        output_blocks * 4 + 4
    };

    let mut output: Vec<u8> = vec![0; output_length];
    for i in 0..(output_blocks + 1) {
        if 3 * i >= input_length {
            break;
        }
        let mut last = 0;

        let i0 = input[3 * i + 0];
        let i1 = if 3 * i + 1 >= input_length {
            last += 1;
            0
        } else {
            input[3 * i + 1]
        };
        let i2 = if 3 * i + 2 >= input_length {
            last += 1;
            0
        } else {
            input[3 * i + 2]
        };

        // |     0     |     1     |     2     |     3     |
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |0|1|2|3|4|5|6|7|0|1|2|3|4|5|6|7|0|1|2|3|4|5|6|7|
        // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        // |       0       |       1       |       2       |
        let o0 = i0 >> 2;
        let o1 = ((i0 & 0x03) << 4) | ((i1 & 0xf0) >> 4);
        let o2 = ((i1 & 0x0f) << 2) | ((i2 & 0xc0) >> 6);
        let o3 = i2 & 0x3f;

        output[4 * i + 0] = BASE64_CHARS[o0 as usize];
        output[4 * i + 1] = BASE64_CHARS[o1 as usize];
        output[4 * i + 2] = BASE64_CHARS[o2 as usize];
        output[4 * i + 3] = BASE64_CHARS[o3 as usize];
        if last > 0 {
            output[4 * i + 3] = '=' as u8;
            if last > 1 {
                output[4 * i + 2] = '=' as u8;
            }
        }
    }

    output.into()
}


static BASE64_DECODE_MAP: &[u8] = &[
    // 0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x00 - 0x0f
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x10 - 0x1f
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x3e, 0x00, 0x3f, // 0x20 - 0x2f
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x30 - 0x3f
    0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, // 0x40 - 0x4f
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x00, 0x00, 0x00, 0x00, 0x3e, // 0x50 - 0x5f
    0x00, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, // 0x60 - 0x6f
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x70 - 0x7f
];

pub fn base64_decode(input: &[u8]) -> Vec<u8> {
    let input_length : usize = input.len();
    let output_blocks: usize = input_length / 4;
    let output_length: usize = if input[input_length - 2] == '=' as u8 {
        output_blocks * 3 - 2
    } else if input[input_length - 1] == '=' as u8 {
        output_blocks * 3 - 1
    } else {
        output_blocks * 3
    };

    let mut output = vec![0; output_length];
    for i in 0..output_blocks {
        let c0 = input[4 * i + 0];
        let c1 = input[4 * i + 1];
        let c2 = input[4 * i + 2];
        let c3 = input[4 * i + 3];

        let i0 = BASE64_DECODE_MAP[c0 as usize];
        let i1 = BASE64_DECODE_MAP[c1 as usize];
        let i2 = BASE64_DECODE_MAP[c2 as usize];
        let i3 = BASE64_DECODE_MAP[c3 as usize];

        let o0 = (i0 << 2) | (i1 >> 4);
        let o1 = ((i1 & 0x0f) << 4) | (i2 >> 2);
        let o2 = ((i2 & 0x03) << 6) | i3;

        output[3 * i + 0] = o0;
        if c2 != '=' as u8{
            output[3 * i + 1] = o1;
        }

        if c3 != '=' as u8{
            output[3 * i + 2] = o2;
        }
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let case = vec![0x00, 0x00, 0x00];
        let got = base64_encode(&case);
        let expected = "AAAA".as_bytes();
        assert_eq!(got, expected);

        let case = vec![0x00, 0x00];
        let got = base64_encode(&case);
        let expected = "AAA=".as_bytes();
        assert_eq!(got, expected);

        let case = vec![0x00];
        let got = base64_encode(&case);
        let expected = "AA==".as_bytes();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_base64_decode() {
        let case = "AAAA".as_bytes();
        let got = base64_decode(&case);
        let expected = vec![0x00, 0x00, 0x00];
        assert_eq!(got, expected);

        let case = "AAA=".as_bytes();
        let got = base64_decode(&case);
        let expected = vec![0x00, 0x00];
        assert_eq!(got, expected);

        let case = "AA==".as_bytes();
        let got = base64_decode(&case);
        let expected = vec![0x00];
        assert_eq!(got, expected);
    }
}