install:
	cargo install cargo-deny cargo-audit cargo-about --locked

audit:
	cargo audit

deny:
	cargo deny check

ackowledgments:
	cargo about generate about.hbs > ACKNOWLEDGEMENTS.html

test:
	cargo test --features=default
	cargo test --features=skip-inherent,impl-enuminfo
	cargo test --features=skip-inherent
	cargo test --features=impl-enuminfo