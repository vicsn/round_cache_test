# Mapping versus Vector speed test

## Results

Only 1 validator inserts rounds
```
Updating v1 100000 times took 42 milliseconds, last round: 0
Updating v2 100000 times took 317881 milliseconds, last round: 0
Updating v3 100000 times took 73 milliseconds, last round: 0
```

Every validator inserts rounds in a round-robin fashion, increasing every time: after <quorum> insertions, each insertion should return (latest_round - <quorum>).
```
Updating v1 100000 times took 2633 milliseconds, last round: 99866
Updating v2 100000 times took 2283 milliseconds, last round: 99866
Updating v3 100000 times took 473 milliseconds, last round: 99866
``` 
