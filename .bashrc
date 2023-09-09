# プロンプトの設定
PS1='\[\e[1;34m\]🍓 \$ \[\e[m\]'


# エイリアスの設定
alias ls='ls -F'
alias la='ls -aF'
alias ll='ls -lhF'


# Rustの設定
if [[ -e "$HOME/.cargo/env" ]]; then
	. "$HOME/.cargo/env"
fi


# 関数を定義
arisu() {
	echo '🍓 橘です。'
}

arichu() {
	echo '🍓 橘です！'
}

tachibana-san() {
	echo '🍓 ありすでいいです。'
}
