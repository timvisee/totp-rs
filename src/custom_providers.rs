#[cfg(feature = "steam")]
use crate::{Algorithm, TOTP};

#[cfg(feature = "steam")]
impl TOTP {
    #[cfg(feature = "otpauth")]
    /// Will create a new instance of TOTP using the Steam algorithm with given parameters. See [the doc](struct.TOTP.html#fields) for reference as to how to choose those values
    ///
    /// # Description
    /// * `secret`: expect a non-encoded value, to pass in base32 string use `Secret::Encoded(String)`
    ///
    /// ```rust
    /// use totp_rs::{Secret, TOTP};
    /// let secret = Secret::Encoded("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".to_string());
    /// let totp = TOTP::new_steam(secret.to_bytes().unwrap(), Some("username".to_string()));
    /// ```
    pub fn new_steam(secret: Vec<u8>, account_name: Option<String>) -> TOTP {
        Self::new_unchecked(
            Algorithm::Steam,
            5,
            1,
            30,
            secret,
            Some("Steam".into()),
            account_name
                .map(|n| format!("Steam:{}", n))
                .unwrap_or_else(|| "".into()),
        )
    }

    #[cfg(not(feature = "otpauth"))]
    /// Will create a new instance of TOTP using the Steam algorithm with given parameters. See [the doc](struct.TOTP.html#fields) for reference as to how to choose those values
    ///
    /// # Description
    /// * `secret`: expect a non-encoded value, to pass in base32 string use `Secret::Encoded(String)`
    ///
    /// ```rust
    /// use totp_rs::{Secret, TOTP};
    /// let secret = Secret::Encoded("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".to_string());
    /// let totp = TOTP::new_steam(secret.to_bytes().unwrap());
    /// ```
    pub fn new_steam(secret: Vec<u8>) -> TOTP {
        Self::new_unchecked(Algorithm::Steam, 5, 1, 30, secret)
    }
}