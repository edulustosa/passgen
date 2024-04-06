# Passgen

A simple password generator for the terminal. It generates random characters based on the current thread of execution and copies them directly to the clipboard.

## Usage

``` bash
passgen <optional length>
```

**Output**:

``` bash
Password: <random_password>
Copied to clipboard
```

## Setup

1. **Clone the repository**

``` bash
git clone https://github.com/edulustosa/passgen.git
```

2. **Move into the project directory and copy the binaries to the global directory**

``` bash
sudo cp release/passgen /usr/local/bin/
```

3. **You may need to grant execution permissions**

``` bash
sudo chmod +x /usr/local/bin/passgen
```

4. **Finally, update the binaries cache**

``` bash
sudo update-binfmts --import
```
