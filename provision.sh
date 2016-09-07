#!/bin/bash

# This is the provisioning script which is executed when the virtual machine is first created.
# Here is where we install all of the dependencies for our project.


apt-get update

# Some other tools we might want to use in the future.
# apt-get -y install doxygen
# apt-get -y install pandoc
apt-get -y install gnuplot-x11

# Developer tools.
apt-get -y install curl
apt-get -y install git

# ARM tools
# apt-get -y install gcc-arm-none-eabi gdb-arm-none-eabi openocd qemu-system-arm

# Install rustup (https://github.com/rust-lang-nursery/rustup.rs)
sudo -u vagrant HOME=/home/vagrant bash -c "curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly"

# Grab xargo for cross-compiling rust
# cargo install xargo

# Configure the machine to switch us to the shared folder immediately upon login.
# Start from default .bashrc so that multiple re-provisionings don't cause duplicate cd statements
cp /etc/skel/.bashrc /home/vagrant/
printf 'cd /vagrant\n' >> /home/vagrant/.bashrc

# Add rustup to path
printf 'export PATH=$PATH:/home/vagrant/.cargo/bin\n' >> /home/vagrant/.bashrc
