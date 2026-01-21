#!/usr/bin/env bash
set -euo pipefail

output_dir="docs/screenshots"
sleep_seconds="3"
crop_pixels="0"
fullscreen="false"
working_dir="$(pwd)"
single_demo=""
capture_ok="false"

while getopts "o:w:p:fd:" opt; do
  case "${opt}" in
    o) output_dir="${OPTARG}" ;;
    w) sleep_seconds="${OPTARG}" ;;
    p) crop_pixels="${OPTARG}" ;;
    f) fullscreen="true" ;;
    d) single_demo="${OPTARG}" ;;
  esac
done

cleanup() {
  if [[ "${capture_ok}" != "true" ]]; then
    return
  fi
  osascript <<OSA >/dev/null 2>&1 || true
tell application "System Events"
  if exists process "Ghostty" then
    tell process "Ghostty"
      try
        set front_window to first window whose value of attribute "AXMain" is true
        click button 1 of front_window
      end try
    end tell
  end if
end tell
OSA
}
trap cleanup EXIT

mkdir -p "${output_dir}"

if [[ -n "${single_demo}" ]]; then
  demos=("${single_demo}")
else
  mapfile -t demos < <(just --list --unsorted | awk '{print $1}' | sed -n 's/^\(demo-[^ ]*\)$/\1/p')
fi

if [[ "${#demos[@]}" -eq 0 ]]; then
  printf "No demo commands found.\n" >&2
  exit 1
fi

open -a Ghostty
sleep 0.5

osascript <<OSA
tell application "Ghostty" to activate
tell application "System Events"
  keystroke "n" using {command down}
  delay 0.2
  if "${fullscreen}" is "true" then
    delay 0.2
    keystroke "f" using {control down, command down}
  end if
end tell
OSA

for demo in "${demos[@]}"; do
  capture_ok="false"
  output_file="${output_dir}/${demo}.png"

  osascript <<OSA
tell application "Ghostty" to activate
tell application "System Events"
  keystroke "c" using {control down}
  delay 0.1
  keystroke "bash -lc \"cd ${working_dir} && just ${demo}\""
  key code 36
end tell
OSA

  sleep "${sleep_seconds}"

  window_bounds=$(osascript <<'OSA'
tell application "System Events"
  tell process "Ghostty"
    set front_window to first window whose value of attribute "AXMain" is true
    set window_position to position of front_window
    set window_size to size of front_window
    set x to item 1 of window_position as integer
    set y to item 2 of window_position as integer
    set w to item 1 of window_size as integer
    set h to item 2 of window_size as integer
    return (x as text) & "," & (y as text) & "," & (w as text) & "," & (h as text)
  end tell
end tell
OSA
  )

  window_bounds=${window_bounds//, /,}
  window_bounds=${window_bounds// /}

  if [[ -z "${window_bounds}" ]]; then
    printf "Failed to find Ghostty window bounds for %s.\n" "${demo}" >&2
    exit 1
  fi

  screencapture -x -R "${window_bounds}" "${output_file}"
  if [[ ! -f "${output_file}" ]]; then
    printf "Failed to capture screenshot for %s.\n" "${demo}" >&2
    exit 1
  fi
  capture_ok="true"

  if [[ "${crop_pixels}" != "0" ]]; then
    if ! command -v magick >/dev/null 2>&1 && ! command -v convert >/dev/null 2>&1; then
      printf "Missing dependency: ImageMagick (install with 'brew install imagemagick').\n" >&2
      exit 1
    fi
    crop_geometry="${crop_pixels}x${crop_pixels}"
    if command -v magick >/dev/null 2>&1; then
      magick "${output_file}" -shave "${crop_geometry}" "${output_file}"
    else
      convert "${output_file}" -shave "${crop_geometry}" "${output_file}"
    fi
  fi

  printf "Captured screenshot to %s\n" "${output_file}"

  osascript <<OSA
tell application "Ghostty" to activate
tell application "System Events"
  keystroke "q"
end tell
OSA
done

capture_ok="true"
