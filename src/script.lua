---@diagnostic disable: undefined-global

local l = require("log")

l.warn("huge warning something broke!")
l.info("cool info")
l.error("Error beep boop")
l.debug("debug only works with RUST_LOG=DEBUG")
l.trace("tracing o.0 RUST_LOG=TRACE")