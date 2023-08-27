PS1='\[\e[1;34m\]🍓 \$ \[\e[m\]'

alias ls='ls -FG'
alias la='ls -aFG'
alias ll='ls -FGl'
alias arisu='echo "🍓 橘です！"'
alias arichu='arisu'

if [[ -e "$HOME/.cargo/env" ]]; then
	. "$HOME/.cargo/env"
fi
