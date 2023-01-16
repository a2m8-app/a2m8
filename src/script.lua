---@diagnostic disable: undefined-global

local audio = require("audio")
local u = require("utils")

audio.play_audio("../assets/fireworks_launch_boom.wav")
audio.play_audio("../assets/assets_music.mp3")
u.sleep(1)
audio.play_audio_blocking("../assets/fireworks_launch_boom.wav")
audio.play_audio_blocking("../assets/assets_music.mp3")
