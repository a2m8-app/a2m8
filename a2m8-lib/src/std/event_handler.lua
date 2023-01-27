---@diagnostic disable: undefined-global

local internal = require("event_handler_internal")

local eventHandler = { eventHandlers = {} }
---
--- Runs the function on every event trigger
---
---@param eventName string
---@param callback function
function eventHandler:addEventListener(eventName, callback)
    if (self.eventHandlers[eventName] == nil) then
        self.eventHandlers[eventName] = {}
    end
    local eventHandlers = self.eventHandlers[eventName]
    eventHandlers[#eventHandlers + 1] = callback
end

---
--- This function will invoke all the event handlers for the given event
---
---@param eventName string the name of the event this can be anything or a event from the StartHandler events
---@param param function the function that will be called when the event is triggered
function eventHandler:invoke(eventName, param)
    if (self.eventHandlers[eventName] == nil) then
        return
    end
    for i = 1, #self.eventHandlers[eventName] do
        self.eventHandlers[eventName][i](param)
    end
end

---
--- This function will start the event handler
--- This is a blocking function that calls event_handler:read() and then calls EventHandler:invoke(event.name, event)
---
--- TODO: make it so new events can be received while EventHandler:invoke is being called
local function startHandler()
    while (true) do
        local event = internal.read()
        eventHandler:invoke(event.name, event)
    end
end

local function start_handler_coroutine()
    return coroutine.create(function()
        while (true) do
            coroutine.yield(internal.read())
        end
    end)
end

return {
    eventHandler = eventHandler,
    startHandler = startHandler,
    internal = internal,
    start_handler_coroutine = start_handler_coroutine
}
