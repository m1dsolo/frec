# zsh initialization
function e() {
    if [ -z "$1" ]; then
        frec_editor_fzf
    else
        frec_editor "$1"
    fi
}

zle -N e
bindkey '^e' e
