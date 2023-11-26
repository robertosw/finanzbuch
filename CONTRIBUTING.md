## Contribute

### User Feedback
If you have used this software and have a suggestion for something that could be done better, or ideas for more features, it would be great if you could start a discussion in the `Ideas & Feedback` category.

### Development

- Please create a new branch for new changes and create a pull request if you finished your work on a feature.
- A overview of the structure of the data file and how it is handled in rust can be found in [finanzbuch_lib/development.md](finanzbuch_lib/development.md)

#### Working in a container
You need `xhost` and `docker` installed on your host.
If your host system is linux with an XServer, you can run `./compose-with-xorg.sh` in the root of the project to get setup with everything this project needs. This script changes your XServer rules to allow other software (in this case docker) to connect to it. This is needed so that the tauri window will be displayed in your host.
I use the DevContainer Plugin for VS Codes to work directly inside the docker container. Inside the container, the project is at `/root/project`.

To run the created binary on your host, check the generated file with `ldd filename` to see if you have all necessary libraries on your host system installed.

#### Working on your machine
If you want to work on your host, just take a look into the [Dockerfile](./Dockerfile) to see what you need to setup.

