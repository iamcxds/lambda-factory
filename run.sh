#!/bin/bash
args=("$@")
if [[ "$1" == "watch" ]]; then
num=$#-1
 cargo watch -x "${args[@]:1:num} --target x86_64-pc-windows-gnu"
else
 cargo "$@" --target x86_64-pc-windows-gnu
fi


