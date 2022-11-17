# Building / Installing:

1. Build the package: mv executable to /target/deb_release_build/bin
2. Install the package: from /target : 'sudo dpkg -i deb_release_build.deb'

note: /deb_release_build/ should look like this:
- /bin
    - EXECUTABLE
- /DEBIAN
    - control
- /etc

cat file 'control':
`
Package: PACKAGE_NAME
Version: 0.1
Section: utils
Priority: optional
Architecture: all
Maintainer: me <me@me.de>
Description: This is a test
`

# Uninstall package:

- run 'sudo dpkg -r PACKAGE_NAME'
