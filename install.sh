#!/bin/sh -e
set -e
cd /tmp
IREADRS=./i-read-rs.tar.gz
IREADRS_VERSION=`curl --silent https://api.github.com/repos/s2terminal/i-read-rs/releases | grep browser_download_url | awk -F'/' '{ print $8 }' | head -n 1`
wget https://github.com/s2terminal/i-read-rs/releases/download/$IREADRS_VERSION/i-read-rs.tar.gz --output-document=$IREADRS
tar --gunzip --extract --file $IREADRS
rm $IREADRS
chmod 755 ./i-read-rs
mv ./i-read-rs /usr/local/bin/
echo "Install $IREADRS_VERSION"
unset IREADRS
unset IREADRS_VERSION
