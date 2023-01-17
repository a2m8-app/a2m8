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

### Development tip

only run with the features you want to test, for example if you want to test the clipboard module only run with the clipboard feature

```sh   
cargo run --features clipboard --no-default-features
```

## TODO:

- potentially switch out rdev for https://github.com/ostrosco/device_query
- potentially switch out clipboard libs for https://github.com/1Password/arboard
  
## Dependencies

ubuntu/debian:

```sh
libxi-dev libxtst-dev libxcb-composite0-dev librust-alsa-sys-dev
```
