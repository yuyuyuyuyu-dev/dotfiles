local on_attach = require("plugins.configs.lspconfig").on_attach
local capabilities = require("plugins.configs.lspconfig").capabilities

local lspconfig = require("lspconfig")

-- if you just want default config for the servers then put them in a table
local servers = {
  "html",
  "cssls",
  "tsserver",
  "clangd",
  "bashls",
  "gopls",
  "rust_analyzer",
  "pylsp",
  "angularls",
}

for _, lsp in ipairs(servers) do
  lspconfig[lsp].setup({
    on_attach = on_attach,
    capabilities = capabilities,
  })
end

-- Angular
local node_modules = vim.fn.stdpath("data") .. "/mason/packages/angular-language-server/node_modules"
local ngls_cmd = {
  node_modules .. "/@angular/language-server/bin/ngserver",
  "--stdio",
  "--tsProbeLocations",
  node_modules .. "/typescript/lib",
  "--ngProbeLocations",
  node_modules .. "/@angular/language-server/bin",
}

lspconfig.angularls.setup({
  cmd = ngls_cmd,
  on_attach = on_attach,
  capabilities = capabilities,
  on_new_config = function(new_config, _)
    new_config.cmd = ngls_cmd
  end,
})

--
-- lspconfig.pyright.setup { blabla}
