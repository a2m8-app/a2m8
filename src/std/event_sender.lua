local events = require("event_sender_internal")
local u = require("utils")
local sim = events.simulate

---
---@param word string
function events:type(word)
    for i = 1, #word do
        sim(events.create_key_press(string.sub(word, i, i)))
        sim(events.create_key_release(string.sub(word, i, i)))
    end
end
---comment
---@param x number
---@param y number
---@param time number time in seconds
function events:move_to(x,y,time)
    -- use u.sleep(time) to sleep
end

return events
