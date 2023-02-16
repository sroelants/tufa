use sha1::Sha1;
use hmac::{Hmac, Mac};

pub struct Totp<'a> {
    secret: &'a [u8],
    pub max_digits: u32,
    pub reference_time: u64,
    pub window: u64,
}

impl<'a> Totp<'a> {
    pub fn new(secret: &'a str, max_digits: u32, reference_time: u64, window: u64) -> Totp {
        Totp {
            secret: secret.as_bytes(),
            max_digits,
            reference_time,
            window
        }
    }

    pub fn simple(secret: &'a str) -> Totp {
        Totp {
            secret: secret.as_bytes(),
            max_digits: 6,
            reference_time: 0,
            window: 30,
        }
    }

    pub fn generate(&self) -> String {
        // Get HOTP counter from current timestamp
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let counter: u64 = (now - self.reference_time) / self.window;

        self.hotp(&counter.to_be_bytes())
    }

    fn hotp(&self, counter: &[u8]) -> String {
        // Compute hash
        let mut mac = Hmac::<Sha1>::new_from_slice(self.secret).unwrap();
        mac.update(counter);
        let mac_bytes: [u8;20] = mac.finalize().into_bytes().try_into().unwrap();

        // Calculate the offset by grabbing the last 4 bits
        let offset = (mac_bytes[19] & 0xf) as usize;
            
        // Grab the 4 subsequent bytes starting at the offset
        let bytes = mac_bytes.into_iter().skip(offset).take(4);
        let bytes = bytes.collect::<Vec<_>>().as_slice().try_into().unwrap();
        let truncated = u32::from_be_bytes(bytes) & 0x7fffffff;

        // Reduce to required number of digits and return as a padded string
        let otp = truncated % 10u32.pow(self.max_digits);

        format!("{0:1$}", otp , self.max_digits as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC example
    #[test]
    fn test_totp() {
        let test_secret = "12345678901234567890";
        let totp = Totp::new(test_secret, 8, 0, 30);
        let now = 59;
        let counter: u64 = (now - totp.reference_time) / totp.window;

        let otp = totp.hotp(&counter.to_be_bytes());

        assert_eq!(otp, "94287082");
    }
}
