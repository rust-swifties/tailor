# Tailor

**Tailor** [ˈteɪ.lə˞] is a command-line tool inspired by the utility of the UNIX `tail` command
and the logical inclusivity of the `or` operator. It allows you to view the end of files with
automatic recovery. If the file doesn't exist or isn't accessible, tailor executes your
specified fallback command instead.

## Usage

```bash
cargo build
./targer/debug/tailor <file> [fallback_command] [args...]
```

- If the file exists and is readable, tailor will tail it: `tail <file>`
- If the file doesn't exist or isn't accessible, tailor executes the specified command:
  `[fallback_command] [args...] <file>`

Run `./target/debug/tailor --help` for more information.

# Use Cases

Tailor bridges the gap between “I want to read this file” and “This file doesn’t exist yet.” It’s
a single-command solution for reactive file access with smart fallback logic.

View logs or create them:

```bash
tailor /var/log/myapp.log touch
# Runs: tail /var/log/myapp.log
# OR:   touch /var/log/myapp.log
```

Check for build artifact or rebuild it:

```bash
tailor dist/index.js ./scripts/build-artifact.sh
# Runs fallback: ./scripts/build-artifact.sh dist/index.js
```

View files with permission management:

```bash
tailor /etc/ssl/private.key chmod 600
tailor /var/log/secure.log chown $USER:$USER
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your
changes. Ensure your code follows the project's style guidelines and includes tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The UNIX `tail` command, for its timeless utility.
- The logical `or` operator, for teaching us inclusivity.
