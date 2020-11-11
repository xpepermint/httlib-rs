use hex;
use httlib_hpack::{Encoder, Decoder};

/// Should encode and decode example requests provided by the HPACK
/// specification ([C.3.], [C.4.], [C.5.], [C.6.]).
/// 
/// [C.3.]: https://tools.ietf.org/html/rfc7541#appendix-C.3
/// [C.4.]: https://tools.ietf.org/html/rfc7541#appendix-C.4
/// [C.5.]: https://tools.ietf.org/html/rfc7541#appendix-C.5
/// [C.6.]: https://tools.ietf.org/html/rfc7541#appendix-C.6
#[test]
fn encodes_and_decodes_requests() {
    let examples = vec![
        vec![ // request examples without Huffman coding (C.3.)
            ( // request 1
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 20),
                    (b":scheme".to_vec(), b"http".to_vec(), 20),
                    (b":path".to_vec(), b"/".to_vec(), 20),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 20),
                ],
                vec![
                    "8286", "8441", "0f77", "7777", "2e65", "7861", "6d70", "6c65",  // ...A.www.example
                    "2e63", "6f6d",                                                  // .com
                ],
            ),
            ( // request 2
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 20),
                    (b":scheme".to_vec(), b"http".to_vec(), 20),
                    (b":path".to_vec(), b"/".to_vec(), 20),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 20),
                    (b"cache-control".to_vec(), b"no-cache".to_vec(), 20),
                ],
                vec![
                    "8286", "84be", "5808", "6e6f", "2d63", "6163", "6865"           // ....X.no-cache
                ],
            ),
            ( // request 3
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 20),
                    (b":scheme".to_vec(), b"https".to_vec(), 20),
                    (b":path".to_vec(), b"/index.html".to_vec(), 20),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 20),
                    (b"custom-key".to_vec(), b"custom-value".to_vec(), 20),
                ],
                vec![
                    "8287", "85bf", "400a", "6375", "7374", "6f6d", "2d6b", "6579",  // ....@.custom-key
                    "0c63", "7573", "746f", "6d2d", "7661", "6c75", "65",            // .custom-value
                ],
            ),
        ],
        vec![ // request examples with Huffman coding (C.4.)
            ( // request 1
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 23),
                    (b":scheme".to_vec(), b"http".to_vec(), 23),
                    (b":path".to_vec(), b"/".to_vec(), 23),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 23),
                ],
                vec![
                    "8286", "8441", "8cf1", "e3c2", "e5f2", "3a6b", "a0ab", "90f4",  // ...A......:k....
                    "ff",                                                            // .
                ],
            ),
            ( // request 2
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 23),
                    (b":scheme".to_vec(), b"http".to_vec(), 23),
                    (b":path".to_vec(), b"/".to_vec(), 23),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 23),
                    (b"cache-control".to_vec(), b"no-cache".to_vec(), 23),
                ],
                vec![
                    "8286", "84be", "5886", "a8eb", "1064", "9cbf"                   // ....X....d..
                ],
            ),
            ( // request 3
                vec![
                    (b":method".to_vec(), b"GET".to_vec(), 23),
                    (b":scheme".to_vec(), b"https".to_vec(), 23),
                    (b":path".to_vec(), b"/index.html".to_vec(), 23),
                    (b":authority".to_vec(), b"www.example.com".to_vec(), 23),
                    (b"custom-key".to_vec(), b"custom-value".to_vec(), 23),
                ],
                vec![
                    "8287", "85bf", "4088", "25a8", "49e9", "5ba9", "7d7f", "8925",  // ....@.%.I.[.}..%
                    "a849", "e95b", "b8e8", "b4bf",                                  // .I.[....
                ],
            ),
        ],
        vec![ // response examples without Huffman coding (C.5.)
            ( // response 1
                vec![
                    (b":status".to_vec(), b"302".to_vec(), 20),
                    (b"cache-control".to_vec(), b"private".to_vec(), 20),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:21 GMT".to_vec(), 20),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 20),
                ],
                vec![
                    "4803", "3330", "3258", "0770", "7269", "7661", "7465", "611d",  // H.302X.privatea.
                    "4d6f", "6e2c", "2032", "3120", "4f63", "7420", "3230", "3133",  // Mon, 21 Oct 2013
                    "2032", "303a", "3133", "3a32", "3120", "474d", "546e", "1768",  // 20:13:21 GMTn.h
                    "7474", "7073", "3a2f", "2f77", "7777", "2e65", "7861", "6d70",  // ttps://www.examp
                    "6c65", "2e63", "6f6d",                                          // le.com
                ],
            ),
            ( // response 2
                vec![
                    (b":status".to_vec(), b"307".to_vec(), 20),
                    (b"cache-control".to_vec(), b"private".to_vec(), 20),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:21 GMT".to_vec(), 20),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 20),
                ],
                vec![
                    "4803", "3330", "37c1", "c0bf",                                  // H.307...
                ],
            ),
            ( // response 3
                vec![
                    (b":status".to_vec(), b"200".to_vec(), 20),
                    (b"cache-control".to_vec(), b"private".to_vec(), 20),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:22 GMT".to_vec(), 20),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 20),
                    (b"content-encoding".to_vec(), b"gzip".to_vec(), 20),
                    (b"set-cookie".to_vec(), b"foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1".to_vec(), 20),
                ],
                vec![
                    "88c1", "611d", "4d6f", "6e2c", "2032", "3120", "4f63", "7420",  // ..a.Mon, 21 Oct
                    "3230", "3133", "2032", "303a", "3133", "3a32", "3220", "474d",  // 2013 20:13:22 GM
                    "54c0", "5a04", "677a", "6970", "7738", "666f", "6f3d", "4153",  // T.Z.gzipw8foo=AS
                    "444a", "4b48", "514b", "425a", "584f", "5157", "454f", "5049",  // DJKHQKBZXOQWEOPI
                    "5541", "5851", "5745", "4f49", "553b", "206d", "6178", "2d61",  // UAXQWEOIU; max-a
                    "6765", "3d33", "3630", "303b", "2076", "6572", "7369", "6f6e",  // ge=3600; version
                    "3d31",                                                          // =1
                ],
            ),
        ],
        vec![ // response examples with Huffman coding (C.6.)
            ( // response 1
                vec![
                    (b":status".to_vec(), b"302".to_vec(), 23),
                    (b"cache-control".to_vec(), b"private".to_vec(), 23),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:21 GMT".to_vec(), 23),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 23),
                ],
                vec![
                    "4882", "6402", "5885", "aec3", "771a", "4b61", "96d0", "7abe",  // H.d.X...w.Ka..z.
                    "9410", "54d4", "44a8", "2005", "9504", "0b81", "66e0", "82a6",  // ..T.D. .....f...
                    "2d1b", "ff6e", "919d", "29ad", "1718", "63c7", "8f0b", "97c8",  // -..n..)...c.....
                    "e9ae", "82ae", "43d3",                                          // ....C.
                ],
            ),
            ( // response 2
                vec![
                    (b":status".to_vec(), b"307".to_vec(), 23),
                    (b"cache-control".to_vec(), b"private".to_vec(), 23),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:21 GMT".to_vec(), 23),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 23),
                ],
                vec![
                    "4883", "640e", "ffc1", "c0bf",                                  // H.d.....
                ],
            ),
            ( // response 3
                vec![
                    (b":status".to_vec(), b"200".to_vec(), 23),
                    (b"cache-control".to_vec(), b"private".to_vec(), 23),
                    (b"date".to_vec(), b"Mon, 21 Oct 2013 20:13:22 GMT".to_vec(), 23),
                    (b"location".to_vec(), b"https://www.example.com".to_vec(), 23),
                    (b"content-encoding".to_vec(), b"gzip".to_vec(), 23),
                    (b"set-cookie".to_vec(), b"foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1".to_vec(), 23),
                ],
                vec![
                    "88c1", "6196", "d07a", "be94", "1054", "d444", "a820", "0595",  // ..a..z...T.D. ..
                    "040b", "8166", "e084", "a62d", "1bff", "c05a", "839b", "d9ab",  // ...f...-...Z....
                    "77ad", "94e7", "821d", "d7f2", "e6c7", "b335", "dfdf", "cd5b",  // w..........5...[
                    "3960", "d5af", "2708", "7f36", "72c1", "ab27", "0fb5", "291f",  // 9`..'..6r..'..).
                    "9587", "3160", "65c0", "03ed", "4ee5", "b106", "3d50", "07",    // ..1`e...N...=P.
                ],
            ),
        ],
    ];

    for requests in examples {
        let mut encoder = Encoder::default();
        let mut decoder = Decoder::default();

        for (fields, wire) in requests {

            // encoding
            let mut dst0 = Vec::new();
            for field in fields.clone() {
                encoder.encode(field, &mut dst0).unwrap();
            }
            assert_eq!(hex::encode(&dst0), wire.join(""));

            // decoding
            let mut dst1 = Vec::new();
            decoder.decode(&mut dst0, &mut dst1).unwrap();
            for (i, h) in dst1.iter().enumerate() {
                assert_eq!(fields[i].0, dst1[i].0);
                assert_eq!(fields[i].1, dst1[i].1);
            }
        }
    }
}
