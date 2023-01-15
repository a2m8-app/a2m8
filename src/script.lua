function Make_screenshots()
    local i = 1
    local screens = display.screens
    for var in pairs(screens) do
        screens[var]:capture("hi" .. i .. ".png")
        screens[var]:capture_area(0, 20, 40, 40, "hi-point" .. i .. ".png")
        i = i + 1
    end
end
