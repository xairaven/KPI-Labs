#######
# Dockerfile to build MongoDB container images
# Based on Debian
#######

# Set the base image to Debian
FROM debian

ENV VOID /dev/null

# Update the repository sources list and install gnupg2, software-properties-common, wget
RUN apt-get update 1> $VOID
 
RUN apt-get install -y gnupg 1> $VOID
RUN apt-get install -y software-properties-common 1> $VOID
RUN apt-get install -y wget 1> $VOID

# Add the package verification key
RUN wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | apt-key add -

# Add MongoDB to the repository sources list
RUN add-apt-repository "deb http://repo.mongodb.org/apt/debian bullseye/mongodb-org/6.0 main"

# Update the repository sources list
RUN apt-get update

# Install MongoDB package (.deb)
RUN apt-get install -y mongodb-org 1> $VOID

# Create the default data directory
RUN mkdir -p /data/db

# Expose the default port
EXPOSE 27017

# Default port to execute the entrypoint
CMD ["--port 27017"]

# Set default container command
ENTRYPOINT usr/bin/mongod
