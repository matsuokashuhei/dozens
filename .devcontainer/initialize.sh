#!/bin/bash
set -euo pipefail
umask 077

aws() {
  docker run --rm -t -v "$HOME/.aws:/root/.aws" amazon/aws-cli "$@"
}

if ! aws configure export-credentials --profile dozens --format env-no-export > .env.aws 2>/dev/null; then
  open 'https://d-9567554c74.awsapps.com/start/#/device'

  exec 3< <(
    aws sso login --profile dozens --use-device-code 2>&1 |
      awk '
        /Then enter the code:/ { copy_next = 1; print > "/dev/stderr"; fflush("/dev/stderr"); next }
        copy_next {
          code = $0
          gsub(/\r/, "", code)
          sub(/^[[:space:]]+/, "", code)
          sub(/[[:space:]]+$/, "", code)
          if (code == "") { print > "/dev/stderr"; fflush("/dev/stderr"); next }
          printf "%s", code | "pbcopy"
          close("pbcopy")
          print "Verification code copied to clipboard." > "/dev/stderr"
          print code
          fflush()
          copy_next = 0
          next
        }
        { print > "/dev/stderr"; fflush("/dev/stderr") }
      '
  )
  login_pipeline_pid=$!

  if IFS= read -r verification_code <&3; then
    printf '%s\n' "$verification_code"
    if ! osascript .devcontainer/paste-device-code.applescript >/dev/null 2>&1; then
      printf '%s\n' 'Could not paste automatically; paste the code manually or grant Accessibility access.' >&2
    fi
  fi

  cat <&3 >/dev/null
  exec 3<&-
  wait "$login_pipeline_pid"

  aws configure export-credentials --profile dozens --format env-no-export > .env.aws
fi

chmod 600 .env.aws
