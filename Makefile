install:
	cargo build --release
	mv target/release/mindexer /usr/local/bin/mindexer
	mkdir /etc/mindexer
	touch /etc/mindexer/config.txt /etc/mindexer/log.txt
uninstall:
	rm /usr/local/bin/mindexer
	rm -rf /etc/mindexer
