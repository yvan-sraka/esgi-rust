#! /usr/bin/env sh
rm -rf rust.systems
mdbook build
mv book rust.systems
rsync -azP rust.systems root@rust.systems:/var/www/ --delete