If you came here, looking for a place to leave feedback or post ideas, please check the `Ideas & Feedback` category in GitHubs "Discussion" tab.

# Contributing to development
- Please create a new branch for new changes and create a pull request if you finished your work on a feature.
- A overview of the structure of the data file and how it is handled in rust can be found in [finanzbuch_lib/development.md](finanzbuch_lib/development.md)

## Working in a container
You need `xhost` and `docker` installed on your host.
If your host system is linux with an XServer, you can run `./compose-with-xorg.sh` (your terminals active directory has to be `./docker`) to get setup with everything this project needs. This script changes your XServer rules to allow other software (in this case docker) to connect to it. This is needed so that the tauri window will be displayed in your host.
I use the DevContainer Plugin for VS Code to work directly inside the docker container. Inside the container, the project is at `/root/project`.

To run the created binary on your host, check the generated file with `ldd filename` to see if you have all necessary libraries on your host system installed. Since this is likely not the case, I will look into how I can generate a smaller, dynamically linked binary and a larger statically link binary, that contains all the used library code.

## Working on your machine
If you want to work on your host, just take a look into the [Dockerfile](./Dockerfile) to see what you need to setup.

## How Rust and JS work together
The current way of presenting the data to the user is a somewhat crude form of server-side rendering, but locally. It works like this:
1. The application starts with the navigation bar as static html. The rest of the screenspace is an empty `<div id="content">`
2. A click on each navigation entry triggers a JS function which simply invokes a tauri command to rust
   1. This rust function then builds the corresponding html for the page the user wants to see
   2. This html gets inserted as `#content.innerHTML`

- This means two things:
  1. Since this new block of html is injected after the page is done loading, every JS code that is referenced by that new html block cannot be a module, since their contents are not globally accessable. This also means that all `<script>` tags have to be after the `</body>`.
  2. EventListeners to listen for events in the new html block would have to be created in JS after this html is injected. Since this is a bit error-prone and annoying, it is easier to just use the events that are built into html and reference a globally accessable function there. Like this: `<button onclick="doSomething()"/>`
- Why did I choose this design? A year before I started this project, I tested tauri. This test ended up being very confusing because I had to track every change the user made and still communicate with rust to get new data. To avoid this problem in this project, the frontend will only be used to ...
  1. present data that is calculated / prepared by the rust backend, and 
  2. send any changes or new input back to rust for processing
  - This means that the current state of the user's data is only held by rust, where all the calculations and security checks are already implemented
  - It is also much easier to build whole tables full of data this way, because you don't have to send that data to JS in obscure formats to then insert it

## Building a Flatpak 
I am still figuring this out, this is just what I found so far.
- Tauri generates a .deb package that contains a `control.tar.gz` a `data.tar.gz` and a `debian-binary`
  - `control` describes how a debian system has to install the program
  - `data` contains the program that has to be installed
  - Extract this with `ar -x package.deb`
    - unpack `control` with `tar -xzvf control.tar.gz`
    - unpack `data` with `tar -xzvf data.tar.gz`
- Create a manifest, here called `org.flatpak.Hello.yml` with ID `org.flatpak.Hello`
- Initialize the build directory
  - *Not always needed?? Just skip for now*
  - `flatpak build-init <dir> <id> <runtime> <sdk>` = `flatpak build-init build-dir org.flatpak.Hello org.freedesktop.Platform//23.08 org.freedesktop.Sdk//23.08`

---

- Running the app inside the docker container will likely fail, because the flatpak needs dbus access in some cases, which docker does not allow (even in priviledged containers)
  - To still test this app, build it and add it to a local repo:
    - **Run in container**
    - `flatpak-builder build-dir org.robertosw.finanzbuch.yml`
    - `flatpak-builder --repo=repo --force-clean build-dir org.robertosw.finanzbuch.yml`
  - Add this repo on the host and install from this repo. The host has to have the same runtime installed
    - **Run on host**
    - `flatpak -y --user remote-add --no-gpg-verify repolocal repo`
    - `flatpak -y --user install repolocal org.robertosw.finanzbuch`
    - `flatpak run org.robertosw.finanzbuch`
    - If its already installed, just run `flatpak -y --user update org.robertosw.finanzbuch`
- Stuff I found out, that confused me
  - `/app` is the representation of `/` in the filesystem, but the `/app` folder doesnt exist. Thats why `install -D hello.sh /app/bin/hello.sh` is a valid path. This does not need to point to a folder called `/app` but instead defines where this will be installed at
    - The `hello.sh` probably has to be in the directory in which the build command is called, or in which the .yml file lies
