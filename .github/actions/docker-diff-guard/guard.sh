#!/usr/bin/env sh
set -eu

work="$(mktemp -d)"
container=""

cleanup() {
  if [ -n "${container}" ]; then
    docker rm --force "${container}" > /dev/null || true
  fi
  rm -rf "${work}"
}
trap cleanup EXIT

# Closing stdin turns a command that waits for input into a failure rather than a
# job that hangs until the runner times out.
#
# The workspace is mounted read-only because a bind mount is invisible to
# `docker diff`: anything written there would go unmeasured, and a mount the
# command can write to is a blind spot rather than a convenience.
status=0
docker run \
  --cidfile "${work}/cid" \
  --volume "${GITHUB_WORKSPACE}:${INPUT_WORKDIR}:ro" \
  --workdir "${INPUT_WORKDIR}" \
  "${INPUT_IMAGE}" \
  sh -c "${INPUT_RUN}" \
  < /dev/null || status=$?

# `docker diff` reads the container's writable layer, which is discarded together
# with the container, so the container has to outlive the command it ran.
if [ -f "${work}/cid" ]; then
  container="$(cat "${work}/cid")"
fi

if [ "${status}" -ne 0 ]; then
  echo "::error::the command exited with status ${status}"
  exit "${status}"
fi

# Docker creates the mount point when the image has no directory there, which is
# this action's own doing rather than the command's. The asterisk stands for an
# entry that allows every kind of change.
printf '%s %s\n' '*' "${INPUT_WORKDIR}" > "${work}/allowed"

printf '%s\n' "${INPUT_ALLOWED}" > "${work}/input"
while IFS= read -r entry; do
  entry="$(printf '%s' "${entry}" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
  case "${entry}" in
    "" | \#*) continue ;;
  esac

  kind='*'
  path="${entry}"
  case "${entry}" in
    [ACD][[:space:]]*)
      kind="$(printf '%s' "${entry}" | cut -c 1)"
      path="$(printf '%s' "${entry}" | sed 's/^[ACD][[:space:]]*//')"
      ;;
  esac

  case "${path}" in
    /*) ;;
    *)
      echo "::error::allowed entry '${entry}' is not an absolute path"
      exit 1
      ;;
  esac

  printf '%s %s\n' "${kind}" "${path}" >> "${work}/allowed"
done < "${work}/input"

docker diff "${container}" | sort > "${work}/changes"

covered_by_the_allowed_list() {
  change_kind="$1"
  change_path="$2"

  while read -r allowed_kind allowed_path; do
    if [ "${allowed_kind}" != '*' ] && [ "${allowed_kind}" != "${change_kind}" ]; then
      continue
    fi
    # The pattern is deliberately unquoted: an allowed entry is a glob.
    case "${change_path}" in
      ${allowed_path}) return 0 ;;
    esac
  done < "${work}/allowed"

  return 1
}

# A directory is reported as changed merely because something inside it was
# created or deleted, and as created when it had to exist before something
# inside it could. Such an entry follows from the change below it rather than
# being a change of its own, so the allowed list only has to name the latter.
# Nothing hides behind this: the change below is reported on a line of its own
# and is checked there.
follows_from_a_change_below() {
  directory="$1"

  while read -r below; do
    below="${below#* }"
    case "${below}" in
      "${directory}") continue ;;
      "${directory%/}"/*) return 0 ;;
    esac
  done < "${work}/changes"

  return 1
}

echo "docker diff reported $(grep -c '' "${work}/changes") change(s)"
cat "${work}/changes"

: > "${work}/violations"
while IFS= read -r change; do
  kind="${change%% *}"
  path="${change#* }"

  if covered_by_the_allowed_list "${kind}" "${path}"; then
    continue
  fi

  if [ "${kind}" != D ] && follows_from_a_change_below "${path}"; then
    continue
  fi

  printf '%s\n' "${change}" >> "${work}/violations"
done < "${work}/changes"

if [ -s "${work}/violations" ]; then
  while IFS= read -r violation; do
    case "${violation%% *}" in
      A) verb="created" ;;
      C) verb="changed" ;;
      D) verb="deleted" ;;
      *) verb="touched" ;;
    esac
    echo "::error::the command ${verb} ${violation#* }, which the allowed list does not cover"
  done < "${work}/violations"
  exit 1
fi

echo "nothing outside the allowed list was created, changed or deleted"
