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
if [ -f "$HOME/.cabal" ]; then
    addpath PATH $HOME/.cabal/bin
fi

export DISABLE_AUTO_UPDATE="true"

# colored GCC warnings and errors
export GCC_COLORS='error=01;31:warning=01;35:note=01;36:caret=01;32:locus=01:quote=01'

# set TERM for emacs --no-window-system
export TERM=xterm-256color
