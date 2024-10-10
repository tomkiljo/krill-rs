use anyhow::{anyhow, Context, Ok};
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, HeaderName, HeaderValue},
    Method, Url, Version,
};
use std::{
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(PartialEq, Eq)]
enum ParserState {
    Base,
    Header,
    Body,
}

pub fn parse_http_file(client: &Client, path: &String) -> Result<RequestBuilder, anyhow::Error> {
    let mut method: Option<Method> = None;
    let mut url: Option<Url> = None;
    let mut version: Option<Version> = None;
    let mut headers = HeaderMap::new();
    let mut body: Vec<String> = Vec::new();

    let mut state = ParserState::Base;

    for line in read_lines(path)?.map_while(Result::ok) {
        let trimmed = line.trim().to_string();
        let mut chunks = trimmed.split_ascii_whitespace();

        match state {
            ParserState::Base => {
                if trimmed.is_empty() {
                    continue;
                }
                method = Some(parse_method(chunks.next())?);
                url = Some(parse_url(chunks.next())?);
                version = Some(parse_version(chunks.next())?);
                state = ParserState::Header;
            }
            ParserState::Header => {
                if trimmed.is_empty() {
                    state = ParserState::Body;
                    continue;
                }
                let (key, val) = parse_header(chunks.next(), chunks.next())?;
                headers.insert(key, val);
            }
            ParserState::Body => {
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

    Ok(builder)
}

fn read_lines(path: &String) -> Result<io::Lines<io::BufReader<File>>, anyhow::Error> {
    let file = File::open(path).with_context(|| format!("Failed to open file \"{}\"", path))?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_method(value: Option<&str>) -> Result<Method, anyhow::Error> {
    let str_value = value.unwrap_or_default();
    let method = Method::from_str(str_value)
        .with_context(|| format!("Method should be a valid HTTP verb, got \"{}\"", str_value))?;
    Ok(method)
}

fn parse_url(value: Option<&str>) -> anyhow::Result<Url> {
    let str_value = value.unwrap_or_default();
    let url = Url::parse(str_value)
        .with_context(|| format!("Url should be valid, got \"{}\"", str_value))?;
    Ok(url)
}

fn parse_version(value: Option<&str>) -> anyhow::Result<Version> {
    value.map_or(Ok(Version::default()), |s| match s {
        "HTTP/0.9" => Ok(Version::HTTP_09),
        "HTTP/1.0" => Ok(Version::HTTP_10),
        "HTTP/1.1" => Ok(Version::HTTP_11),
        "HTTP/2.0" => Ok(Version::HTTP_2),
        "HTTP/3.0" => Ok(Version::HTTP_3),
        _ => Err(anyhow!("Invalid HTTP version {}", s)),
    })
}

fn parse_header(
    key: Option<&str>,
    value: Option<&str>,
) -> anyhow::Result<(HeaderName, HeaderValue)> {
    let str_key = key.unwrap_or_default();
    let str_val = value.unwrap_or_default();
    let key = str_key.strip_suffix(":").unwrap_or_default();
    Ok((
        HeaderName::from_str(key)
            .with_context(|| format!("Invalid HTTP header key \"{}\"", key))?,
        HeaderValue::from_str(str_val)
            .with_context(|| format!("Invalid HTTP header value \"{}\"", str_val))?,
    ))
}
