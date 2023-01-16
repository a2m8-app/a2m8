---@diagnostic disable: undefined-global
require("std/event_handler")
require("std/sleep")

EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)

Sleep(4)

StartHandler()
