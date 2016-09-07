# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|

  # Use Ubuntu 64-bit.
  config.vm.box = "bento/ubuntu-16.04"

  # Run the provisioning script.
  config.vm.provision :shell, path: "provision.sh"

  # Fix for VirtualBox and Ubuntu losing internet connection.
  config.vm.provider "virtualbox" do |v|
    v.customize ["modifyvm", :id, "--natdnshostresolver1", "on"]
    v.customize ["modifyvm", :id, "--natdnsproxy1", "on"]
  end

  # Enable GUI mode
  # v.gui = true

  # Enable ssh forwarding to X11
  config.ssh.forward_x11 = true
end