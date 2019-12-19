local ret_status="%(?:%{$fg_bold[green]%}%?:%{$fg_bold[red]%}%?%s)"
#PROMPT='${ret_status}%{$fg_bold[green]%} %* %{$fg[cyan]%}%n@%M:%0~%{$fg_bold[blue]%} $(git_prompt_info)%{$fg_bold[blue]%} % %{$reset_color%}'
PROMPT='${ret_status}%{$fg_bold[green]%} %* %{$fg[cyan]%}%n@%M:%0~%{$fg_bold[blue]%} $(git_prompt_info)%{$fg_bold[blue]%}%{$reset_color%}
%{$fg_bold[cyan]%}→  %{$reset_color%}'

ZSH_THEME_GIT_PROMPT_PREFIX="git:(%{$fg[red]%}"
ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%}"
ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[blue]%})%{$fg[yellow]%}X%{$reset_color%}"
ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[blue]%})"
