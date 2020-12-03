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
const TR:[u8;256] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 13, 12, 46, 31, 43, 18, 40, 28, 5, 0, 0, 0, 0, 0, 0, 0, 54, 20, 15, 8, 39, 57, 45, 36, 0, 38, 51, 42, 49, 52, 0, 53, 7, 4, 9, 50, 10, 44,
34, 6, 25, 1, 0, 0, 0, 0, 0, 0, 26, 29, 56, 3, 24, 0, 47, 27, 22, 41, 16, 0, 11, 37, 2, 35, 21, 17, 33, 30, 48, 23, 55, 32, 14, 19, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
