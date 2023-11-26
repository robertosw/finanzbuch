If you came here, looking for a place to leave feedback or post ideas, please check the `Ideas & Feedback` category in GitHubs "Discussion" tab.

## Contributing to development
- Please create a new branch for new changes and create a pull request if you finished your work on a feature.
- A overview of the structure of the data file and how it is handled in rust can be found in [finanzbuch_lib/development.md](finanzbuch_lib/development.md)

### Working in a container
You need `xhost` and `docker` installed on your host.
If your host system is linux with an XServer, you can run `./compose-with-xorg.sh` in the root of the project to get setup with everything this project needs. This script changes your XServer rules to allow other software (in this case docker) to connect to it. This is needed so that the tauri window will be displayed in your host.
I use the DevContainer Plugin for VS Code to work directly inside the docker container. Inside the container, the project is at `/root/project`.

To run the created binary on your host, check the generated file with `ldd filename` to see if you have all necessary libraries on your host system installed. Since this is likely not the case, I will look into how I can generate a smaller, dynamically linked binary and a larger statically link binary.

### Working on your machine
If you want to work on your host, just take a look into the [Dockerfile](./Dockerfile) to see what you need to setup.

### How Rust and JS work together
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
