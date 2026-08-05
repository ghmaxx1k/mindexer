install:
	cargo build --release
	mv target/release/mindexer /usr/local/bin/mindexer
	mkdir /etc/mindexer
	touch /etc/mindexer/config /etc/mindexer/logs
uninstall:
	rm /usr/local/bin/mindexer
	rm -rf /etc/mindexer