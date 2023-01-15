---@diagnostic disable: undefined-global

function Make_screenshots()
    local i = 1
    local screens = display.screens
    for var in pairs(screens) do
        screens[var]:capture("hi" .. i .. ".png")
        screens[var]:capture_area(0, 20, 40, 40, "hi-point" .. i .. ".png")
        i = i + 1
    end
end

function Manual_event_handler()
    while (true) do
        local event = event_handler:read()
        if event.name == "mouse_move" then
            -- print(event.x, event.y)
        elseif event.name == "key_press" then
            print(event.key)
            print("F1")
            if event.key == "Escape" then
                print("Stopping")
                break
            elseif event.key == "F1" then
                print("F1 screenshot time")
                Make_screenshots()
            end
        end
    end
end

function Clipboard_methods()
    print(clipboard.value)
    clipboard.value = "lol"
    clipboard.clear()
end

Manual_event_handler()
