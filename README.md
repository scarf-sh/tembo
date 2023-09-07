# Tembo CLI

Tembo CLI allows users to experience [Tembo](https://tembo.io) locally, as well as, 
manage and deploy to Tembo Cloud. It abstracts away complexities of configuring, 
managing, and running Postgres in a local environment. 

# Local Testing

Clone this repo and run:

`cargo install --path .`

If the install path is in your shell path, you can then run `tembo help` and other `tembo` commands.

# Commands

## `tembo init`

The `init` command initializes your environment and can be used to generate configuration files. It will
also alert you to any missing requirements.

The default configuration file path is $HOME/.config/tembo.

For more information: `tembo init --help`

## `tembo stack create`

WIP: being renamed to `instance create`

The `stack create` command installs a local instance of a Tembo locally. 
It includes the Tembo flavored version of Postgres and an additional items like extensions. You can specify the 
type of instance you want to create.

It all runs in a Docker container.

Currently supported types include: 

* standard
* data-warehouse

More stack types will be added shortly.

## `tembo instance start`

Coming soon - will start an installed instance.

# Contributing

Before you start working on something, it's best to check if there is an existing plan 
first. Join our [Slack community](https://join.slack.com/t/trunk-crew/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg) and ask there.

# Semver

Tembo CLI is following [Semantic Versioning 2.0](https://semver.org/).
