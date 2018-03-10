#!/bin/bash

echo $@
outputdir=$(cargo build "$@" --message-format=json | tail -n 1 | jq '.filenames|.[0]' | cut -c 2- | rev |  cut -d'/' -f2- | rev )
echo $outputdir
cp -R -t $outputdir "./src/gui/assets" 