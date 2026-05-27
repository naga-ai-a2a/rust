#!/usr/bin/env bash

# compile-run-one.sh

cd `dirname $0`

if [ $# -eq 0 ]; then
    printf "\n\tNeed Arg(s): <package> [prompt]; genai needs prompt\n\n"
    printf "\tHere's list of valid packages ...\n\n"
    for p in `ls src`; do printf "`basename $p .rs`\n"; done | cat -n
    echo; exit 1
fi

if echo $1 | grep -q genai; then
    printf "\n\tgenai needs prompt, so pass it as 2nd arg\n\n"; exit 2
fi

bn=`basename $1 .rs`; fn=src/$bn.rs; tn=toml/$bn.toml

ls $fn >/dev/null 2>&1

if [ $? -eq 0 ]; then
    echo; cat -n $fn; echo
else
    printf "\n\tInvalid arg. No such file: $fn\n\n"; exit 1
fi

trap "rm -fr $bn" EXIT SIGINT SIGTERM

rm -fr $bn; cargo new $bn

cp $fn $bn/src/main.rs

[ -f $tn ] && cp $tn $bn/Cargo.toml

shift; (cd $bn; echo; cargo run "$@"; echo)
