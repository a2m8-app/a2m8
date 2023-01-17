local events = require("event_sender_internal")
local u = require("utils")
local sim = events.simulate

---
---@param word string
function events:type(word)
    for i = 1, #word do
        local letter = string.sub(word, i, i);
        local name = "Key" .. letter:upper()
        if (name == "Key ") then
            name = "Space"
        end
        if letter:upper() == letter then
            sim(events.create_key_press("ShiftLeft"))
        end
        sim(events.create_key_press(name))
        sim(events.create_key_release(name))
        if letter:upper() == letter then
            sim(events.create_key_release("ShiftLeft"))
        end
    end
end

---comment
---@param x number
---@param y number
---@param time number time in seconds
function events:move_to(x, y, time)
    -- use u.sleep(time) to sleep
end

return events
