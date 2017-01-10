#!/usr/bin/zsh

# if running bash
if [ -n "$BASH_VERSION" ]; then
    # include .bashrc if it exists
    if [ -f "$HOME/.bashrc" ]; then
    . "$HOME/.bashrc"
    fi
fi

# if running zsh
if [ -n "$ZSH_VERSION" ]; then
    # include .zshrc if exists
    if [ -f "$HOME/.zshrc" ]; then
    . "$HOME/.zshrc"
    fi
fi

# set PATH so it includes user's private bin if it exists
if [ -d "$HOME/bin" ] ; then
    PATH="$HOME/bin:$PATH"
fi

## user defined variable

# set GTAGSLIBPATH
# man global for details
if [ -z "$GTAGSLIBPATH" ]; then
    export GTAGSLIBPATH="/usr/include:/usr/local/include"
else
    export GTAGSLIBPATH="$GTAGSLIBPATH:/usr/include:/usr/local/include"
fi


# set GOPATH
if [ -z "$GOPATH" ]; then
    export GOPATH="$HOME/gohome"
    PATH="$GOPATH/bin:$PATH"
fi

# pyenv PATH
if [ -f "$HOME/.pyenv/bin/pyenv" ]; then
    export PYENV_ROOT="$HOME/.pyenv"
    PATH="$PYENV_ROOT/bin:$PATH"
fi

# cabal
if [ -f "$HOME/.cabal" ]; then
    PATH="$HOME/.cabal/bin:$PATH"
fi

export DISABLE_AUTO_UPDATE="true"

# colored GCC warnings and errors
export GCC_COLORS='error=01;31:warning=01;35:note=01;36:caret=01;32:locus=01:quote=01'

# set TERM for emacs --no-window-system
export TERM=xterm-256color
