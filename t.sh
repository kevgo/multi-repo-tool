#!/bin/sh

curl \
  -I \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/orgs/kevgo-ory/repos
