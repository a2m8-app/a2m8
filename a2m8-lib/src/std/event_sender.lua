local events = require("event_sender_internal")
local u = require("utils")
local sim = events.simulate

---
---@param word string
function events.type(word)
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

function events.tab()
    sim(events.create_key_press("Tab"))
    sim(events.create_key_release("Tab"))
end

function events.enter()
    sim(events.create_key_press("Return"))
    sim(events.create_key_release("Return"))
end

---@param key string
function events.down(key)
    sim(events.create_key_press(key))
end

---@param key string
function events.up(key)
    sim(events.create_key_release(key))
end

---@param key string
---@param delay number
function events.press_key(key, delay)
    sim(events.create_key_press(key))
    if delay ~= nil then
        u.sleep(delay)
    end
    sim(events.create_key_release(key))
end

---@param key "left"|"right"|"middle"|string? key to press use a number if its a utility mouse button
---@param delay number?
function events.click(key, delay)
    sim(events.create_button_press(key or "left"))
    u.sleep(delay or 0.01)
    sim(events.create_button_release(key or "left"))
end

return events
