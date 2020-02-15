## Merger

Binary to merge yaml files and print to the standard output.

Example:
```
$ cat foo.yml
foo: bar
foobar: bar
$ cat bar.yml
foo: foo
bar:
  foo: test
  list:
    - foo
    - bar
    - foobar

$ merger foo.yml bar.yml
foobar: bar
foo: foo
bar:
  foo: test
  list:
    - foo
    - bar
    - foobar
```

### Install
`cargo install merger`
or download it from the releases page.
