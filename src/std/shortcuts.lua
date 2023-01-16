require("event_handler")

-- this module requires you to run StartHandler() in order for shortcuts to work

local shortcuts = {}
local keydowns = {}

EventHandler:addEventListener("key_down", function(event)
    keydowns[#keydowns+1] = event.key
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

EventHandler:addEventListener("key_up", function(event)
    for i = 1, #keydowns do
        if (keydowns[i] == event.key) then
            table.remove(keydowns, i)
            break
        end
    end
end)

---comment
---@param shortcut string keys to be matched in order `Ctrl.F8` for example
---@param callback function the function to run
function AddShortcut(shortcut, callback)
    shortcuts[shortcut] = callback
end