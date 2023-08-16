#!/usr/bin/env bash
set -x
set -eo pipefail

RUNNING_CONTAINER=$(docker ps --filter "name=redis" --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
	echo >&2 "Redis already running"
	exit 1
fi

docker run \
	-p "6379:6379" \
	-d \
	--name "redis_$(date '+%s')" \
	redis:7

echo >&2 "Redis Running"
