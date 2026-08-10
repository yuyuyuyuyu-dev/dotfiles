# Nothing builds this file. It exists so that Dependabot, which reads image
# references out of Dockerfiles but not out of workflow inputs, can keep the
# default base image current; the action reads the `FROM` line below whenever the
# caller passes no `image`.
#
# Ubuntu because every GitHub hosted Linux runner is Ubuntu, so a command that
# breaks here breaks in the caller's other jobs too.
FROM ubuntu:24.04
