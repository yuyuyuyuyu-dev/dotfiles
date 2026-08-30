if test -e ~/.zlogout && [ ! ~/.zlogout -ot ~/.zlogout.zwc ]; then
  zcompile ~/.zlogout
  if type terminal-notifier; then
    terminal-notifier -message '.zlogout was compiled!'
  else
    echo '.zlogout was compiled!'
  fi
fi

if test -e ~/.zprofile && [ ! ~/.zprofile -ot ~/.zprofile.zwc ]; then
  zcompile ~/.zprofile
  if type terminal-notifier; then
    terminal-notifier -message '.zprofile was compiled!'
  else
    echo '.zprofile was compiled!'
  fi
fi

if test -e ~/.zshenv && [ ! ~/.zshenv -ot ~/.zshenv.zwc ]; then
  zcompile ~/.zshenv
  if type terminal-notifier; then
    terminal-notifier -message '.zshenv was compiled!'
  else
    echo '.zshenv was compiled!'
  fi
fi

if test -e ~/.zshrc && [ ! ~/.zshrc -ot ~/.zshrc.zwc ]; then
  zcompile ~/.zshrc
  if type terminal-notifier; then
    terminal-notifier -message '.zshrc was compiled!'
  else
    echo '.zshrc was compiled!'
  fi
fi
