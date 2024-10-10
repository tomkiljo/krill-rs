use reqwest::{blocking::Response, Error};

pub fn output_http(result: Result<Response, Error>) -> anyhow::Result<()> {
    let res = result?;
    println!("{:?} {}", res.version(), res.status());
    for (key, value) in res.headers().iter() {
        println!("{}: {}", key, value.to_str().unwrap());
    }
    println!("\n{}", res.text()?);
    Ok(())
}
