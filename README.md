# About

This is a package to serve local files.


# Usage

Install by

```
$ carge install --path .
```

Then, the following command launches the server on the given port (`8000` here):

```
$ serve 8000
Listen on 0.0.0.0:8000
```

If you omit a port, it defaults to `8080`.

```
$ serve
Listen on 0.0.0.0:8080
```

This will serve local files. For a request to `/path/to/file`, the server reads a file `./path/to/file`.
If the path indicates a directory (i.e., ending with a slash `/`), then the server returns `./path/to/file/index.html` instead.
