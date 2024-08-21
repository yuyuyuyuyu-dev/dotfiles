return {
  {
    "yu-ko-ba/insert-end-semicolon.nvim",
    lazy = true,
    keys = {
      {
        ";<Esc>",
        "<Cmd>lua require('insert-end-semicolon').insertEndSemicolon()<CR><ESC>",
        mode = "i",
        desc = "insert 「;」 at the end of line",
      },
    },
  },
}
