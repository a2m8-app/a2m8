-- this module requires you to run StartHandler() in order for shortcuts to work

local e = require("event_handler")

local shortcuts = {}
local keydowns = {}

e.eventHandler:addEventListener("key_press", function(event)
    keydowns[#keydowns + 1] = event.key:lower()
    local shortcut = ""
    for i = 1, #keydowns do
        shortcut = shortcut .. keydowns[i]
        if (i ~= #keydowns) then
            shortcut = shortcut .. "."
        end
    end
    if (shortcuts[shortcut] ~= nil) then
        shortcuts[shortcut]()
    end
end)

e.eventHandler:addEventListener("key_release", function(event)
    local key = event.key:lower()
    for i = 1, #keydowns do
        if (keydowns[i] == key) then
            table.remove(keydowns, i)
            break
        end
    end
end)

---comment
---@param shortcut string keys to be matched in order `Ctrl.F8` for example
---@param callback function the function to run
local function addShortcut(shortcut, callback)
    shortcuts[shortcut:lower()] = callback
end

return {
    addShortcut = addShortcut
}
