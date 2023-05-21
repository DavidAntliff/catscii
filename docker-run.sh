#!/bin/bash -x
docker run \
    --env SENTRY_DSN \
    --env HONEYCOMB_API_KEY \
    --publish 8080:8080/tcp \
    --rm \
    catscii
