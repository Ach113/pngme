## pngme

My implementation of [pngme](https://picklenerd.github.io/pngme_book/). Encodes/decodes secret messages inside png files. 

## How to use:
```
USAGE:
    pngme.exe <path> <chunk-name> [FLAGS] [OPTIONS]
        FLAGS:
            -d, --decode    retrieves message from <chunk-name>
            -h, --help    Prints help information
            -l, --list    lists existing chunk names in file
            -r, --remove    removes specified <chunk-name> from file
            -V, --version    Prints version information
        OPTIONS:
            -e, --encode <message>    writes <message> into file
        ARGS:
            <path>    path to the .png file
            <chunk-name>    four letter alias of a chunk in .png file
``` 
## Example:
```
$ cargo run -- dice.png -l
    chunk type: IHDR, 25 bytes
    chunk type: IDAT, 22719 bytes
    chunk type: IEND, 12 bytes
```
These are the default contents of a .png file: header `IHDR`, tail `IEND`, and `IDAT`.
`IDAT` chunk contains the actual data of a .png file. When viewing the contents of the file,
only contents of `IDAT` will be displayed. Contents of any other chunk are virtually 
invisible to the viewer. This allows us to encode "secret" messages inside .png files
by inserting new chunks of data into the file.

(It's recommended to refrain from naming new chunks `IHDR`, `IEND` or `IDAT`)
```
$ cargo run -- dice.png SCRT -e "birds aren't real"
$ cargo run -- dice.png -l
    chunk type: IHDR, 25 bytes
    chunk type: IDAT, 22719 bytes
    chunk type: IEND, 12 bytes
    chunk type: SCRT, 29 bytes
```
We can retrieve the message by specifying the chunk name. 
(note, program will crash if contents of `IHDR`, `IDAT` and `IEND` are read)
```
$ cargo run -- dice.png SCRT
    birds aren't real
```
We can then remove the chunk (and the message within it).
```
$ cargo run -- dice.png SCRT -r
$ cargo run -- dice.png -l
    chunk type: IHDR, 25 bytes
    chunk type: IDAT, 22719 bytes
    chunk type: IEND, 12 bytes
```