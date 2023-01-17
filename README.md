## Feature support

| Feature             | Windows | Linux |     Mac     |
| :------------------ | :-----: | :---: | :---------: |
| Clipboard           |    X    |   X   |      O      |
| Listening to events |    X    | X/x11 |      X      |
| Screenshots         |    X    |   X   |      X      |
| Command             |    X    |   X   |      X      |
| Audio               |    X    |   X   |      X      |
| Notifications       |    X    |   X   | X (partial) |

## List of modules

You can import modules with require("module_name")

| Module name    | Description                                                                                |
| :------------- | :----------------------------------------------------------------------------------------- |
| clipboard      | Clipboard module                                                                           |
| display        | View screens and make screenshots                                                          |
| events_handler | Events module                                                                              |
| sleep          | Add the function Sleep(time_ms) to wait this is more efficient than using pure lua sleep   |
| versions       | Version info                                                                               |
| command        | Run shell commands with deno shell include a few built in commands for cross compatibility |

## Contributing

This is a fairly new project and I'm very new to lua so if you know better ways to do things / follow conventions / etc. please let me know with a github issue.

## TODO:

- figure out how to make src/std work without having the files in the filesystem [x] partially done rust-embed still needs to be implemented

## Dependencies

ubuntu/debian:

```sh
libxi-dev libxtst-dev libxcb-composite0-dev librust-alsa-sys-dev
```
