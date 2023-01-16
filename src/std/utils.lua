function Prompt(prompt, default )
    print(prompt)
    local result = default
    local input = io.read()
    if input ~= "" then
        result = input
    end
    return result
end

--- https://stackoverflow.com/questions/12069109/getting-input-from-the-user-in-lua
function Read( prompt, default )
    io.write(prompt)
    return Prompt(prompt, default)
end

