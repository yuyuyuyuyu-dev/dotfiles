# Docker diff guard

Runs a command in a container and fails when it creates, changes or deletes
anything the allowed list does not cover.

The command runs in a container that is kept alive after it exits, and
`docker diff` then reports every path in the container's filesystem that differs
from the image it started from. Every reported path has to be covered by the
allowed list; anything else fails the step.

```yaml
- uses: ./.github/actions/docker-diff-guard
  with:
    image: my-image
    run: ./install.sh
    allowed: |
      A /usr/local/bin/mytool
      A /root/.cache/*
```

## Inputs

| Input | Required | Description |
| --- | --- | --- |
| `image` | yes | Image the command runs in. |
| `run` | yes | Command to run inside the container, passed to `sh -c`. |
| `allowed` | yes | One entry per line, see below. |
| `workdir` | no | Where the checkout is mounted, and the working directory of the command. Defaults to `/github/workspace`. |

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

## Limits worth knowing

- **Linux runners only.** The step needs a Docker daemon, which the macOS and
  Windows runners do not have.
- **The image has to be complete.** Whatever the command installs after the
  container starts is itself a change, and will be reported as one. Install the
  command's dependencies in the image instead.
- **The checkout is mounted read-only.** A bind mount is invisible to
  `docker diff`, so anything written there would go unmeasured. Keeping it
  read-only turns that blind spot into an error the command reports itself.
- **Changes to a directory's own metadata can be masked.** `docker diff` reports
  a `chmod` on a directory the same way it reports the directory holding a
  changed file, so a permission change on a directory that also holds an allowed
  change is not distinguishable from it.
