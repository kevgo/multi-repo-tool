#!/bin/sh

curl \
  -I \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: token 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" \
  https://api.github.com/orgs/kevgo-ory/repos
