#!/bin/sh
openssl req -nodes -new -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
          -days 365 -sha256 -subj "/C=US/ST=VA/L=GMU/O=Zeke/OU=Org/CN=zeke.us" || exit 1

