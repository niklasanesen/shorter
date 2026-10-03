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

to view the website open the `web/index.html` file in a browser. if you want to make any changes make sure you've got the [tailwind cli](https://tailwindcss.com/docs/installation/tailwind-cli) installed and running in the background.

## architecture

- **server**: an http api built with [axum](https://github.com/tokio-rs/axum) and deployed to [aws lambda](https://aws.amazon.com/lambda)
- **web**: a static site built with [htmx](https://htmx.org)/[tailwindcss](https://tailwindcss.com) and deployed to [cloudflare pages](https://www.cloudflare.com/products/pages)

## credits

inspired by [panelsdesu](https://panelsdesu.com) and [instant domain search](https://instantdomainsearch.com)
