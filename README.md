# Rust Programming Test

## Running

```sh
$ cargo run -- --filename TestInput.json
```

## Correct output:

```
Input data:
Object name: A Rect: Rect { x: 336.32, y: 117.207, width: 918.7, height: 158.55 } Area: 145659.89
        Intersection: Object name: B Rect: Rect { x: 336.32, y: 117.207, width: 259.2, height: 158.55002 } Area: 41096.168
Object name: B Rect: Rect { x: 72.32, y: -250.207, width: 523.2, height: 1780.55 } Area: 931583.8
        Intersection: Object name: A Rect: Rect { x: 336.32, y: 117.207, width: 259.2, height: 158.55002 } Area: 41096.168
Object name: C Rect: Rect { x: 672.32, y: 300.207, width: 25.55, height: 76.21 } Area: 1947.1654
```

## Error handling:

Invalid input:

```
error: The following required arguments were not provided:
    --filename <FILENAME>

USAGE:
    sowork-test-task --filename <FILENAME>

For more information try --help
```

Missing file:

```
Error reading input data: No such file or directory (os error 2)
```

Malformed data:

```
Error reading input data: key must be a string at line 35 column 17
```
