# k2so
Deploys your software -- The captain said I have to

The purpose of this crate is to assist with deployments and infrastructure changes performed via Chef.

When using [knife solo](https://matschaffer.github.io/knife-solo/) it requires some arguments passed in such as IP address and the role name. To ensure that the right machine gets provisioned with the right cookbook this tool lets the user define roles upfront which then can be used afterwards to perform deployments and/or infrastructure changes in an easy way.

## Usage

First, a new mapping between role and IP address has to be defined:

```bash
$ k2so -add app 192.168.33.10
```

This maps the address `192.168.33.10` to the `app` role. If that role already existed before, it gets overwritten automatically.

Then if that's done, the actual deploy can be performed:

```
$ k2so deploy app
```

which then runs the `app` cookbooks on the configured `app` machine.
