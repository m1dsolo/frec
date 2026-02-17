#!/bin/bash

selected=$(frec query --table=editor | fzf --scheme=path --layout=reverse --height=50% --border=rounded --bind "tab:down,shift-tab:up")
if [ -n "$selected" ]; then
    frec add --table=editor "$selected"
    $EDITOR "$selected"
fi
