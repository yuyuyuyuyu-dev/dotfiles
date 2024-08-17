return {
  {
    "yu-ko-ba/insert-end-semicolon.nvim",
    lazy = false,
    config = function()
      vim.keymap.set("i", ";<Esc>", "<Cmd>lua require('insert-end-semicolon').insertEndSemicolon()<CR><ESC>", {
        desc = "insert 「;」 at the end of line",
        nowait = true,
      })
    end,
  },
}
