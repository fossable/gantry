<p align="center">
	<img src="https://raw.githubusercontent.com/fossable/gantry/master/.github/images/logo-bg-256.png" />
</p>

![License](https://img.shields.io/github/license/fossable/gantry)
![Build](https://github.com/fossable/gantry/actions/workflows/test.yml/badge.svg)
![GitHub repo size](https://img.shields.io/github/repo-size/fossable/gantry)
![Stars](https://img.shields.io/github/stars/fossable/gantry?style=social)
<hr>

`gantry` is a delivery mechanism for _Software as a Service_. You just deposit cryptocurrency
into the `gantry` and a private instance is created for you.

##### Goals
- Enable SaaS that lasts
- Require no personal information to sign up

## Using the SaaS vending machine

To provision a new instance through `gantry`, you first generate a new account ID
from the service's gantry dashboard. This can also be done from the API:

```
curl -X POST gantry.example.com/xmr/provision
```

The account ID is actually a crypto wallet address that's permanently linked to your
instance. All you have to do is send funds to your account ID which will automatically
extend your instance's lease.

You can send any amount to your account ID and the added time scales according to
the service's configuration. You can request the current rates from the API:

```
curl -X POST gantry.example.com/xmr/rates
```

Once you've added some time to your instance's lease, it can be accessed from `<account id>.example.com`.

## Running up a `gantry` for your own SaaS

TODO
