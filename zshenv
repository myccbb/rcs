#!/usr/bin/zsh

source "$HOME/.rcs/utils"

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/bin" ] ; then
    addpath PATH $HOME/bin
fi

## user defined variable

# set GTAGSLIBPATH
# man global for details
addpath GTAGSLIBPATH '/usr/local/include'
addpath GTAGSLIBPATH '/usr/include'

# detect gnu coreutils
if [ -f /usr/local/opt/coreutils/libexec/gnubin/ls ]; then
    # use gnu coreutils
    export PATH="/usr/local/opt/coreutils/libexec/gnubin:$PATH"
    export MANPATH="/usr/local/opt/coreutils/libexec/gnuman:$MANPATH"

    eval $(dircolors --sh)
fi

# add ~/.local/bin to PATH
addpath PATH $HOME/.local/bin

# set GOPATH
if [ -z "$GOPATH" ]; then
    addpath GOPATH $HOME/go
    addpath PATH $GOPATH/bin
fi

# pyenv PATH
if [ -f "$HOME/.pyenv/bin/pyenv" ]; then
    addpath PYENV_ROOT $HOME/.pyenv
    addpath PATH $PYENV_ROOT/bin
    eval "$(pyenv init -)"
fi

# cabal
if [ -d "$HOME/.cabal" ]; then
    addpath PATH $HOME/.cabal/bin
fi

# cargo
if [ -d "$HOME/.cargo" ]; then
    addpath PATH $HOME/.cargo/bin
fi

export DISABLE_AUTO_UPDATE="true"

# colored GCC warnings and errors
export GCC_COLORS='error=01;31:warning=01;35:note=01;36:caret=01;32:locus=01:quote=01'

# set TERM for emacs --no-window-system
export TERM=xterm-256color

if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

