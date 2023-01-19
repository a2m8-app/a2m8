---@diagnostic disable: undefined-global

-- local sender = require("event_sender")

-- sender:enter()
-- sender:enter()
-- sender.enter()

local network = require("network")
local json = require("json")

local r = network.request("https://ipinfo.io", {
    headers = {
        ["User-Agent"] = "curl/7.64.1",
        ["Accept"] = "application/json"
    }
})

local table = json.parse(r.text)
print("Your IP is: " .. table["ip"])

