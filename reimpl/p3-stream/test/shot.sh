#!/bin/bash
# Uso: shot.sh DIR SEGUNDOS "guion" [tamaño]  -> DIR/last.png = último fotograma recibido
D=$1; mkdir -p $D
P3_PASSWORD=${P3_PASSWORD:-prueba-1234} python3 $(dirname $0)/client.py --out $D --secs $2 --guion "$3" --size ${4:-1024x768} | tail -3
N=$(cat $D/lastn); S=$(cat $D/lastseg)
ffmpeg -y -loglevel error -f h264 -i $D/video$S.h264 -vf "select=eq(n\,$N)" -vframes 1 $D/last.png
