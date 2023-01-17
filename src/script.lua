---@diagnostic disable: undefined-global

local l = require("log")
local network = require("network")

print(
    network.fetch_text("POST", "https://httpbin.org/post", "deez nuts")
)

