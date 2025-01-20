// src/utils/validators.rs
use url::Url;

/// Validates a URL
#[allow(dead_code)]
pub fn validate_url(url: &str) -> Result<String, String> {
    Url::parse(url)
        .map(|_| url.to_string()) // Return the original string if valid
        .map_err(|_| format!("Invalid URL provided: {}", url)) // Error if parsing fails
}

#[allow(dead_code)]
pub fn validate_port(port: &str) -> Result<u16, String> {
    port.parse::<u16>()
        .map_err(|_| {
            format!(
                "Port '{}' must be a valid number in the range 1–65535",
                port
            )
        })
        .and_then(|p| {
            if (1..=65535).contains(&p) {
                Ok(p)
            } else {
                Err(format!("Port '{}' must be in the range 1–65535", port))
            }
        })
}

/// Validates a bind address (valid IPv4/IPv6 address)
#[allow(dead_code)]
pub fn validate_bind_address(address: &str) -> Result<String, String> {
    address
        .parse::<std::net::IpAddr>()
        .map(|_| address.to_string())
        .map_err(|_| format!("Bind address '{}' is invalid", address))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url() {
        // Valid URLs
        assert!(validate_url("http://example.com").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://127.0.0.1").is_ok());
        assert!(validate_url("https://example.com?key=value").is_ok());
        assert!(validate_url("ftp://example.com").is_ok());

        // Invalid URLs
        assert!(validate_url("not-a-url").is_err());
        assert!(validate_url("").is_err());
    }

    #[test]
    fn test_validate_port() {
        // Valid ports
        assert!(validate_port("1").is_ok());
        assert!(validate_port("65535").is_ok());
        assert_eq!(validate_port("80").unwrap(), 80);

        // Invalid ports
        assert!(validate_port("0").is_err());
        assert!(validate_port("65536").is_err());
        assert!(validate_port("not-a-number").is_err());
        assert!(validate_port("9999999999").is_err());
    }

    #[test]
    fn test_validate_bind_address() {
        // Valid IPv4 and IPv6
        assert!(validate_bind_address("127.0.0.1").is_ok());
        assert!(validate_bind_address("192.168.1.1").is_ok());
        assert!(validate_bind_address("255.255.255.255").is_ok());
        assert!(validate_bind_address("::1").is_ok());
        assert!(validate_bind_address("2001:db8::ff00:42:8329").is_ok());

        // Invalid addresses
        assert!(validate_bind_address("invalid-address").is_err());
        assert!(validate_bind_address("").is_err());
    }
}
