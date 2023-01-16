-- function Make_screenshots()
--     require("display")
--     local i = 1
--     local screens = display.screens
--     for var in pairs(screens) do
--         screens[var]:capture("hi" .. i .. ".png")
--         screens[var]:capture_area(0, 20, 40, 40, "hi-point" .. i .. ".png")
--         i = i + 1
--     end
-- end

-- function Manual_event_handler()
--     require("event_handler")
--     while (true) do
--         local event = event_handler:read()
--         if event.name == "mouse_move" then
--             -- print(event.x, event.y)
--         elseif event.name == "key_press" then
--             print(event.key)
--             print("F1")
--             if event.key == "Escape" then
--                 print("Stopping")
--                 break
--             elseif event.key == "F1" then
--                 print("F1 screenshot time")
--                 Make_screenshots()
--             end
--         end
--     end
-- end

-- function Clipboard_methods()
--     require("clipboard")
--     print(clipboard.value)
--     clipboard.value = "lol"
--     clipboard.clear()
-- end

-- function Version_info()
--     require("versions")
--     print(version_info.version)
--     print(version_info.lua)
-- end

-- function Run_commands()
--     require('command')
--     local result = run_command("cat main.rs")

--     print(result.stdout)
--     print(result.stderr)
--     print(result.exit_code)
--     local code = run_command_piped("cat main.rs")

--     print(code)
-- end
local function misc()
    local events = require("event_handler")
    local shortcuts = require("shortcuts")

    events.eventHandler:addEventListener("key_press", function(event)
        print(event.key)
    end)
    shortcuts.addShortcut("F8", function()
        print("you died!")
    end)

    events.startHandler()
end

local function trollMouse()
    local e = require("event_sender")
    local h = require("event_handler")
    local u = require("utils")

    h.eventHandler:addEventListener("mouse_move", function(event)
        print(event.x, event.y)
        if math.random(0, 200) ~= 1 then
            return
        end
        e.simulate(
            e.create_mouse_move(event.x + math.random(-50, 50), event.y + math.random(-50, 50))
        )
        u.sleep(0.02)
    end)

    h.startHandler()
end
