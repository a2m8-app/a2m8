---
title: "Creating a module"
description: "Info on how to create a module"
---

## Prerequisites

- Some coding knowledge is not required since lua is very easy to use but it can help a bit.
- Git is needed if you want to edit the module with a ide like vscode or intelij idea.
- A2M8 is needed to run the module.

## Creating a module

You can start by creating a account on the [A2M8 Gitea instance](https://a2m8-git.tricked.dev) and then by using the [starting template](https://a2m8-git.tricked.dev/tricked/script-template).

Clone the repo you just created from the template

```sh
git clone https://a2m8-git.tricked.dev/yourname/yourmodule
```

Then you can start by modifying the module.lua with your own code for example:

> the workspace has a list of example modules too you can use reference

```lua
local shortcuts = require("shortcuts")
local e = require("event_handler")

shortcuts.addShortcut("ControlLeft.keyl", function()
    print("Omg the legendary keyl keybinding is here!")
end)

while true do
    local event = e.internal.read()
    -- uncomment this to see what keys are being pressed
    -- print(event.key)
    e.eventHandler:invoke(event.name, event)
end
```

Then you can run the module by running the following command in the root of the module to run your newly created module

```sh
a2m8 run module.lua
```

You can then run `a2m8 add module.lua` to install the module or just open the A2M8 ui and do it from there.

## Adding a module to the A2M8 module list

To get your module added to the workshop modules all you have to do is tag your script with `script` this will make it show up in the workshop. You can do this by clicking "Manage Topics" under the repository description or above the commits count info.

## A2M8 cli

```
A Tauri App

Usage: a2m8 [OPTIONS] [COMMAND]

Commands:
  run      Run a script from path
  open     Open the tauri UI
  list     List all the scripts
  add      Add a file to the list of scripts
  delete   Remove a script this requires the id you can view it in list sub
  start    Start
  inspect  Inspect
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --data-dir <DATA_DIR>  [env: DATA_DIR=]
  -h, --help                 Print help
  -V, --version              Print version
```

## mode documentation

go to [a2m8_lib](/lib/a2m8_lib) for a list of all native modules in a2m8
