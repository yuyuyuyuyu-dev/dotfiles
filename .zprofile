if [ -f "/opt/homebrew/bin/brew" ]; then
    eval "$(/opt/homebrew/bin/brew shellenv)"
fi

if [ -e "$HOME/Library/Android/sdk" ]; then
    export ANDROID_HOME="$HOME/Library/Android/sdk"
fi

case "$OSTYPE" in
    darwin*)
        ;;
    linux*)
        ;;
esac

export PATH="$HOME/.local/bin:$PATH"
