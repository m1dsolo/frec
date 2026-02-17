#!/bin/sh

frec add --table=editor "$1"
$EDITOR "$1"
