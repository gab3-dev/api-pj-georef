# Install spawnctl
RUN curl https://run.spawn.cc/install | sh && ln -s $HOME/.spawnctl/bin/spawnctl /usr/local/bin

# Install Flyway
RUN wget -qO- https://repo1.maven.org/maven2/org/flywaydb/flyway-commandline/7.0.2/flyway-commandline-7.0.2-linux-x64.tar.gz | \
tar xvz && sudo ln -s `pwd`/flyway-7.0.2/flyway /usr/local/bin
