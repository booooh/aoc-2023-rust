#! /bin/bash -e

DAY=$(printf "%02d" $1)
CRATE=day${DAY}

# add crate to workspace members
sed -i -e '/# placeholder/i "'${CRATE}'",' Cargo.toml

# create new crate
cargo new $CRATE
carg add --package ${CRATE} --path ../common

# check that things build
cargo build

cd $CRATE ; aoc -d ${DAY} d ; mv input input_part1 ; cd ..


git add Cargo.toml
git add $CRATE
git commit -m "feat: adding ${CRATE}"
  
