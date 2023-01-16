local clock = os.clock
function Sleep(n)
    local t0 = clock()
    while clock() - t0 <= n do end
end
