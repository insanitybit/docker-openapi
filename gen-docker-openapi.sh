#!/bin/bash

OPENAPI_GENERATOR_CLI_VERSION="v5.3.1"
SPEC_VERSION="v1.41.yaml"

docker run \
    --user "1000:1000" \
    --rm \
    -v "${PWD}:/local/" \
    openapitools/openapi-generator-cli:${OPENAPI_GENERATOR_CLI_VERSION} generate \
    --input-spec /local/${SPEC_VERSION} \
    --output /local/output/ \
    --generator-name rust \
    --library hyper \
    --additional-properties=packageName="docker-openapi" \
    --additional-properties=packageVersion="v0.0.1" \
    --additional-properties=useSingleRequestParameter=true
