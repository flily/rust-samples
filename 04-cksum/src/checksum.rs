
mod checksum;

pub use checksum::Checksum;

mod bsd;

pub use bsd::BSDSum;

// BSD checksum, option 1
pub fn create_bsd_checksum() -> Box<dyn Checksum> {
    let calc = BSDSum::new();
    return Box::new(calc);
}

#[cfg(test)]
mod bsd_sum_tests {
    use super::create_bsd_checksum;

    #[test]
    fn test_bsdsum() {
        let mut calc = create_bsd_checksum();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 4290);
    }
}

mod att;

pub use att::ATTSum;

// ATT checksum, option 2
pub fn create_att_checksum() -> Box<dyn Checksum> {
    let calc = ATTSum::new();
    return Box::new(calc);
}

#[cfg(test)]
mod att_sum_tests {
    use super::create_att_checksum;

    #[test]
    fn test_attsum() {
        let mut calc = create_att_checksum();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 495);
    }
}

mod iso;

pub use iso::ISOSum;

// ISO checksum, option 3
pub fn create_iso_checksum() -> Box<dyn Checksum> {
    let calc = ISOSum::new();
    return Box::new(calc);
}

#[cfg(test)]
mod iso_sum_tests {
    use super::create_iso_checksum;

    #[test]
    fn test_isosum() {
        let mut calc = create_iso_checksum();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 2240272485);
    }
}

mod posix;

pub use posix::PosixSum;

// POSIX checksum, default option
pub fn create_posix_checksum() -> Box<dyn Checksum> {
    let calc = PosixSum::new();
    return Box::new(calc);
}

#[cfg(test)]
mod posix_sum_tests {
    use super::create_posix_checksum;

    #[test]
    fn test_posixsum() {
        let mut calc = create_posix_checksum();
        let data:[u8; 5] = [0x61, 0x62, 0x63, 0x64, 0x65];
        calc.update(&data);
        let sum = calc.checksum();
        assert_eq!(sum, 996742021);
    }
}
