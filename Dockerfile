FROM jodogne/orthanc-plugins:1.5.8

WORKDIR /orthanc-gcp-storage

# Install debian packages
RUN apt-get update && apt-get install -y \
  procps \
  build-essential \
  pkg-config \
  libffi-dev \
  libclang-dev \
  clang \
  curl

# Install rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add cargo to PATH
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Copy project into working directory
COPY . .

# Copy orthanc config into orthanc dir
COPY scripts/orthanc.json /etc/orthanc/orthanc.json