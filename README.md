![krill](docs/krill.png)

A small and fast🤞CLI for testing REST APIs.

Krill is a personal learning project to get to know the Rust programming language Inspired by the [REST Client](https://github.com/Huachao/vscode-restclient) extension for Visual Studio Code, I wanted to create similar functionality in the command line.

# Examples

Example requests use Postman Echo service.

```shell
cargo run -- -p examples/delete.http
cargo run -- -p examples/get_notfound.http
cargo run -- -p examples/get.http
cargo run -- -p examples/post.http
```
