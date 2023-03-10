use crate::checksum::Checksum;

pub struct BSDSum {
    sum: u32,
}

impl BSDSum {
    pub fn new() -> BSDSum {
        BSDSum { sum: 0 }
    }
}

impl Checksum for BSDSum {
    fn checksum(&self) -> u32 {
        self.sum
    }

    fn update(&mut self, data: &[u8]) -> usize {
        let mut s: u32 = self.sum;
        for &b in data {
            s = (s >> 1) | ((s & 1) << 15);
            s += b as u32;
            s &= 0xffff;
        }

        self.sum = s;
        return data.len();
    }
}

#[cfg(test)]
mod bsd_sum_tests {
    use super::Checksum;
    use super::BSDSum;

    #[test]
    fn test_bsdsum() {
        let mut calc = BSDSum::new();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 4290);
    }
}
