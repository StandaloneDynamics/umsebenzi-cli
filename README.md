# Umsebenzi Cli

This command line tool is a companion to the [django-umsebenzi package]()
This tool allows you to interact with the API to manage your tasks


## Commands

### Projects

* Show projects `umsebenzi project list`
* Add new project `umsebenzi project add`
* Detail project `umsebenzi project detail <project id>`
* Edit project `umsebenzi project edit <project id>`
* Delete project `umsebenzi project delete <project id>`


### Tasks

* Show tasks `umsebenzi task list`
* Add task `umsebenzi task add`
* Detail task `umsebenzi task detail <task code>`
* Edit task `umsebenzi task edit <task code>`
* Delete task `umsebenzi task delete <task code>`
* Update task status `umsebenzi task status <task code> <task status>`

## Config

When adding new configs, this creates a folder called umsebenzi in your `$XDG_CONFIG_HOME` directory


* Show config `umsebenzi config`
* Add new config `umsebenzi config add`
* Edit auth `umsebenzi config edit`
* Show task statuses `umsebenzi config task-status`