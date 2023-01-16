---@diagnostic disable: undefined-global

require("event_handler")
require("sleep")
require("versions")


EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)
EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)
EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)


print(version_info.version)
print(version_info.lua)