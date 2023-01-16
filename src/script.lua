---@diagnostic disable: undefined-global


-- io.write('Hello, what is your name? ')
-- local name = io.read()
-- io.write('Nice to meet you, ', name, '!\n')

require("event_handler")
require("shortcuts")

AddShortcut("ControlLeft.F8", function()
    print("F8-S")
end)
EventHandler:addEventListener("key_press", function(event)
    -- print(event.key)
end)
StartHandler()
