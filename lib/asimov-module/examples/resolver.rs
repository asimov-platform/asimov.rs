// This is free and unencumbered software released into the public domain.

use std::io::Write;

use asimov_module::{models::Manifest, resolve::ResolverBuilder};

const YAMLS: &str = r#"
name: near
label: NEAR Protocol
summary: Data import from the NEAR Protocol blockchain network.
links:
  - https://github.com/asimov-modules/asimov-near-module
  - https://crates.io/crates/asimov-near-module
  - https://pypi.org/project/asimov-near-module
  - https://rubygems.org/gems/asimov-near-module
  - https://npmjs.com/package/asimov-near-module

provides:
  flows:
    - asimov-near-fetcher

handles:
  url_protocols:
    - near

---
name: serpapi
label: SerpApi
summary: Data import powered by the SerpApi search data platform.
links:
  - https://github.com/asimov-modules/asimov-serpapi-module
  - https://crates.io/crates/asimov-serpapi-module
  - https://pypi.org/project/asimov-serpapi-module
  - https://rubygems.org/gems/asimov-serpapi-module
  - https://npmjs.com/package/asimov-serpapi-module

provides:
  flows:
    - asimov-serpapi-fetcher
    - asimov-serpapi-importer

handles:
  url_prefixes:
    - https://bing.com/search?q=
    - https://duckduckgo.com/?q=
    - https://google.com/search?q=

---
name: apify
label: Apify
summary: Data import powered by the Apify web automation platform.
links:
    - https://github.com/asimov-modules/asimov-apify-module
    - https://crates.io/crates/asimov-apify-module
    - https://pypi.org/project/asimov-apify-module
    - https://rubygems.org/gems/asimov-apify-module
    - https://npmjs.com/package/asimov-apify-module

provides:
    flows:
    - asimov-apify-fetcher
    - asimov-apify-importer

handles:
    url_patterns:
    - https://google.com/search?q=:query
    - https://x.com/:account/followers
    - https://x.com/:account/following

---
name: brightdata
label: Bright Data
summary: Data import powered by the Bright Data web data platform.
links:
    - https://github.com/asimov-modules/asimov-brightdata-module
    - https://crates.io/crates/asimov-brightdata-module
    - https://pypi.org/project/asimov-brightdata-module
    - https://rubygems.org/gems/asimov-brightdata-module
    - https://npmjs.com/package/asimov-brightdata-module

provides:
    flows:
    - asimov-brightdata-fetcher
    - asimov-brightdata-importer

handles:
    url_prefixes:
    - https://airbnb.com/rooms/
    - https://amazon.com/
    - https://amazon.com/sp?seller=
    - https://crunchbase.com/organization/
    - https://ebay.com/itm/
    - https://facebook.com/events/
    - https://facebook.com/groups/
    - https://facebook.com/marketplace/item/
    - https://facebook.com/share/p/
    - https://finance.yahoo.com/quote/
    - https://google.com/shopping/product/
    - https://indeed.com/cmp/
    - https://instagram.com/
    - https://instagram.com/p/
    - https://instagram.com/reel/
    - https://linkedin.com/company/
    - https://linkedin.com/in/
    - https://linkedin.com/jobs/
    - https://linkedin.com/posts/
    - https://linkedin.com/pulse/
    - https://walmart.com/global/seller/
    - https://walmart.com/ip/
    - https://x.com/
    - https://youtube.com/@
    - https://youtube.com/watch?v=
"#;

fn main() {
    let mut builder = ResolverBuilder::new();

    for module in YAMLS.split("---") {
        let module: Manifest = serde_yml::from_str(module).unwrap();
        for protocol in module.handles.url_protocols {
            builder.insert_protocol(&module.name, &protocol).unwrap();
        }

        for prefix in module.handles.url_prefixes {
            builder.insert_prefix(&module.name, &prefix).unwrap()
        }

        for pattern in module.handles.url_patterns {
            builder.insert_pattern(&module.name, &pattern).unwrap()
        }
    }

    let resolver = builder.build().unwrap();

    let mut stdout = std::io::stdout().lock();
    let mut lines = std::io::stdin().lines();
    loop {
        stdout.write_all(b"query > ").unwrap();
        stdout.flush().unwrap();

        let Some(Ok(query)) = lines.next() else {
            break;
        };
        let answers = resolver.resolve(&query).unwrap();
        println!("answers:");
        for answer in answers {
            println!(" - {}", answer.name);
        }
    }
}
