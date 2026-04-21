# Timelog

A simple rust cli to track your working time.

## Important

This is a rewrite of DerBaumann/timelog, which is currently deprecated.
I strongly advise anyone interested in this tool to use the rust version, as I am actively working on it.

I might patch up the go version to be functionally on the same level, but I cant give any guarantees!

## Features

Timelog is primarily a small time-tracking tool, that you can use to track:

- Hours spent on a customer project
- How long you spent on chores
- working time, for when your boss forces you to have a work-journal.

You can:

- `list` all entries
- Either `record` a new entry or programatically `add` one
- `edit` and `delete` entries by id
- `export` the entries to a markdown doc (currently a predefined german version. Might change it to a more general template or add the feature to add your own ones)
- Show useful auto-generated help text for any command with the `--help` flag (thx clap)

## Configuration

If you want your store to be somewhere, other that your default config path,
you can manually override the `TIMELOG_STOREPATH` environment variable.

Outside of that there isn't much in terms of configuration that you can do.
