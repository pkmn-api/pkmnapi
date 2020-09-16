#!/usr/bin/env bash

DIR=`dirname ${BASH_SOURCE[0]}`

gpg \
    --yes \
    --symmetric \
    --cipher-algo AES256 \
    $DIR/pkmn.gb
