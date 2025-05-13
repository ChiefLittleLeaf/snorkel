# snorkel
This repo is for a collection of algorithms and other maintenance/admin functionality inside a rust developed cli tool

# Example usage
## Basic ema example with raw values on a Linux machine
```bash
while true; do
  usage=$(grep 'cpu ' /proc/stat | awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage}')
  echo "$usage"
  sleep 1
done | cargo run -- ema
```
## Basic ema example with timestamps on a Linux machine
```bash
while true; do
  ts=$(date -Iseconds)
  usage=$(grep 'cpu ' /proc/stat | awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage}')
  echo "$ts,$usage"
  sleep 1
done | cargo run -- ema --timestamp
```

## Basic zscore example with raw values on a Linux machine
```bash
while true; do
  usage=$(grep 'cpu ' /proc/stat | awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage}')
  echo "$usage"
  sleep 1
done | cargo run -- zscore
```

## Basic zscore example with timestamps on a Linux machine
```bash
while true; do
  ts=$(date -Iseconds)
  usage=$(grep 'cpu ' /proc/stat | awk '{usage=($2+$4)*100/($2+$4+$5)} END {print usage}')
  echo "$ts,$usage"
  sleep 1
done | cargo run -- zscore --timestamp
```
