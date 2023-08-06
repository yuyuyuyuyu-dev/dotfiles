---@type MappingsTable
local M = {}

M.general = {
  n = {
    [";"] = { ":", "enter command mode", opts = { nowait = true } },
  },
  i = {
    [";<ESC>"] = { "<Cmd>lua require('insert-end-semicolon').insertEndSemicolon()<CR><ESC>", "insert 「;」 at the end of line", opts = { nowait = true } },
    ["<C-S-j>"] = { "<Nop>", "", opts = { nowait = true } },
    ["<C-S-;>"] = { "<Nop>", "", opts = { nowait = true } },
  },
}

-- more keybinds!

return M
