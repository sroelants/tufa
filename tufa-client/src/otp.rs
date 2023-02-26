use sha1::Sha1;
use hmac::{Hmac, Mac};
use anyhow::Error;

pub struct Totp {
    secret: Vec<u8>,
    pub max_digits: u32,
    pub reference_time: u64,
    pub window: u64,
}

impl Totp {
    #[allow(dead_code)]
    pub fn new(secret: &str, max_digits: u32, reference_time: u64, window: u64) -> Totp {
        // Remove any whitespace
        let secret: String = secret.split_whitespace().collect();
        let alphabet = base32::Alphabet::RFC4648 { padding: false };
        
        Totp {
            // Decode base32 to bytes
            secret: base32::decode(alphabet, &secret).unwrap(),
            max_digits,
            reference_time,
            window
        }
    }

    pub fn simple(secret: &str) -> Totp {
        // Remove any whitespace
        let secret: String = secret.split_whitespace().collect();
        let alphabet = base32::Alphabet::RFC4648 { padding: false };
        
        Totp {
            // Decode base32 to bytes
            secret: base32::decode(alphabet, &secret).unwrap(),
            max_digits: 6,
            reference_time: 0,
            window: 30,
        }
    }

    pub fn generate(&self) -> Result<String, Error> {
        // Get HOTP counter from current timestamp
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let counter: u64 = (now - self.reference_time) / self.window;

        self.hotp(&counter.to_be_bytes())
    }

    fn hotp(&self, counter: &[u8]) -> Result<String, Error> {
        // Compute hash
        let mut mac = Hmac::<Sha1>::new_from_slice(&self.secret)?;
        mac.update(counter);
        let mac_bytes: [u8;20] = mac.finalize().into_bytes().try_into()?;

        // Calculate the offset by grabbing the last 4 bits
        let offset = (mac_bytes[19] & 0xf) as usize;
            
        // Grab the 4 subsequent bytes starting at the offset
        let bytes = mac_bytes.into_iter().skip(offset).take(4);
        let bytes = bytes.collect::<Vec<_>>().as_slice().try_into()?;
        let truncated = u32::from_be_bytes(bytes) & 0x7fffffff;

        // Reduce to required number of digits and return as a padded string
        let otp = truncated % 10u32.pow(self.max_digits);

        Ok(format!("{:01$}", otp , self.max_digits as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC example
    #[test]
    fn test_totp() -> Result<(), Error>{
        let test_secret = "12345678901234567890";
        let totp = Totp::new(test_secret, 8, 0, 30);
        let now = 59;
        let counter: u64 = (now - totp.reference_time) / totp.window;

        let otp = totp.hotp(&counter.to_be_bytes())?;

        assert_eq!(otp, "94287082");
        Ok(())
    }
}
