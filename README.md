# Composer run-script and Windows console handle

This repo demonstrates how `composer run-script` fails to pass the Windows console handle to child processes. This breaks scripts with an interactive console on Windows.

Execute the following commands in PowerShell.

If you don't have a Rust toolchain, you can copy the precompiled `dia.exe` from the root into `./target/debug/dia.exe` and skip the first command.

The second command shows the expected behavior: interractive option selection.
The third command demonstrates the issue.

```
composer run-script build
./target/debug/dia.exe
composer run-script dia
```
