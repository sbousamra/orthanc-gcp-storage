## orthanc-gcp-storage
A plugin for orthanc providing an interface with google cloud storage

## development

It is recommended to run the orthanc server with docker-compose for ease of use and portability across systems. Simply type `docker-compose up`.

The plugin is written in rust and must be compiled for the desired OS. To use the plugin with the orthanc server, exec into the container and compile from within the container to have a compatible plugin.

1. Exec into container `docker exec -it orthanc /bin/bash`
2. Build the plugin by running `cargo build`. This should create a file called `liborthanc_gcp_storage.so` in the `target/debug` rust compilation output directory. This is the shared object that orthanc will use as the plugin. 

Copy the plugin to the orthanc plugins folder and restart the server to run the new plugin.
