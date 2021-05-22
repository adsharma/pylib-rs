This is a companion library for [py2many](https://github.com/adsharma/py2many/)

There are several differences in the design of python stdlib and rust stdlib.
Transpiling requires that we translate language constructs as well as APIs.

Sometimes the APIs don't translate well. Having these traits helps bridge the
two. For example open("filename").read() on python returns a string. There is
no equivalent function on rust. Generating idiomatic rust code adds complexity
to the transpiler. pylib hopefully makes it possible with less complexity.
