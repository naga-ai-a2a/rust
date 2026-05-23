#!/usr/bin/env bash

# compile-run.sh

if [ $# -eq 0 ]; then
    printf "\n\tNeed Arg: <package> [prompt]\n\n"; exit 1
fi

bn=`basename $1 .rs`; fn=src/$bn.rs; tn=toml/$bn.toml

ls $fn >/dev/null 2>&1

if [ $? -eq 0 ]; then
    echo; cat $fn; echo
else
    printf "\n\tNo such file: $fn\n\n"; exit 1
fi

trap "rm -fr $bn" EXIT SIGINT SIGTERM

rm -fr $bn; cargo new $bn

cp $fn $bn/src/main.rs

[ -f $tn ] && cp $tn $bn/Cargo.toml

shift; (cd $bn; echo; cargo run "$@"; echo)
