# Settings > MCP server: AI assistants on this computer (Claude Desktop,
# Claude Code, LM Studio, …) using the mail through `katnactl mcp`. MCP is
# the Model Context Protocol, which AI assistants use; keep "MCP".

mcp-assistants = AI assistants
mcp-assistants-switch = Let AI assistants on this computer use your mail
mcp-assistants-switch-detail = Claude Desktop, Claude Code, LM Studio and others can search and read your mail when you ask them to.
mcp-assistants-off = Off: assistants that try are told it is turned off here.

mcp-drafts = Drafts
mcp-drafts-switch = Let them save drafts
mcp-drafts-switch-detail = A draft waits in Drafts for you to check and send. Assistants never send, delete or move mail.

mcp-accounts = Accounts
mcp-accounts-detail = The accounts whose mail assistants can search and read.

mcp-connect = Connect an assistant
mcp-client-other = Other
# Claude Desktop's own menus, which stay in English there.
mcp-connect-claude-desktop = In Claude Desktop, open Settings > Developer > Edit Config and add this, then restart Claude Desktop:
mcp-connect-claude-code = In a terminal, run:
mcp-connect-lm-studio = In LM Studio, open Program > Install > Edit mcp.json and add this:
mcp-connect-other = Any assistant that runs MCP servers on this computer can start Katna's with this command:
mcp-copy = Copy
mcp-copied = Copied

mcp-recently = Recently
mcp-recently-detail = What assistants did, kept on this computer only.
mcp-recently-none = Nothing yet.
mcp-recently-clear = Clear
# { $client } is the assistant ("Claude Desktop"); "An assistant" when it gave no name.
mcp-someone = An assistant
mcp-did-search = { $client } searched for “{ $query }”
mcp-did-search-all = { $client } looked at the newest mail
mcp-did-read = { $client } read “{ $subject }”
mcp-did-draft = { $client } saved a draft to { $to }
mcp-did-draft-nobody = { $client } saved a draft
