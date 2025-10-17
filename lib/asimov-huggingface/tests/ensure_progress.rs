use asimov_huggingface::ensure_file;

#[test]
#[ignore]
fn download_file_creates_local_copy() {
    let path = ensure_file("julien-c/dummy-unknown", "config.json")
        .expect("download should succeed");

    assert!(path.exists(), "downloaded file should exist");
    assert!(
        path.to_string_lossy().contains("julien-c--dummy-unknown"),
        "path should contain repo name"
    );
}
