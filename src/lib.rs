use lazy_static::*;
use std::fmt;
use std::convert::From;

#[cfg(test)]
mod tests {
    use super::BiliId;

    #[test]
    fn enc_works() {
        assert_eq!(BiliId::Av(170001).enc().get_bv().unwrap(), "17x411w7KC");
        assert_eq!(BiliId::Av(314).enc().get_bv().unwrap(), "1xx411c7XW");
    }

    #[test]
    fn dec_works() {
        assert_eq!(BiliId::Bv("17x411w7KC".to_string()).dec().get_av().unwrap(), 170001);
        assert_eq!(BiliId::Bv("1xx411c7XW".to_string()).dec().get_av().unwrap(), 314);
    }
}

const TABLE: &[u8;58] = b"fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const S: [u8;6] = [9,8,1,6,2,4];
const XOR: u64 = 177451812;
const ADD: u64 = 8728348608;

lazy_static! {
    static ref TR: [u8;256] = {
        let mut tr = [0u8;256];
        for i in 0..58 {
            tr[ TABLE[i] as usize ] = i as u8;
        }
        tr
    };
}


#[derive(Clone,Debug)]
pub enum BiliId {
    Bv(String),
    Av(u64),
}

impl BiliId {
    /// Get AV Number:
    /// 
    /// ```
    /// use bv2av::BiliId;
    /// 
    /// assert_eq!(BiliId::Bv(
    /// "17x411w7KC".to_string())
    ///     .dec().get_av()
    ///     , Some(170001));
    /// 
    /// assert_eq!(BiliId::Bv("17x411w7KC".to_string()).get_av()
    ///     , None);
    /// 
    /// 
    /// ```
    pub fn get_av(&self) -> Option<u64> {
        match self {
            BiliId::Av(n) => Some(*n),
            _ => None,
        }
    }

    /// Get BV Id ( NO 'bv' INCLUDED! ) :
    pub fn get_bv(&self) -> Option<&str> {
        match self {
            BiliId::Bv(s) => Some(s),
            _ => None,
        }
    }

    /// Decode Bv
    /// 
    /// ```
    /// use bv2av::BiliId;
    /// 
    /// assert_eq!(BiliId::from("1xx411c7XW").dec().get_av(), Some(314));
    /// ```
    /// 
    pub fn dec(&self) -> BiliId {
        match self {
            BiliId::Bv(str) => BiliId::Av({
                let b = str.as_bytes();
                ((0..6)
                    .map(|i| {
                        let index = S[i as usize] as usize;
                        let index = b[index];
                        TR[ index as usize ] as u64 * 58u64.pow(i)
                    })
                    .sum::<u64>() - ADD) ^ XOR}),
            _ => self.clone(),
        }
    }

    /// Encode Av
    /// ```
    /// use bv2av::BiliId;
    /// 
    /// assert_eq!(BiliId::from(314).enc().get_bv(), Some("1xx411c7XW"));
    /// ```
    /// 
    pub fn enc(&self) -> BiliId {
        match self {
            BiliId::Av(mut num) => BiliId::Bv({
                num = (num ^ XOR) + ADD;
                let mut str = String::from("1  4 1 7  ");
                let b = unsafe { str.as_bytes_mut() };
                for i in 0..6 {
                    let index = S[i as usize] as usize;
                    b[index] = TABLE[ (num / 58u64.pow(i) % 58) as usize ];
                }
                str
            }),
            _ => self.clone(),
        }
    }

}

impl std::fmt::Display for BiliId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BiliId::Bv(_) => write!(f,"bv{}", self.get_bv().unwrap()),
            BiliId::Av(_) => write!(f,"av{}", self.get_av().unwrap()),
        }
    }
}

impl From<u64> for BiliId {
    fn from(av_num: u64) -> Self {
        BiliId::Av(av_num)
    }
}

impl<'a> From<&str> for BiliId {
    fn from(str: &str) -> Self {
        BiliId::Bv(str.to_string())
    }
}