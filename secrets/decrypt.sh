#!/usr/bin/env bash

DIR=`dirname ${BASH_SOURCE[0]}`

gpg \
    --quiet \
    --batch \
    --yes \
    --decrypt \
    --passphrase="$SECRET_PASSPHRASE" \
    --output $DIR/pkmn-ff0000.gb \
    $DIR/pkmn-ff0000.gb.gpg

gpg \
    --quiet \
    --batch \
    --yes \
    --decrypt \
    --passphrase="$SECRET_PASSPHRASE" \
    --output $DIR/pkmn-0000ff.gb \
    $DIR/pkmn-0000ff.gb.gpg
