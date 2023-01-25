--ex
local function misc()
    local events = require("event_handler")
    local shortcuts = require("shortcuts")

    events.eventHandler:addEventListener("key_press", function(event)
        print(event.key)
    end)
    shortcuts.addShortcut("F8", function()
        print("you died!")
    end)

    events.startHandler()
end

--ex
local function trollMouse()
    local e = require("event_sender")
    local h = require("event_handler")
    local u = require("utils")

    h.eventHandler:addEventListener("mouse_move", function(event)
        print(event.x, event.y)
        if math.random(0, 200) ~= 1 then
            return
        end
        e.simulate(
            e.create_mouse_move(event.x + math.random(-50, 50), event.y + math.random(-50, 50))
        )
        u.sleep(0.02)
    end)

    h.startHandler()
end

--ex
local function playAudio()
    local audio = require("audio")
    local u = require("utils")

    audio.play_audio("../assets/fireworks_launch_boom.wav")
    audio.play_audio("../assets/assets_music.mp3")
    u.sleep(1)
    audio.play_audio_blocking("../assets/fireworks_launch_boom.wav")
    audio.play_audio_blocking("../assets/assets_music.mp3")
end

--ex
local function sendNotifaction()
    local n = require('notify')
    local u = require('utils')
    n:new()
        :appname("This is my amazing app")
        :summary("You died")
        :body("press f to try again")
        :icon("spotify")
        :show()


    u.sleep(2.5)
end

--ex
local function logging()
    local l = require("log")
    l.warn("huge warning something broke!")
    l.info("cool info")
    l.error("Error beep boop")
    l.debug("debug only works with RUST_LOG=DEBUG")
    l.trace("tracing o.0 RUST_LOG=TRACE")
end

--ex
local function networking()
    local network = require("network")

    print(
        network.fetch_text("POST", "https://httpbin.org/post", "deez nuts")
    )
end

--ex
local function tipping()
    local s = require('event_sender')
    s:type("Hello World")
end

--ex
local function env_stuff()
    local env = require("env")

    for k, v in pairs(env) do
        print(k, v)
    end
end

--ex
local function block_typing()
    local internal = require("event_handler_internal")
    while (true) do
        internal.event_handler.grab(function(event)
            if event.key == "F8" then
                os.exit()
            end
            if event.name == "key_press" and event.key ~= nil then
                return nil
            end
            return event
        end)
    end
end

--ex
local function ip_info()
    local network = require("network")
    local json = require("json")

    local r = network.request("https://ipinfo.io", {
        headers = {
            ["User-Agent"] = "curl/7.64.1",
            ["Accept"] = "application/json"
        }
    })

    local table = json.parse(r.text)
    print("Your IP is: " .. table["ip"])
end

--ex
local function performance()
    local network = require("network")
    local json = require("json")
    local u = require("utils")

    local perf = u.performance.now()
    local r = network.request("https://ipinfo.io", {
        headers = {
            ["User-Agent"] = "curl/7.64.1",
            ["Accept"] = "application/json"
        }
    })



    local table = json.parse(r.text)
    print("Your IP is: " .. table["ip"])
    print("took " .. perf:elapsed() .. "s")
end

--ex
local function app_starting()
    local u = require("utils")

    local dpath = u.find_app("discord")
    print("discord is at " .. dpath)
    print("opening discord")
    u.open_app("discord")
end
