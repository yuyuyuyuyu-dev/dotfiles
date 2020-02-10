function fish_prompt --description 'Write out the prompt'
  set exit_status {$status}

  set_color $fish_color_user

  echo

  if [ {$exit_status} -ne 0 ]
    echo '       _,／￣￣`￣＼､/ﾚ'
    echo '    ／/    ,   /＼  .i i Ｖ〈'
    echo '   / /   ∠ﾑ/ ー-V l  「ヽ'
    echo '     j v､!●    ● i  '  ├''
    echo '  ／   〈     ワ   /  .i y\''
    echo '/ _ ,.イ , `ｰｩ   ｔ-!,､_У'
    echo '´  \'  .ﾚ^V´  Ｖ_,ィtｰ〈    ｢| ｢|'
    echo '         /  `央ー\'j    ＼_|:| |:|'
    echo -n '       ,/ー､{,_ﾉ  /ー､!   ＼::::]'
  else
    echo -n ' '
    echo -n (date '+%H')
    echo -n ':'
    echo -n (date '+%M')
    echo -n ' '
    echo -n (whoami)
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
