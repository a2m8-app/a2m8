---@diagnostic disable: undefined-global

local c = require("clipboard")
local u = require("utils")
print("settings")
c.clipboard.set("test")

u.sleep(20)

-- local dpath = u.find_app("discord")
-- print("discord is at " .. dpath)
-- print("opening discord")
-- u.open_app("discord")
