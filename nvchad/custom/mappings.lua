---@type MappingsTable
local M = {}

M.general = {
  n = {
    [";"] = { ":", "enter command mode", opts = { nowait = true } },
    ["<C-S-j>"] = { "<Nop>", "入力切り替えのショートカットに反応しないようにする", opts = { nowait = true } },
    ["<C-S-;>"] = { "<Nop>", "入力切り替えのショートカットに反応しないようにする", opts = { nowait = true } },
  },
  i = {
    [";<ESC>"] = { "<Cmd>lua require('insert-end-semicolon').insertEndSemicolon()<CR><ESC>", "insert 「;」 at the end of line", opts = { nowait = true } },
    ["<C-S-j>"] = { "<Nop>", "入力切り替えのショートカットに反応しないようにする", opts = { nowait = true } },
    ["<C-S-;>"] = { "<Nop>", "入力切り替えのショートカットに反応しないようにする", opts = { nowait = true } },
  },
}

-- more keybinds!

return M
