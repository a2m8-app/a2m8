---@diagnostic disable: undefined-global

EventHandler = { eventHandlers = {} }
--- 
--- Runs the function on every event trigger
---
---@param eventName string
---@param eventHandler function
function EventHandler:addEventListener(eventName, eventHandler)
    if (self.eventHandlers[eventName] == nil) then
        self.eventHandlers[eventName] = {}
    end
    local evenHanlders = self.eventHandlers[eventName]
    evenHanlders[#evenHanlders+1] = eventHandler
end

---
--- This function will invoke all the event handlers for the given event
---
---@param eventName string the name of the event this can be anything or a event from the StartHandler events
---@param param function the function that will be called when the event is triggered
function EventHandler:invoke(eventName, param)
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
function StartHandler()
    while (true) do
        local event = event_handler:read()
        EventHandler:invoke(event.name, event)
    end
end
