install:
	cargo install cargo-deny cargo-audit cargo-about --locked

audit:
	cargo audit

deny:
	cargo deny check

ackowledgments:
	cargo about generate about.hbs > ACKNOWLEDGEMENTS.html