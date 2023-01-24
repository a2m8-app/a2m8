local module = require('utils_internal')
function module.prompt(question, default)
    print(question)
    local result = default
    local input = io.read()
    if input ~= "" then
        result = input
    end
    return result
end

--- https://stackoverflow.com/questions/12069109/getting-input-from-the-user-in-lua
function module.read()
    local input = io.read()
    return input
end

function module.loop(fun)
    while true do
        fun()
    end
end

function module.tprint(tbl, indent)
    if not indent then indent = 0 end
    for k, value in pairs(tbl) do
        local formatting = string.rep("  ", indent) .. k .. ": "
        if type(value) == "table" then
            print(formatting)
            module.tprint(value, indent + 1)
        elseif type(value) == 'boolean' or type(value) == "function" or type(value) == "userdata" then
            print(formatting .. tostring(value))
        else
            print(formatting .. value)
        end
    end
end

return module
