use core::num::ParseIntError;

pub const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453595320494249;

#[repr(transparent)]
pub struct Revision {
    val: u32,
}
impl Revision {
    pub const fn new(major: u32, minor: u32) -> Revision {
        Revision {
            val: major << 16 | (minor & 0xff),
        }
    }
    pub fn minor(&self) -> u32 {
        self.val & 0xff
    }
    pub fn major(&self) -> u32 {
        (self.val >> 16) & 0xff
    }
}
impl std::str::FromStr for Revision {
    type Err = ParseIntError;

    fn from_str(rev: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = rev.split(".").collect();

        let major = u32::from_str_radix(v[0], 10)?;

        let mut minor_str: String = String::new();
        (&v[1..]).iter().for_each(|&x| minor_str.push_str(x));
        let minor = u32::from_str_radix(&minor_str, 10)?;

        Ok(Revision {
            val: major << 16 | minor,
        })
    }
}

#[repr(C)]
pub struct EfiTableHeader {
    pub sig: u64,
    pub rev: Revision,
    pub hdr_size: u32,
    pub crc32: u32,
    reserved: u32,
}

impl Default for EfiTableHeader {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::{num::ParseIntError, str::FromStr};

    use super::Revision;

    #[test]
    fn revision_test() -> Result<(), ParseIntError> {
        let a = Revision::from_str("1.2.1")?;
        assert_eq!(a.major(), 1);
        assert_eq!(a.minor(), 21);
        Ok(())
    }
}
