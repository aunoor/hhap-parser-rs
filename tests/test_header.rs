extern crate hap_http_parser;

use self::hap_http_parser::{HttpParser, HttpParserCallback, HttpParserType,
                        HttpErrno};

pub mod helper;

const HEADER_LINE : &'static str = "header-key: header-value\r\n";

#[test]
fn test_request_header() {
    test_header(HttpParserType::Request);
}

#[test]
fn test_request_header_overflow() {
    test_header_overflow(HttpParserType::Request);
}

#[test]
fn test_response_header() {
    test_header(HttpParserType::Response);
}

#[test]
fn test_response_header_overflow() {
    test_header_overflow(HttpParserType::Response);
}

fn test_header(tp : HttpParserType) {
    let mut hp : HttpParser = HttpParser::new(tp);
    let mut cb = helper::CallbackEmpty;

    before(&mut hp, &mut cb, tp);

    let parsed: usize = hp.execute(&mut cb, HEADER_LINE.as_bytes());
    assert_eq!(parsed, HEADER_LINE.len());
}

fn test_header_overflow(tp: HttpParserType) {
    let mut hp : HttpParser = HttpParser::new(tp);
    let mut cb = helper::CallbackEmpty;

    before(&mut hp, &mut cb, tp);

    let len : usize = HEADER_LINE.len();
    let mut done = false;

    while !done {
        let parsed = hp.execute(&mut cb, HEADER_LINE.as_bytes());
        if parsed != len {
            assert!(hp.errno == Option::Some(HttpErrno::HeaderOverflow));
            done = true;
        }
    }
    assert!(done);
}

fn before<CB: HttpParserCallback>(hp : &mut HttpParser, cb : &mut CB, tp : HttpParserType) {
    let line = if tp == HttpParserType::Request {
        "GET / HTTP/1.1\r\n"
    } else {
        "HTTP/1.0 200 OK\r\n"
    };
    let parsed : usize = hp.execute(cb, line.as_bytes());
    assert_eq!(parsed, line.len());
}
