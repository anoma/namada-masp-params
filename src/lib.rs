use hex_literal::hex;

pub const NAMADA_MASP_SPEND_PARAMS: &[u8] = include_bytes!("masp-spend.params");
pub const NAMADA_MASP_CONVERT_PARAMS: &[u8] = include_bytes!("masp-convert.params");
pub const NAMADA_MASP_OUTPUT_PARAMS: &[u8] = include_bytes!("masp-output.params");

pub const NAMADA_MASP_SPEND_PARAMS_SIZE: usize = 49848572;
pub const NAMADA_MASP_CONVERT_PARAMS_SIZE: usize = 22570940;
pub const NAMADA_MASP_OUTPUT_PARAMS_SIZE: usize = 16398620;

pub const NAMADA_MASP_SPEND_PARAMS_BLAKE2B_HASH: [u8; 64] = hex!(
    "
    196e7c717f25e16653431559ce2c8816e750a4490f98696e3c031efca37e25e0
    647182b7b013660806db11eb2b1e365fb2d6a0f24dbbd9a4a8314fef10a7cba2
    "
);
pub const NAMADA_MASP_CONVERT_PARAMS_BLAKE2B_HASH: [u8; 64] = hex!(
    "
    dc4aaf3c3ce056ab448b6c4a7f43c1d68502c2902ea89ab8769b1524a2e8ace9
    a5369621a73ee1daa52aec826907a19974a37874391cf8f11bbe0b0420de1ab7
    "
);
pub const NAMADA_MASP_OUTPUT_PARAMS_BLAKE2B_HASH: [u8; 64] = hex!(
    "
    eafc3b1746cccc8b9eed2b69395692c5892f6aca83552a07dceb2dcbaa64dcd0
    e22434260b3aa3b049b633a08b008988cbe0d31effc77e2bc09bfab690a23724
    "
);

#[test]
fn test_sizes() {
    let sizes_to_verify = [
        (
            "spend",
            NAMADA_MASP_SPEND_PARAMS,
            NAMADA_MASP_SPEND_PARAMS_SIZE,
        ),
        (
            "convert",
            NAMADA_MASP_CONVERT_PARAMS,
            NAMADA_MASP_CONVERT_PARAMS_SIZE,
        ),
        (
            "output",
            NAMADA_MASP_OUTPUT_PARAMS,
            NAMADA_MASP_OUTPUT_PARAMS_SIZE,
        ),
    ];

    for (name, params, size) in sizes_to_verify {
        let actual_size = params.len();
        println!(
            "actual size of {} params: {} (expected {})",
            name, actual_size, size
        );
        assert_eq!(actual_size, size);
    }
}

#[test]
fn test_blake2b_hashes() {
    use blake2::{Blake2b512, Digest};

    let hashes_to_verify = [
        (
            "spend",
            NAMADA_MASP_SPEND_PARAMS,
            NAMADA_MASP_SPEND_PARAMS_BLAKE2B_HASH,
        ),
        (
            "convert",
            NAMADA_MASP_CONVERT_PARAMS,
            NAMADA_MASP_CONVERT_PARAMS_BLAKE2B_HASH,
        ),
        (
            "output",
            NAMADA_MASP_OUTPUT_PARAMS,
            NAMADA_MASP_OUTPUT_PARAMS_BLAKE2B_HASH,
        ),
    ];

    for (name, params, hash) in hashes_to_verify {
        let mut hasher = Blake2b512::new();
        hasher.update(params);
        let result = hasher.finalize();
        let result_bytes: [u8; 64] = result.into();
        println!("hashed {} to {:x?}", name, result_bytes);
        assert_eq!(result_bytes, hash);
    }
}
