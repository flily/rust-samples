
static BASE64_CHARS: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub fn base64_encode(input: &[u8]) -> Vec<u8> {
    let input_length = input.len();
    let output_blocks = input_length / 3;
    let output_remain = input_length % 3;
    let output_length = if output_remain == 0 {
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
    }
}