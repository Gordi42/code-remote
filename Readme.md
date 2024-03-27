# Code-Remote
A terminal user interface to automate the process of allocating a slurm-session on a remote machine and connecting VSCode to it.

**Supported Platforms:** Linux, MacOS

# Content:
- [Code-Remote](#code-remote)
- [Content:](#content)
- [Installation](#installation)
  - [Prerequisites: Local Machine](#prerequisites-local-machine)
  - [Prerequisites: Remote machine](#prerequisites-remote-machine)
  - [Installation: Build from source](#installation-build-from-source)
- [Usage](#usage)
  - [Setting up the remote host](#setting-up-the-remote-host)
  - [Selecting a host](#selecting-a-host)
  - [Removing a host](#removing-a-host)
  - [Setting up a session](#setting-up-a-session)
  - [Spawn a session](#spawn-a-session)
  - [Editing the configuration files manually](#editing-the-configuration-files-manually)
- [Author](#author)


# Installation
## Prerequisites: Local Machine
- VSCode with the ssh-remote extension: `ms-vscode-remote.remote-ssh`
- VSCode can be opened from the terminal with the command `code`. On linux, 
this should be the case by default. On MacOS, you have to launch VSCode, 
open the command palette with `Cmd+Shift+P` and search for:
```
Shell Command: Install 'code' command in PATH
```
- 'ssh' and 'ssh-keygen' installed, and a config file under `~/.ssh/config`. You can create the config file by executing 
```bash
touch ~/.ssh/config
```
in the terminal.
- `openssl` installed. Openssl can be installed on debian based systems (Debian, Ubuntu, Linux Mint)
```bash
sudo apt-get install libssl-dev
```
On macos, it can be installed using homebrew:
```bash
brew install openssl
```
## Prerequisites: Remote machine
- Slurm installed: Check with `scontrol show partition`, if it returns a list of partitions, you are good to go.
## Installation: Build from source
To build the binary from source, you must have rust and cargo installed.
1. Check if rust and cargo is installed, to do this, type `cargo --version`, if it returns a version, cargo is already installed, if it returns an error, you need to install cargo and rust with
```bash
curl https://sh.rustup.rs -sSf | sh
```
2. Clone the repository with 
```bash
git clone https://github.com/Gordi42/code-remote.git
```
3. Change into the directory with 
```bash
cd code-remote
```
4. Build the binary with 
```bash
cargo build --release
```
5. Move the binary to a directory that is in your PATH. The binary will be in the `target/release` directory.

# Usage
## Setting up the remote host
1. Select the 'Create New' option and press enter. You will be asked to enter a name for the remote host. 
This name is only for your convenience and can be anything you like. Press enter after you have entered the name.
2. You can navigate through the entries with the arrow keys. Select the 'Host' entry and press enter. 
You will be asked to enter the hostname of the remote machine. This is the name that you use to connect to the remote machine with 'ssh user@host'.
3. Select the 'User' entry and press enter. You will be asked to enter the username that you use to connect to the remote machine with 'ssh user@host'.
4. If you have a private key that you use to connect to the remote machine, select the 'IdentityFile' entry and press enter. Enter the absolute path to the private key file. If you do not have a private key, you can leave this entry empty, and you will be asked for the password when connecting to the remote machine.
5. Press 'tab' to switch the focus back to the Cluster list. You can later change the entries by selecting the host and pressing 'tab' to focus on the entry menu.
## Selecting a host
1. You can navigate through the host list with the arrow keys. Select the host that you want to connect to and press enter.
2. You will be asked to enter a password if you did not provide a private key.
3. After you have entered the password, the program tries to establish a connection to the remote machine. If the connection is successful, you will be directed to the spawner menu. Otherwise, an error message will be displayed.
## Removing a host
Select the host that you want to remove and press 'd'. You will be asked to confirm the deletion. If you confirm, the host will be removed from the list.
## Setting up a session
1. Select the 'Create New' option and press enter. You will be asked to enter a name for the session.
2. Specify the account that you want to use (where the resources are billed to).
3. Specify the partition that you want to use. You can check the available partitions with `scontrol show partition` on your remote machine.
4. Specify the maximum time that the session is allowed to run. The format is `hours:minutes:seconds`.
5. Specify the working directory. This is the directory that you want to open in VSCode. The default is the home directory of the user on the remote machine.
6. Specify other options if you want to. These are appended to the salloc command. For example, you can specify the memory that you want to use with `--mem=8G` (for 8 gigabytes of memory).
7. Press 'tab' to switch the focus back to the session list. You can later change the entries by selecting the session and pressing 'tab' to focus on the entry menu.
## Spawn a session
Similar to selecting a host: Navigate through the session list with the arrow keys and select the session that you want to spawn. Press enter to spawn the session. If the session is successfully spawned, you will be directed to the VSCode menu. Otherwise, an error message will be displayed.
## Editing the configuration files manually
The configuration files are located in `~/.config/code-remote`. You can edit the files with a text editor. The `clusters.toml` file contains the remote hosts, and the `($Hostname).toml` file contains the information about the corresponding sessions.

# Author
Silvano Rosenau
