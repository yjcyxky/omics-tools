build:
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -v ~/.cargo/config:/root/.cargo/config -w /usr/src/myapp rust:latest cargo build --release

build-jar:
	build
	cp target/release/vcf-util omics-tools-clj/resources/vcf-util
	lein deploy clojars