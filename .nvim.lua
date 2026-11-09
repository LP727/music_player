local dap = require("dap")
dap.providers.configs["project"] = function(bufnr)
	return {
		{
			name = "Debug (Music_player)",
			type = "codelldb",
			request = "launch",
			program = require("utils.cargo_dap").program,
			cwd = "${workspaceFolder}",
			terminal = "external",
			env = { RUST_LOG = "debug" },
		},
	}
end
