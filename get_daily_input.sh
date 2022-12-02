#!/usr/bin/env bash

set -e

usage() {
  echo """
Fetch the input of a certain day of aoc2022

Usage: get_daily_input.sh <day_number>

Optional arguments:
  -h, --help   Display this message and exit.
"""
  exit $1
}


POSITIONAL=()
while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    -h|--help) usage 0; shift ;;
    -*) usage 1; shift ;;
    *)  POSITIONAL+=("$1"); shift ;;
  esac
done

set -- "${POSITIONAL[@]}" # restore positional parameters

if [ -z $1 ]; then
  usage 1
fi

day_nr=$1
curl -H "Cookie: session=$(cat .session_cookie)" -o "inputs/day${day_nr}.txt" "https://adventofcode.com/2022/day/${day_nr}/input"
