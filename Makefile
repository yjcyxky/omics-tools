build-linux:
	@echo "Build omics-tools for linux..."
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/vcf-util omics-tools-clj/resources/vcf-util-x86_64-linux

build-mac:
	@echo "Build omics-tools for mac..."
	cargo build --release
	cp target/release/vcf-util omics-tools-clj/resources/vcf-util-x86_64-macosx

build-jar: build-linux build-mac
	cd omics-tools-clj && lein jar

publish-jar: build-linux build-mac
	cd omics-tools-clj && lein deploy clojars
