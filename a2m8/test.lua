--[[
@description Sending a notification is easy as this
--]]



local n = require('notify')
local u = require('utils')
n:new()
    :appname("This is my amazing app")
    :summary("You died")
    :body("press f to try again")
    :icon("spotify")
    :show()


u.sleep(2.5)
