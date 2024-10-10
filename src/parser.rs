use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, Url, Version,
};

#[derive(PartialEq, Eq)]
enum ParserState {
    Base,
    Header,
    Body,
}

pub fn parse_http_file(client: &Client, path: &String) -> RequestBuilder {
    let mut method: Option<Method> = None;
    let mut url: Option<Url> = None;
    let mut version: Option<Version> = None;
    let mut headers = HeaderMap::new();
    let mut body: Vec<String> = Vec::new();

    let mut state = ParserState::Base;

    if let Ok(lines_it) = read_lines(path) {
        for line in lines_it.flatten() {
            let trimmed = line.trim().to_string();
            let mut chunks = trimmed.split_ascii_whitespace();

            if state == ParserState::Base {
                if trimmed.is_empty() {
                    continue;
                }
                method = Some(parse_method(chunks.next()));
                url = Some(parse_url(chunks.next()));
                version = Some(parse_version(chunks.next()));
                state = ParserState::Header;
            } else if state == ParserState::Header {
                if trimmed.is_empty() {
                    state = ParserState::Body;
                    continue;
                }
                let (key, val) = parse_header(chunks.next(), chunks.next());
                headers.insert(key, val);
            } else if state == ParserState::Body {
                if trimmed.is_empty() {
                    break;
                }
                body.push(line.clone());
            }
        }
    }

    let mut builder = client
        .request(method.unwrap(), url.unwrap())
        .version(version.unwrap())
        .headers(headers);

    if !body.is_empty() {
        builder = builder.body(body.join("\n"));
    }

    builder
}

fn read_lines(path: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_method(value: Option<&str>) -> Method {
    let str_value = value.expect("Method should have a value");
    return Method::from_str(str_value).expect("Method should be a valid HTTP verb");
}

fn parse_url(value: Option<&str>) -> Url {
    let str_value = value.expect("Url should have a value");
    return Url::parse(str_value).expect("Url should be valid");
}

fn parse_version(value: Option<&str>) -> Version {
    value.map_or(Version::default(), |s| match s {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2.0" => Version::HTTP_2,
        "HTTP/3.0" => Version::HTTP_3,
        _ => panic!("Invalid HTTP version {}", s),
    })
}

fn parse_header(key: Option<&str>, value: Option<&str>) -> (HeaderName, HeaderValue) {
    let str_key = key.expect("Header should have a key");
    let str_val = value.expect("Header should have a value");

    let key = str_key.strip_suffix(":").expect("Invalid HTTP header key");
    (
        HeaderName::from_str(key).expect("Invalid HTTP header key"),
        HeaderValue::from_str(str_val).expect("Invalid HTTP header value"),
    )
}
