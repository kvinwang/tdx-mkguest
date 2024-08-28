#!/bin/bash

set -e

add-apt-repository -y ppa:kobuk-team/tdx-release
add-apt-repository -y ppa:kobuk-team/tdx-attestation-release

apt install -y libtdx-attest-dev