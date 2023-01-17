---@diagnostic disable: undefined-global

local internal = require("event_handler_internal")
local s = require('event_sender')
s:type("Hello World")
while (true) do
    internal.event_handler.grab(function(event)
        if event.key == "F8" then
            os.exit()
        end
        if event.name == "key_press" and event.key ~= nil then
            if match.random(0, 2) == 1 then
                s:type("Hello World")
            end
        end
        return event
    end)
end
