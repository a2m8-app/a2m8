---@diagnostic disable: undefined-global

local u = require("utils")

local dpath = u.find_app("discord")
print("discord is at " .. dpath)
print("opening discord")
u.open_app("discord")
