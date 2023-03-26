-- TODO rewrite this module to use sqlite or something
local json = require("json")
local env = require("env")
local u = require("utils")

local open = io.open

local function read_file(path)
    local file = open(path, "rb")  -- r read mode and b binary mode
    if not file then return nil end
    local content = file:read "*a" -- *a or *all reads the whole file
    file:close()
    return content
end

local kv = {
    file = env.A2M8_SCRIPT_DATA_DIR .. "kv.json",
    data = nil
}

local function prepare()
    local content = read_file(kv.file)
    if content then
        kv.data = json.parse(content)
    else
        kv.data = {}
    end
end

function kv.get(key)
    prepare();
    return kv.data[key]
end

function kv.set(key, value)
    prepare();
    kv.data[key] = value
    kv.setAll()
end

function kv.delete(key)
    prepare();
    kv.data[key] = nil
    kv.setAll()
end

function kv.setAll()
    local file = open(kv.file, "w")
    if not file then return nil end
    file:write(json.stringify(kv.data))
    file:close()
end

function kv.listKeys()
    prepare();
    local keys = {}
    for k, _ in pairs(kv.data) do
        table.insert(keys, k)
    end
    return keys
end

function kv.listValues()
    prepare();
    local values = {}
    for _, v in pairs(kv.data) do
        table.insert(values, v)
    end
    return values
end

function kv.list()
    prepare();
    return kv.data
end

return kv
