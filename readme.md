# Compilation Database Cleanup #

Postprocess and clean up your `compilation_commands.json`, as generated from your build tool.  Does the following:

 - Converts relative paths into absolute paths for increased readability
 - Removes problematic `cmd /C` prefix in commands on Windows, which confuses clangd
 - Strip linker commands and phony targets which are not real compilation commands
 - Increases readability with unix-style slashes on all platforms (clangd on Windows accepts this)

## Usage ##

`cd` to your project root, and:

`ninja -t compdb | cleanup-compdb > compile_commands.json`

## Project Status ##

Works on the author's Windows machine, using output from Ninja 1.10.2.  Not intended for widespread use just yet.

# Copyright and Credit #

Copyright &copy; 2022 [Frogtoss Games](http://www.frogtoss.com), Inc.
File [LICENSE](LICENSE) covers all files in this repo.

Compilation Database Cleanup by Michael Labbe
<mike@frogtoss.com>