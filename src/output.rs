use reqwest::{blocking::Response, Error};

pub fn output_http(result: Result<Response, Error>) {
    match result {
        Ok(res) => {
            println!("{:?} {}", res.version(), res.status());
            for (key, value) in res.headers().iter() {
                println!("{}: {}", key, value.to_str().unwrap());
            }
            println!("\n{}", res.text().unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
