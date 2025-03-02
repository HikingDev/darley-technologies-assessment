pub mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn trigger_error() -> Result<(), error::BinanceOptionsClientError> {
    Err(error::BinanceOptionsClientError::Unknown(
        "Triggered error for testing".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_trigger_error() {
        match trigger_error() {
            Err(error::BinanceOptionsClientError::Unknown(_)) => {
                // Expected error
            }
            _ => panic!("Expected Unknown error"),
        }
    }
}
