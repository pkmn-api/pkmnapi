#!/usr/bin/env bash

DIR=`dirname ${BASH_SOURCE[0]}`

gpg \
    --quiet \
    --batch \
    --yes \
    --decrypt \
    --passphrase="$SECRET_PASSPHRASE" \
    --output $DIR/pkmn.gb \
    $DIR/pkmn.gb.gpg
