function fish_right_prompt
    # PWD
    set_color $fish_color_cwd
    echo -n (prompt_pwd)
    set_color normal
end
