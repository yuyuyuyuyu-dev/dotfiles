PS1='\[\e[1;34m\]🍓 \$ \[\e[m\]'

alias ls='ls -F'
alias la='ls -aF'
alias ll='ls -lhF'
alias arisu='echo "🍓 橘です！"'
alias arichu='arisu'

if [[ -e "$HOME/.cargo/env" ]]; then
	. "$HOME/.cargo/env"
fi
