function fish_prompt --description 'Write out the prompt'
  set exit_status {$status}

  set_color $fish_color_user

  echo

  if [ {$exit_status} -ne 0 ]
    echo -n ' '
    echo -n (date '+%H')
    echo -n ':'
    echo -n (date '+%M')
    echo -n ' '
    echo -n '💀 '
  else
    echo -n ' '
    echo -n (date '+%H')
    echo -n ':'
    echo -n (date '+%M')
    echo -n ' '
    echo -n '🍓 '
  end

  echo -n '$ '
set_color normal

  # Host
  #   set_color $fish_color_host
  #   echo -n (prompt_hostname)
  #   set_color normal

  # echo -n ': '

  # User
  # set_color $fish_color_user
  # echo -n (whoami)
  # set_color normal

  # echo -n '$ '
  # set_color normal
end
