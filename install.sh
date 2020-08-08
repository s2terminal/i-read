#!/bin/sh -e
set -e
cd /tmp
IREAD=./iread.tar.gz
IREAD_VERSION=`curl --silent https://api.github.com/repos/s2terminal/i-read/releases | grep browser_download_url | awk -F'/' '{ print $8 }' | head -n 1`
curl --location --remote-name https://github.com/s2terminal/i-read/releases/download/$IREAD_VERSION/iread.tar.gz
tar --gunzip --extract --file $IREAD
rm $IREAD
chmod 755 ./iread
mv ./iread /usr/local/bin/
echo "Installed $IREAD_VERSION"
unset IREAD
unset IREAD_VERSION
