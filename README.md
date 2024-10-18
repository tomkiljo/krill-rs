![krill](docs/krill.png)

A small and fast🤞CLI for testing REST APIs.

Krill is a personal learning project to get to know the Rust programming language Inspired by the [REST Client](https://github.com/Huachao/vscode-restclient) extension for Visual Studio Code, I wanted to create similar functionality in the command line.

# Examples

Example requests use Postman Echo service.

Simple requests

```shell
cargo run -- -f examples/delete.http
cargo run -- -f examples/get_notfound.http
cargo run -- -f examples/get.http
cargo run -- -f examples/post.http
```

With variables from command line, environment, and request

```shell
KRILL_ENV=var_from_env cargo run -- \
  -f examples/variables.http \
  -p query=foo \
  -p header=bar \
  -p value=baz
```

With dynamic variables

```shell
cargo run -- -f examples/dynamic.http
```

With includes

```shell
cargo run -- -f examples/include/requst.http
```
