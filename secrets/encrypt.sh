#!/usr/bin/env bash

DIR=`dirname ${BASH_SOURCE[0]}`

gpg \
    --yes \
    --symmetric \
    --cipher-algo AES256 \
    $DIR/pkmn-ff0000.gb

gpg \
    --yes \
    --symmetric \
    --cipher-algo AES256 \
    $DIR/pkmn-0000ff.gb
