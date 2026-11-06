# Security policy

## Report a vulnerability

Please report it [privately on GitHub](https://github.com/sventmueller/lowspeak/security/advisories/new), not in an
issue or a discussion: both are public. Keep it private until a fix is out.

Name the Lowspeak version and the macOS version and describe how to reproduce it.

## What counts

Lowspeak hears everything you say while fn is held, can type into any app and keeps your API keys. Anything that breaks
a promise under [Privacy and security](README.md#privacy-and-security) is a vulnerability, for example audio or text
that reaches anyone but the provider you chose, an API key that leaves the keychain other than in the request to its
provider, or text pasted somewhere you did not dictate it.

## Supported versions

Security fixes go into the latest release only.
