# Crypto Editor
This editor is used as a tool to write and edit encrypted journal logs stored in
the local hard drive. It's meant to be used in a local computer disconnected from
the internet.

Used this [guide](https://www.philippflenker.com/hecto/) to build a text editor and edited it as required.

# Usage
```sh
# Create  release build
cargo build --release
```

`Ctrl-c` to leave the program at anytime.

- **Create a new user**

![gif](https://gitlab.com/Fernie/screenshots/-/raw/master/crypto_editor/create_user.gif)

- **Add a new log**

![gif](https://gitlab.com/Fernie/screenshots/-/raw/master/crypto_editor/create_log.gif)

- **Edit/Read a previous log**

![gif](https://gitlab.com/Fernie/screenshots/-/raw/master/crypto_editor/edit_log.gif)

# Commands while in the text editor
`Ctrl-s` to save.

`Ctrl-q` to leave the editor.

# License
This project is distributed under [the MIT License](./LICENSE.txt).
