---@diagnostic disable: undefined-global


local events = require("event_handler")
local shortcuts = require("shortcuts")

events.eventHandler:addEventListener("key_press", function(event)
    print(event.key)
end)
shortcuts.addShortcut("F8", function()
    print("you died!")
end)

events.startHandler()
