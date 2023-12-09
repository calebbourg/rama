fmt:
	cargo fmt --all

sort:
	cargo sort --workspace --grouped

lint: fmt sort

check:
	cargo check --all --all-targets --all-features

clippy:
	cargo clippy --all --all-targets --all-features

clippy-fix:
	cargo clippy --fix

doc:
	RUSTDOCFLAGS="-D rustdoc::broken-intra-doc-links" cargo doc --all-features --no-deps

hack:
	cargo hack check --each-feature --no-dev-deps --workspace

test:
	cargo test --all-features

qa: lint check clippy doc hack test

rama +ARGS:
    cargo run -p rama-cli -- {{ARGS}}

docker-build:
    docker build -t rama:latest -f Dockerfile .

example-tcp-hello:
		cargo run -p rama --example tokio_tcp_hello

example-tcp-echo:
		cargo run -p rama --example tokio_tcp_echo_server

example-tls-proxy:
		cargo run -p rama --example tokio_tls_proxy

example-tcp-http-hello:
		cargo run -p rama --example tokio_tcp_http_hello

example-tls-https-hello:
		cargo run -p rama --example tokio_tls_https_hello
