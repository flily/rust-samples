use crate::checksum::Checksum;

pub struct ATTSum {
    sum: u64,
}

impl ATTSum {
    pub fn new() -> ATTSum {
        ATTSum { sum: 0 }
    }
}

impl Checksum for ATTSum {
    fn checksum(&self) -> u32 {
        let lo: u64 = self.sum & 0x0000ffff;
        let hi: u64 = (self.sum & 0xffff0000) >> 16;
        let r: u64 = lo + hi;
        ((r & 0x0000ffff) + (r >> 16)) as u32
    }

    fn update(&mut self, data: &[u8]) -> usize{
        let mut s: u64 = 0;
        for &b in data {
            s += b as u64;
        }
        self.sum += s;
        return data.len();
    }
}

#[cfg(test)]
mod att_sum_tests {
    use super::Checksum;
    use super::ATTSum;

    #[test]
    fn test_attsum() {
        let mut calc = ATTSum::new();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 495);
    }
}
