local sleep_module = require('sleep')
local function prompt(question, default)
    print(question)
    local result = default
    local input = io.read()
    if input ~= "" then
        result = input
    end
    return result
end

--- https://stackoverflow.com/questions/12069109/getting-input-from-the-user-in-lua
local function read()
    local input = io.read()
    return input
end

--- Sleep a amount of seconds periods work
--- @param seconds number
local function sleep(seconds)
    sleep_module.sleep(seconds)
end

local function loop(fun)
    while true do
        fun()
    end
end

return {
    prompt = prompt,
    read = read,
    sleep = sleep,
    loop = loop
}
