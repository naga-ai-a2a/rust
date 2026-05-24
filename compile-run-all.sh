#!/usr/bin/env bash

# compile-run-all.sh

cd `dirname $0`

if [ $# -eq 0 ]; then
  printf "\n\tNeed Args (for genai): <model> <prompt>\n\n"; exit 1
fi

for f in src/*; do

  printf "\nf: $f\n\n"

  bn=`basename $f .rs`; fn=src/$bn.rs; tn=toml/$bn.toml

  echo; cat -n $fn; echo

  trap "rm -fr $bn" EXIT SIGINT SIGTERM

  rm -fr $bn; cargo new $bn

  cp $fn $bn/src/main.rs

  [ -f $tn ] && cp $tn $bn/Cargo.toml

  ( cd $bn; echo
    if [ "$bn" == "genai" ]; then
      model=$1; shift; prompt="$@"
      cargo run $model $prompt
    else
      cargo run
    fi )

done | tee compile-run-all.out
