---@diagnostic disable: undefined-global

-- local sender = require("event_sender")

-- sender:enter()
-- sender:enter()
-- sender.enter()

local network = require("network")

local r = network.request("https://httpbin.org/get", {
    method = "POST",
    headers = {
        ["Content-Type"] = "application/json",
        ["user-agent"] = "curl/7.64.1"
    },
})

print(r.text)
