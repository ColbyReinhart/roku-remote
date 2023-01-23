# ROKU REMOTE

Roku remote is a home server which allows you to control Roku TVs with your mobile device
without having to install the remote app. Roku TVs use a simple HTTP API to allow themselves
to be controlled, but CORS prevents this from happening using a mobile browser directly.

---

## Setup

On any device connected to the same LAN subnet as the device you wish to control, clone
this repository. In src/main.rs there is a static constant named SUBNET_SEARCH_LIMIT.
This defines the highest subnet IPV4 address the server should consider. If you're unsure
what to put, somewhere between 15 and 25 is sufficient for most home networks. Compile
using rust cargo as so:

`cargo build --release`

The executable will go to PROJECT_ROOT/target/release.

IT IS EXTREMELY RECOMMENDED TO RUN THIS WITH LINUX! Setting up servers on windows is a huge
pain and is ususally not worth your time. The rest of this tutorial assumes you're using
linux. If you really want to use windows, contact me and I can help you.

Next you'll just need to set up a systemd service to manage the program. If you're new to
systemd, I recommend this video: https://youtu.be/fYQBvjYQ63U

## Usage

Once the server starts, it'll scan the network for Roku devices. Please allow between 30 seconds and 5 minutes, depending on what you set the subnet search limit to. This only happens when the
server first starts, so it shouldn't be a nuisance. Furthermore, if you add a new Roku device to
your network or change your network in any way, you'll need to restart the server.

All you need to do is connect to the server's ip or hostname at port 5000. This will serve you
the remote and you should be able to use it right away as expected. The dropdown menu at the top
will list all found Roku devices. If this list is not poulated as expected, make sure that your
server is on the same subnet as your Roku devices. You may also need to adjust the
SUBNET_SEARCH_LIMIT constant and recompile.