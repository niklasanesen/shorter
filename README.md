## installation

make sure you've got [git](https://git-scm.com) installed, then clone this repo:

```sh
git clone https://github.com/niklasanesen/shorter
```

## server

to start the server you need [rust](https://rust-lang.org) installed, then run this command from inside the `shorter` directory:

```sh
cargo run
```

## web

to view the website simply open the `web/index.html` file in a browser.

## architecture

- **server**: an http api written in [axum](https://github.com/tokio-rs/axum) and deployed to [aws lambda](https://aws.amazon.com/lambda).
- **web**: a static site written in [htmx](https://htmx.org)/css/js and deployed to [cloudflare pages](https://www.cloudflare.com/products/pages).

## credits

inspired by [panelsdesu](https://panelsdesu.com) and [instant domain search](https://instantdomainsearch.com).
