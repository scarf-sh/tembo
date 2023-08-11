# Tembo CLI

The Tembo CLI allows users to experience [Tembo](https://tembo.io) locally, as well as, 
manage and deploy to the Tembo cloud. It abstracts away complexities of configuring, 
managing, and running Postgres in a local environment. The Tembo CLI aims to provide the 
best experience working with Postgres in any environment.

# Getting Started

This repo is a work in progress. Soon, it will provide install instructions and a 
detailed Getting Started guide.

# Commands

`tembo help`

The help command will respond with the various commands and options available.

`tembo init`

The `init` command initializes your environment and can be used to generate configuration files. 
The command supports a `dryrun` flag to test where a configuration file will be written. It also 
supports a `file-path` flag that can be used to explicitly provide an absolute or relative file 
path for the configuration file.

The default configuration file path is $HOME/.config/tembo.

The `init` command can be used to create global and project specific configuration files.

For more information: `tembo init --help`

# Contributing

Before you start working on something, it's best to check if there is an existing plan 
first. Stop by the Discord server and confirm with the team if it makes sense or if 
someone else is already working on it.

A more detailed Contributing Guide is coming soon.

# Semver

Tembo CLI is following [Semantic Versioning 2.0](https://semver.org/).

