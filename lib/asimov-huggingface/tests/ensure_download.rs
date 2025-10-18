use asimov_huggingface::ensure_file;

#[test]
#[ignore] // requires network
fn download_file_creates_local_copy() {
    let path = ensure_file("julien-c/dummy-unknown", "config.json").unwrap();
    assert!(path.exists());
}
