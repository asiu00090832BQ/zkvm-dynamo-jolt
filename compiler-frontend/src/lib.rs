pub fn compile(source: &str) -> Vec<u8> {
    source.as_bytes().to_vec()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compile() {
        assert_eq!(compile("test"), b"test");
    }
}
