function update 
  if type brew > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'brew'
    echo '########'
    brew update -v && brew upgrade -v
  end
  if type apt-get > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'apt'
    echo '########'
    sudo apt update && sudo apt upgrade
  end
  if type volta > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'volta'
    echo '########'
    volta install node
  end
  if type rustup > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'rustup'
    echo '########'
    rustup update
  end
  if cargo install-update --version > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'cargo'
    echo '########'
    cargo install-update -a
  end
  if type pyenv > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'pyenv'
    echo '########'
    pyenv update
  end
end
