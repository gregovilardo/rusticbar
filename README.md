# Installation and Setup

This is pretty experimental, i make this proyect to learn rust and without knowing gtk so probably not the best 
but for my use case this bar is really good and is not cpu intensive so if you want to give it a try:

1. **Install Dependencies**: Ensure all dependencies listed above are installed on your system by running the commands provided.

2. **Sway Configuration**: This bar is specifically designed for Sway. Make sure you're running Sway as your compositor.

3. **wl-gammarelay-rs**: The bar relies on wl-gammarelay-rs. Ensure it's running with the command `wl-gammarelay-rs run`.

---

You can send a SIGUSR1 and it will hide, for example you can add this on the sway config `bindsym Mod1+b exec $HOME/.config/sway/send_hide_signal_bar.sh`
where send_hide_signal_bar.sh is 
```bash
#!/bin/bash
# Send SIGUSR1 signal to rusticbar process
kill -SIGUSR1 $(pgrep rusticbar)
```

# Dependencies

To install the necessary dependencies, run the following commands:

```bash
sudo apt install libasound2-dev
sudo apt install network-manager
sudo apt install libdbus-1-dev
sudo apt install pkg-config
```

**gtk4-layer-shell** installation instructions on its [GitHub page](https://github.com/wmww/gtk4-layer-shell). (This is a must)

**wl-gammarelay-rs**. installation instructions on its [GitHub page](https://github.com/MaxVerevkin/wl-gammarelay-rs). (Not necessary)



