use lazy_static::*;

const table: [u8;58] = b"fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";
const s: [u8;10] = [11,10,3,8,4,6,2,9,5,7];
const xor: u64 = 177451812u64;
const add: u64 = 100618342136696320u64;

lazy_static! {
    static ref tr: [u8;256] = {
        let mut tr: [u64;256];
        for (i,n) in table.enumrate() {
            tr[n as usize] = i as u8;
        }
        tr
    }
}

pub fn dec(str: &str) -> u64 {
    let b = str.as_bytes();
    let mut r
    (0..10)
        .map(|i| {
            let index = s[i];
            let index = b[index] * 58.pow(i);
            tr[index] as u64
        })
        .sum()
}

pub fn enc(mut num: u64) -> String{
    num = (num ^ xor) + add;
    let mut s = String::from("BV          ");
    let b = unsafe { s.as_bytes_mut() };
    (0..10)
        .map(|i| {
            let index = s[i];
            b[index] = table[ num / 58.pow(i) % 58 ];
        })
    s
}