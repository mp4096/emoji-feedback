xcompile-arm:
	cargo build --release --target=armv7-unknown-linux-gnueabihf

prepare-deployment-bundle:
	rm -rf './deployment-ef'
	mkdir './deployment-ef'
	cp './target/armv7-unknown-linux-gnueabihf/release/emoji-feedback' './deployment-ef'
	cp -r './examples/.' './deployment-ef'
	cp -r './static/' './deployment-ef'
	cp -r './templates/' './deployment-ef'

.PHONY: xcompile-arm prepare-deployment-bundle
