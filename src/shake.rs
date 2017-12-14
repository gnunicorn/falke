
use tiny_keccak::Keccak;

// Falcon-Sign.info cuts the SHAKE128 and SHAKE256 XOF off at 32

fn shake128(data: &[u8]) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];
    Keccak::shake128(data, &mut res);
    res
}


fn shake256(data: &[u8]) -> [u8; 32] {
    let mut res: [u8; 32] = [0; 32];
    Keccak::shake256(data, &mut res);
    res
}


#[cfg(test)]
mod tests {
    use shake::*;
    use hex;

    // let's ensure our algorithm has the same behaviour
    // as the reference uses - using _exact same tests_

    #[test]
    fn test_shake128() {
        assert_eq!(hex::encode(shake128("".as_bytes())), "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");
        assert_eq!(hex::encode(shake128("The quick brown fox jumps over the lazy dog".as_bytes())),
            "f4202e3c5852f9182a0430fd8144f0a74b95e7417ecae17db0f8cfeed0e3e66e");
        assert_eq!(hex::encode(shake128("The quick brown fox jumps over the lazy dof".as_bytes())),
            "853f4538be0db9621a6cea659a06c1107b1f83f02b13d18297bd39d7411cf10c");
    }

    #[test]
    fn test_shake256() {
        assert_eq!(hex::encode(shake256("".as_bytes())),
            "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f");
    }
}
