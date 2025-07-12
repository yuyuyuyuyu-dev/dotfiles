linuxbrew_path="/home/linuxbrew/.linuxbrew/bin/brew"
if [ -e ${linuxbrew_path} ]; then
	eval "$(${linuxbrew_path} shellenv)"
fi

export VOLTA_HOME="$HOME/.volta"
export PATH="$VOLTA_HOME/bin:$PATH"

if [[ -e "$HOME/.cargo/env" ]]; then
	. "$HOME/.cargo/env"
fi

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
