# dprint-plugin-shfmt

Shell formatting plugin for dprint.

This uses the [shfmt](https://github.com/mvdan/sh#shfmt) parser.

## Install

See [Release](https://github.com/TiceCosmos/dprint-plugin-shfmt/releases/latest)

## Configuration

See [Options](https://github.com/mvdan/sh/blob/master/cmd/shfmt/shfmt.1.scd#options)

| Name             | Type | Default | description  |
| :--------------- | :--- | ------: | :----------- |
| indentWidth      | u8   |       2 | indent width |
| binaryNextLine   | bool |   false | like -bn     |
| switchCaseIndent | bool |   false | like -ci     |
| spaceRedirects   | bool |   false | like -sr     |
| keepPadding      | bool |   false | like -kp     |
| functionNextLine | bool |   false | like -fn     |
