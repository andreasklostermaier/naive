fn main() {

    fn parse_something_a (txt: &str) -> i32 {
        // do some parsing
    }

    fn parse_something_b (txt: &str) -> Option<i32> {
        // do some parsing
    }

    fn parse_something_c (txt: &str) -> Result<i32, _> {
        // do some parsing
    }

    fn parse_something_d (txt: &str) -> Option<Result<i32>> {
        // do some parsing
    }

    fn parse_something_e (txt: &str) -> Result<Option<i32>> {
        // do some parsing
    }

    enum MaybeResult<T, E> {
        None,
        Ok(T),
        Err(E),
    }

    fn parse_something_f (txt: &str) -> MaybeResult<i32> {
        // do some parsing
    }

    ////////////////////////////////////////////////////////

    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ParseError {
        #[error("input was empty")]
        EmptyInput,

        #[error("invalid format: {0}")]
        InvalidFormat(String),

    }

    fn parse_something_f(txt: &str) -> Result<i32, ParseError> {
        if txt.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        txt.trim().parse::<i32>()
            .map_err(|_| ParseError::InvalidFormat(txt.to_string()))
    }

}
