# k2so ![](https://img.shields.io/crates/v/k2so.svg)
Deploys your software -- The captain said I have to

The purpose of this crate is to assist with deployments and infrastructure changes performed via Chef.

When using [knife solo](https://matschaffer.github.io/knife-solo/) it requires some arguments passed in such as IP address and the role name. To ensure that the right machine gets provisioned with the right cookbook this tool lets the user define roles upfront which then can be used afterwards to perform deployments and/or infrastructure changes in an easy way.

## Installation

To install k2so, you first need to install Rust from https://www.rust-lang.org/en-US/install.html.

Then you can run `cargo install k2so`.

## Usage

This requires the following tools to be installed:

- [ChefDK](https://downloads.chef.io/chefdk)
- [knife solo](https://matschaffer.github.io/knife-solo/)

First, a new mapping between role and IP address has to be defined:

```bash
$ k2so add 192.168.33.10 app
```

This maps the address `192.168.33.10` to the `app` role. If that role already existed before, it gets overwritten automatically.

Then execute these commands as well:

```bash
$ k2so add_user root
$ k2so add_key keys/id_rsa
```

These two configure the user which shall be used to connect and the ssh key. These are global for all configured roles.

Then if that's done, the actual deploy can be performed:

```bash
$ k2so deploy app
```

which then runs the `app` cookbooks on the configured `app` machine.
