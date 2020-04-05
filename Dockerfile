FROM jodogne/orthanc-plugins:1.5.8

WORKDIR /orthanc-gcp-storage

# Install basic debian packages
RUN apt-get update && apt-get install -y \
  build-essential \
  curl

# Install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add cargo to PATH
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Copy project into working directory
COPY . .