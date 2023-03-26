---@diagnostic disable: lowercase-global
local versions = require("versions")
local kv = require("kv")

a2m8 = {}

a2m8.version = versions.a2m8;
a2m8.kv = kv

return a2m8
