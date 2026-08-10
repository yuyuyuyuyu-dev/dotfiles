# Docker diff guard

Runs a command in a container and fails when it creates, changes or deletes
anything the allowed list does not cover.

The command runs in a container that is kept alive after it exits, and
`docker diff` then reports every path in the container's filesystem that differs
from the image it started from. Every reported path has to be covered by the
allowed list; anything else fails the step.

The checkout is copied into a layer on top of `image` rather than mounted into
the container, so that a command which writes into its own project directory --
installing dependencies there, building into it -- can do so at all. A read-only
mount would stop it, and a writable one is invisible to `docker diff` anyway.

Anything the command needs but is not being measured on belongs in `setup`,
which runs while that layer is built and so lands in the image.

```yaml
- uses: ./.github/actions/docker-diff-guard
  with:
    setup: apt-get update && apt-get install --yes curl
    run: ./install.sh
    allowed: |
      A /usr/local/bin/mytool
      A /root/.cache/*
```

## Inputs

| Input | Required | Description |
| --- | --- | --- |
| `image` | no | Image the command runs in. Defaults to the one pinned in `base-image.Dockerfile`. |
| `setup` | no | Commands that prepare the environment, passed to `sh -e -c` while the image is built, with the checkout already in place. What they change is part of the image and so is not measured. |
| `run` | yes | Command whose file changes are measured, passed to `sh -e -c`. |
| `allowed` | yes | One entry per line, see below. |
| `workdir` | no | Where the checkout is placed, and the working directory of the command. Defaults to `/github/workspace`. |

An allowed entry is an absolute path, optionally preceded by `A`, `C` or `D` to
allow only that kind of change: added, changed or deleted. Without one, any kind
of change to that path is allowed. The path is a glob whose `*` also matches `/`,
so `/root/.cache/*` covers the whole subtree. Blank lines and lines starting with
`#` are ignored.

## What the allowed list does not have to mention

A directory is reported as changed whenever something inside it is created or
deleted, and as created when it had to exist before something inside it could.
Such an entry follows from the change below it, so only the change below it needs
an entry. Nothing is hidden this way: that change is reported on a line of its
own and is checked there. A directory with no reported change beneath it is a
change in its own right and does need an entry.

A deletion is never implied. Removing a whole directory is reported as one
deleted path, the directory itself, and allowing what was inside it does not
allow it: `D /foo/bar` covers a deleted file, and says nothing about `/foo`
going with it.

Whatever the container runtime writes on its own is not the command's doing
either. A container that ran nothing is diffed first, and its report is
subtracted from the command's, so a runtime or storage driver that leaves marks
of its own does not put them in every caller's allowed list.

The working directory is allowed whole. It holds a checkout of a repository with
a remote, where damage is already answered by `git status` -- which, unlike
`docker diff`, can tell a tracked file from a build artifact. What has no other
witness is what the command did to the machine around it, which is what this
check is for. Listing build outputs instead would go stale with every dependency
added and catch nothing that git does not already show.

## Limits worth knowing

- **Linux runners only.** The step needs a Docker daemon, which the macOS and
  Windows runners do not have.
- **What the command installs outside the project is a change like any other.** A
  package manager reaching into `/usr` produces the same kind of report as
  anything else, so system dependencies belong in `image` or `setup`.
- **Every step builds a layer.** Copying the checkout in costs a `docker build`
  over the whole build context, so a `.dockerignore` is worth having on a
  repository that carries large directories a build does not need.
- **Changes to a directory's own metadata can be masked.** `docker diff` reports
  a `chmod` on a directory the same way it reports the directory holding a
  changed file, so a permission change on a directory that also holds an allowed
  change is not distinguishable from it.
