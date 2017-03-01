# Create image based on the official Node 6 image from dockerhub.
FROM azasypkin/rust-stable:latest

# backtrace-sys crate requires these dependencies.
RUN pacman -S --noconfirm gawk grep sed

# Create a directory where the app will be placed
RUN mkdir -p /usr/src/api

# Change directory so that our commands run inside this new directory.
WORKDIR /usr/src/api

# Get all the code needed to run the app.
COPY . /usr/src/api

# Install dependecies.
RUN cargo build

# Expose the port the app runs in.
EXPOSE 8009

# Serve the api.
CMD ["cargo", "run", "--", "--db-ip", "db"]