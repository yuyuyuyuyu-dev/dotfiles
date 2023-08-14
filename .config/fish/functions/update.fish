function update 
  if type brew > /dev/null 2>&1
    echo '########'
    echo 'brew'
    echo '########'
    brew update -v && brew upgrade -v
  end
  if type apt-get > /dev/null 2>&1
    echo '########'
    echo 'apt'
    echo '########'
    sudo apt update && sudo apt upgrade
  end
  if type volta > /dev/null 2>&1
    echo '########'
    echo 'volta'
    echo '########'
    volta install node
  end
  if type rustup > /dev/null 2>&1
    echo '########'
    echo 'rustup'
    echo '########'
    rustup update
  end
  if cargo update -h > /dev/null 2>&1
    echo '########'
    echo 'cargo'
    echo '########'
    cargo update -v
  end
end
