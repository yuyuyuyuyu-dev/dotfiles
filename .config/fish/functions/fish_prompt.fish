function fish_prompt --description 'Write out the prompt'
    echo

    # Host
    set_color $fish_color_host
    echo -n (prompt_hostname)
    set_color normal

    echo -n ': '

    # User
    set_color $fish_color_user
    echo -n (whoami)
    set_color normal

    echo -n '$ '
    set_color normal
end
