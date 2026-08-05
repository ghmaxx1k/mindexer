# mindexer
a file indexer written in Rust

ghmaxx1k (c) 2026, licensed under the MIT license <https://mit-license.org>

# how to use:
    -install: run "sudo make install"
    -uninstall: "sudo make uninstall"
    -config syntax: VARIABLE=BOOLEAN/STRING
        example:    mindexerbasename=mindexerbase
                    logging=true

config and logs can be found at /etc/mindexer

to modify config just type the desired flags inside

current version: 0.2.1 BETA

current stable version: 0.1.1 BETA

for future releases:
    -add HTTP/HTTPS request support

# changelog:

0.0.1 ALPHA (pre-release):
    -created repo and code base
    -coded main code, not working, needs fix
    -implemented mp3 and mp4 support for now

0.0.2 ALPHA (pre-release):
    -ditched commands and used rust fs/env libraries instead
    -commented code more
    -made code lighter

0.1.0 BETA:
    -fixed wildcard issue
    -deleted useless code parts
    -made code stable
    -fixed renaming issues
    -added sortfolder cleaning feature
    -configured makefile

0.1.1 BETA:
    -added support for more wildcards
    -deleted commets, boring, just read println messages
    -switched to MIT license

0.2.0 BETA - PRE-RELEASE:
    -significantly trimmed down basechecker algorithm and made it autonomous
    -now supports direct variable modification on top of the "main" thread (unstable)
    -started developing indexer algorithm
    -mindexer is now completely independent

0.2.1 BETA:
    -added config support
    -added logging
    -renewed wildcard detection algorithm