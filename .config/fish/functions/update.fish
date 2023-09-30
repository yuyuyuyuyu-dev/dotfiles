function update 
  if type -q brew
    echo
    echo
    echo '########'
    echo 'brew'
    echo '########'
    brew update -v && brew upgrade -v
  end
  if type -q apt-get
    echo
    echo
    echo '########'
    echo 'apt'
    echo '########'
    sudo apt update && sudo apt -y upgrade
  end
  if type -q volta
    echo
    echo
    echo '########'
    echo 'volta'
    echo '########'
    volta install node
    volta install npm
  end
  if type -q rustup
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
  if pyenv update --help > /dev/null 2>&1
    echo
    echo
    echo '########'
    echo 'pyenv'
    echo '########'
    pyenv update
  end
end
