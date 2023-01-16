---@diagnostic disable: undefined-global

require("event_handler")
require("sleep")

Sleep(0.5)
Sleep(0.5)
Sleep(0.5)
Sleep(0.5)
Sleep(0.5)
Sleep(0.5)

EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)
EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)
EventHandler:addEventListener("mouse_move", function(event)
    print(event.x, event.y)
end)

