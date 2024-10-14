use reqwest::blocking::Response;

pub fn output_http(response: Response) -> anyhow::Result<()> {
    println!("{:?} {}", response.version(), response.status());
    for (key, value) in response.headers().iter() {
        println!("{}: {}", key, value.to_str().unwrap());
    }
    println!("\n{}", response.text()?);
    Ok(())
}
