# Contribution guidelines

Thank you for your interest in contributing!

## If you want to report an issue

- Describe your environment (OS, architecture, software versions) in detail.
- Clearly outline the steps needed to reproduce the issue.
- Mention any troubleshooting you have done.

## If you want to implement a feature/bugfix

- Discuss your proposed changes before starting.
- Clone the repository.
- Set up the development environment:
    - *If you use Nix*: `nix develop`
    - *Otherwise*: refer to `buildInputs = [ .. ]` in `flake.nix` and install dependencies accordingly.
- Write code.
    - Format code: `just fmt`
- Ensure all checks pass: `just ci`
- Open a pull request.
    - Add a short, clear list of your changes.
