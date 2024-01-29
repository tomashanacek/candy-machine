#!/bin/bash

declare -a contracts=("candy-machine")

for i in "${contracts[@]}"
do
    echo "$i"
    cd contracts/$i
    cargo schema
    cd ../../
done

declare -a packages=("cw3")

for i in "${packages[@]}"
do
    echo "$i"
    cd packages/$i
    cargo schema
    cd ../../
done