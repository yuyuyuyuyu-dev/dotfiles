# プロンプトの設定
update_prompt() {
    local exit_status=$?
    local color
    if [ $exit_status -eq 0 ]; then
        color="\e[38;2;187;222;251m" # #bbdefb
    else
        color="\e[38;2;220;20;60m" # #dc143c
    fi
    PS1="🍓 \[$color\]❯ \[\e[m\]"
}
PROMPT_COMMAND=update_prompt


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
