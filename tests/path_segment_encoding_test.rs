//! The generated path-segment encoder keeps `:` and `@` literal (RFC 3986 §3.3)
//! while still encoding `/`, `?`, `#` and `%`.

use openapi_to_rust::{CodeGenerator, GeneratorConfig};

#[test]
fn path_segment_encoder_keeps_colon_and_at_sign() {
    let generator = CodeGenerator::new(GeneratorConfig {
        enable_async_client: true,
        ..Default::default()
    });
    let client = generator.generate_http_client_struct().to_string();
    let start = client
        .find("fn __pct_encode_path_segment")
        .expect("encoder is emitted");
    let encoder = &client[start..];
    let encoder = &encoder[..encoder.find("out }").unwrap_or(encoder.len())];
    for kept in ["b':'", "b'@'", "b'~'"] {
        assert!(
            encoder.contains(kept),
            "{kept} must pass through: {encoder}"
        );
    }
    for encoded in ["b'/'", "b'?'", "b'#'", "b'%'"] {
        assert!(
            !encoder.contains(encoded),
            "{encoded} must be percent-encoded: {encoder}"
        );
    }
}
