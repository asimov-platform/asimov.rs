// This is free and unencumbered software released into the public domain.

use core::fmt::Write;
use iri_string::types::IriReferenceString;
use std::string::String;

pub fn normalize_url(url: &str) -> Result<String, iri_string::types::CreationError<String>> {
    let iri = IriReferenceString::try_from(url)
        .or_else(|_| IriReferenceString::try_from(url.replace(" ", "%20")))?;

    let mut out: String = String::new();

    // default `file:` scheme
    let scheme = iri.scheme_str().unwrap_or("file");
    write!(&mut out, "{scheme}:").unwrap();

    if let Some(auth) = iri.authority_str() {
        write!(&mut out, "//{auth}").unwrap();
    }

    let path = iri.path_str();

    if scheme == "file"
        && let Some(rest) = path.strip_prefix("~/")
    {
        let home_dir = std::env::home_dir().expect("unable to determine home directory");
        let path2 = home_dir.join(rest);
        let path3 = std::path::absolute(&path2).unwrap_or(path2);
        let path4 = path3.to_str().unwrap_or(path);

        write!(&mut out, "{}", path4).unwrap();
    } else if scheme == "file" && !path.starts_with('/') {
        let cur_dir = std::env::current_dir().expect("unable to determine current directory");
        let path2 = cur_dir.join(path);
        let path3 = std::path::absolute(&path2).unwrap_or(path2);
        let path4 = path3.to_str().unwrap_or(path);

        write!(&mut out, "{}", path4).unwrap();
    } else if iri.authority_str().is_some() && path.is_empty() {
        write!(&mut out, "/").unwrap();
    } else {
        write!(&mut out, "{path}").unwrap();
    }

    if let Some(query) = iri.query() {
        write!(&mut out, "?{query}").unwrap()
    }

    if let Some(fraq) = iri.fragment() {
        write!(&mut out, "#{fraq}").unwrap()
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn url_normalization() {
        let cases = [
            ("https://example.org", "https://example.org/"),
            ("https://example.org/", "https://example.org/"),
            ("http://example.com/path", "http://example.com/path"),
            (
                "https://user:pass@example.org:8080/path?query=value#fragment",
                "https://user:pass@example.org:8080/path?query=value#fragment",
            ),
            ("near://testnet/123456789", "near://testnet/123456789"),
            (
                "ftp://files.example.com/file.txt",
                "ftp://files.example.com/file.txt",
            ),
            ("ws://localhost:3000/socket", "ws://localhost:3000/socket"),
            ("mailto:user@example.com", "mailto:user@example.com"),
            (
                "https://example.org/path with spaces",
                "https://example.org/path%20with%20spaces",
            ),
            (
                "https://example.org/path+with+plus",
                "https://example.org/path+with+plus",
            ),
            (
                "https://example.org/path%20already%20encoded",
                "https://example.org/path%20already%20encoded",
            ),
            (
                "https://example.org/?q=test&foo=bar",
                "https://example.org/?q=test&foo=bar",
            ),
            (
                "https://example.org/page#section1",
                "https://example.org/page#section1",
            ),
            (
                "https://example.org/search?q=hello world",
                "https://example.org/search?q=hello%20world",
            ),
            (
                "data:text/plain;base64,SGVsbG8=",
                "data:text/plain;base64,SGVsbG8=",
            ),
            ("tel:+1-555-123-4567", "tel:+1-555-123-4567"),
            ("urn:isbn:1234567890", "urn:isbn:1234567890"),
        ];

        for case in cases {
            assert_eq!(
                normalize_url(case.0).unwrap(),
                case.1,
                "input: {:?}",
                case.0
            );
        }

        #[cfg(unix)]
        {
            let cases = [
                ("/file with spaces.txt", "file:/file%20with%20spaces.txt"),
                ("/file+with+pluses.txt", "file:/file+with+pluses.txt"),
            ];

            for case in cases {
                assert_eq!(
                    normalize_url(case.0).unwrap(),
                    case.1,
                    "input: {:?}",
                    case.0
                );
            }

            if let Some(home_dir) = std::env::home_dir() {
                let home_dir = home_dir.display().to_string();

                let input = "~/path/to/file.txt";
                let want = "file:".to_string() + &home_dir + "/path/to/file.txt";
                assert_eq!(
                    normalize_url(input).unwrap(),
                    want,
                    "relative path should be get added after current directory, input: {:?}",
                    input
                );
            }

            let cur_dir = std::env::current_dir().unwrap().display().to_string();

            let input = "path/to/file.txt";
            let want = "file:".to_string() + &cur_dir + "/path/to/file.txt";
            assert_eq!(
                normalize_url(input).unwrap(),
                want,
                "relative path should be get added after current directory, input: {:?}",
                input
            );

            let input = "../path/./file.txt";
            let want = "file:".to_string() + &cur_dir + "/../path/file.txt";
            assert_eq!(
                normalize_url(input).unwrap(),
                want,
                "relative path should be get added after current directory, input: {:?}",
                input
            );

            let input = "another-type-of-a-string";
            let want = "file:".to_string() + &cur_dir + "/another-type-of-a-string";
            assert_eq!(
                normalize_url(input).unwrap(),
                want,
                "non-path-looking input should be treated as a file in current directory, input: {:?}",
                input
            );

            // let input = "hello\\ world!";
            // let want = "file:".to_string() + &cur_dir + "/hello%5C%20world!";
            // assert_eq!(
            //     normalize_url(input).unwrap(),
            //     want,
            //     "output should be url encoded, input: {:?}",
            //     input
            // );
        }

        #[cfg(windows)]
        {
            let cwd = std::env::current_dir().unwrap();
            let drive = cwd.to_str().unwrap().chars().next().unwrap();
            let cases = [
                (
                    "/file with spaces.txt",
                    format!("file:///{drive}:/file%20with%20spaces.txt"),
                ),
                (
                    "/file+with+pluses.txt",
                    format!("file:///{drive}:/file+with+pluses.txt"),
                ),
            ];

            for case in cases {
                assert_eq!(
                    normalize_url(case.0).unwrap(),
                    case.1,
                    "input: {:?}",
                    case.0
                );
            }
        }
    }
}
