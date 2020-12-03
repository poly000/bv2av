//!
//!  This is a convertor for bilibili's video ID.
//!
//!  For simplier usage, you can use enc(u64) -> String and
//!     dec(&str) -> u64 instead of BiliAv & BiliBv.
//!
//!

#![feature(const_fn)]

use std::convert::From;
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc_works() {
        assert_eq!(BiliAv(170001).enc().get(), "17x411w7KC");
        assert_eq!(BiliAv(314).enc().get(), "1xx411c7XW");

        assert_eq!(enc(314), "1xx411c7XW".to_owned());
    }

    #[test]
    fn dec_works() {
        assert_eq!(BiliBv("17x411w7KC".to_string()).dec().get(), 170001);
        assert_eq!(BiliBv("1xx411c7XW".to_string()).dec().get(), 314);

        assert_eq!(dec("1xx411c7XW"), 314);
    }

    #[test]
    fn display_works() {
        assert_eq!(
            format!("{}", BiliBv("1xx411c7XW".to_string())),
            "bv1xx411c7XW".to_string()
        );
        assert_eq!(format!("{}", BiliAv(170001)), "av170001".to_string())
    }
}

const TABLE: &[u8; 58] = b"fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const S: [u8; 6] = [9, 8, 1, 6, 2, 4];
const XOR: u64 = 177451812;
const ADD: u64 = 8728348608;

const fn get_tr() -> [u8;256] {
    let mut tr = [0u8; 256];
    tr[TABLE[0] as usize] = 0;
    tr[TABLE[1] as usize] = 1;
    tr[TABLE[2] as usize] = 2;
    tr[TABLE[3] as usize] = 3;
    tr[TABLE[4] as usize] = 4;
    tr[TABLE[5] as usize] = 5;
    tr[TABLE[6] as usize] = 6;
    tr[TABLE[7] as usize] = 7;
    tr[TABLE[8] as usize] = 8;
    tr[TABLE[9] as usize] = 9;
    tr[TABLE[10] as usize] = 10;
    tr[TABLE[11] as usize] = 11;
    tr[TABLE[12] as usize] = 12;
    tr[TABLE[13] as usize] = 13;
    tr[TABLE[14] as usize] = 14;
    tr[TABLE[15] as usize] = 15;
    tr[TABLE[16] as usize] = 16;
    tr[TABLE[17] as usize] = 17;
    tr[TABLE[18] as usize] = 18;
    tr[TABLE[19] as usize] = 19;
    tr[TABLE[20] as usize] = 20;
    tr[TABLE[21] as usize] = 21;
    tr[TABLE[22] as usize] = 22;
    tr[TABLE[23] as usize] = 23;
    tr[TABLE[24] as usize] = 24;
    tr[TABLE[25] as usize] = 25;
    tr[TABLE[26] as usize] = 26;
    tr[TABLE[27] as usize] = 27;
    tr[TABLE[28] as usize] = 28;
    tr[TABLE[29] as usize] = 29;
    tr[TABLE[30] as usize] = 30;
    tr[TABLE[31] as usize] = 31;
    tr[TABLE[32] as usize] = 32;
    tr[TABLE[33] as usize] = 33;
    tr[TABLE[34] as usize] = 34;
    tr[TABLE[35] as usize] = 35;
    tr[TABLE[36] as usize] = 36;
    tr[TABLE[37] as usize] = 37;
    tr[TABLE[38] as usize] = 38;
    tr[TABLE[39] as usize] = 39;
    tr[TABLE[40] as usize] = 40;
    tr[TABLE[41] as usize] = 41;
    tr[TABLE[42] as usize] = 42;
    tr[TABLE[43] as usize] = 43;
    tr[TABLE[44] as usize] = 44;
    tr[TABLE[45] as usize] = 45;
    tr[TABLE[46] as usize] = 46;
    tr[TABLE[47] as usize] = 47;
    tr[TABLE[48] as usize] = 48;
    tr[TABLE[49] as usize] = 49;
    tr[TABLE[50] as usize] = 50;
    tr[TABLE[51] as usize] = 51;
    tr[TABLE[52] as usize] = 52;
    tr[TABLE[53] as usize] = 53;
    tr[TABLE[54] as usize] = 54;
    tr[TABLE[55] as usize] = 55;
    tr[TABLE[56] as usize] = 56;
    tr[TABLE[57] as usize] = 57;
    tr
}
const TR:[u8;256] = get_tr();
#[derive(Copy, Clone, Debug)]
pub struct BiliAv(pub u64);

#[derive(Clone, Debug)]
pub struct BiliBv(pub String);

impl BiliBv {
    /// Get BV Id ( NO 'bv' INCLUDED! ) :
    ///
    /// ```
    /// use bv2av::*;
    /// assert_eq!(BiliBv::from("17x411w7KC").get(),
    ///             "17x411w7KC");
    /// ```
    ///
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Decode Bv
    ///
    /// ```
    /// use bv2av::*;
    ///
    /// assert_eq!(BiliBv::from("1xx411c7XW").dec().get(), 314);
    /// ```
    ///
    pub fn dec(&self) -> BiliAv {
        let str = self.get();
        BiliAv(dec(str))
    }
}

impl std::fmt::Display for BiliBv {
    /// Will add 'bv'
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bv{}", self.get())
    }
}

impl std::fmt::Display for BiliAv {
    /// Will add 'av'
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "av{}", self.get())
    }
}

impl From<u64> for BiliAv {
    fn from(av_num: u64) -> Self {
        BiliAv(av_num)
    }
}

impl From<&str> for BiliBv {
    fn from(str: &str) -> Self {
        BiliBv(str.to_string())
    }
}

impl BiliAv {
    /// Get AV Number:
    ///
    /// ```
    /// use bv2av::BiliAv;
    ///
    /// assert_eq!(BiliAv(170001).get()
    ///     , 170001);
    ///
    /// ```

    pub fn get(&self) -> u64 {
        self.0
    }

    /// Encode Bv
    ///
    /// ```
    /// use bv2av::{BiliAv,BiliBv};
    ///
    /// assert_eq!(BiliAv(170001).enc().get(), "17x411w7KC");
    /// ```

    pub fn enc(&self) -> BiliBv {
        let num = self.0;
        BiliBv(enc(num))
    }
}

pub fn dec(str: &str) -> u64 {
    let b = str.as_bytes();
    ((0..6)
        .map(|i| {
            let index = S[i as usize] as usize;
            let index = b[index];
            TR[index as usize] as u64 * 58u64.pow(i)
        })
        .sum::<u64>()
        - ADD)
        ^ XOR
}

pub fn enc(num: u64) -> String {
    let num = (num ^ XOR) + ADD;
    let mut str = String::from("1  4 1 7  ");
    let b = unsafe { str.as_bytes_mut() };
    for i in 0..6 {
        let index = S[i as usize] as usize;
        b[index] = TABLE[(num / 58u64.pow(i) % 58) as usize];
    }
    str
}
