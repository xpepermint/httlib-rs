use glob::glob;
use std::fs::File;
use std::io::BufReader;
use serde_json::{Value};
use httlib_hpack::{Decoder};

/// Should decodes stories provided by the [HTTP/2 Japan Community].
/// 
/// [HTTP/2 Japan Community]: https://github.com/http2jp/hpack-test-case
#[test]
fn decodes_stories() {
    for entry in glob("./fixtures/**/*.json").unwrap() {
        let path = entry.unwrap();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let data: Value = serde_json::from_reader(reader).unwrap();

        let mut decoder = Decoder::default();

        for case in data.get("cases").unwrap().as_array().unwrap() {
            let mut wire = hex::decode(case["wire"].as_str().unwrap()).unwrap();
            let headers = case["headers"].as_array().unwrap();

            for header in headers {
                for (name, value) in header.as_object().unwrap() {
                    let name = name.as_bytes().to_vec();
                    let value = value.as_str().unwrap().as_bytes().to_vec();

                    while !wire.is_empty() {
                        let mut dst = Vec::with_capacity(1);
                        decoder.decode(&mut wire, &mut dst).unwrap();
                        assert_eq!(dst[0].0, name);
                        assert_eq!(dst[0].1, value);
                    }
                }
            }
        }
    }
}
