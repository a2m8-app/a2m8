---@diagnostic disable: undefined-global
local event = {}

EventHandler = { eventHandlers = {} }
function EventHandler:addEventListener(eventName, eventHandler)
    self.eventHandlers[eventName] = eventHandler
end

function EventHandler:invoke(eventName, param)
    if (self.eventHandlers[eventName] == nil) then
        return
    end
    self.eventHandlers[eventName](param)
end

function StartHandler()
    while (true) do
        local event = event_handler:read()
        EventHandler:invoke(event.name, event)
    end
end
