# Security Policy

## Supported Versions

We actively monitor and patch the latest minor release of KOSL. Critical security updates are backported to the immediately preceding minor version where feasible.

| Version | Supported          |
| ------- | ------------------ |
| v0.1.x  | :white_check_mark: |
| < v0.1  | :x:                |

## Reporting a Vulnerability

We take the security of KOSL, its parser, transpiler, and associated tooling seriously. If you find a security vulnerability, please do not report it via public GitHub issues. Instead, follow the process below:

1. Report via Private Vulnerability Reporting in Github, with a detailed description of the vulnerability.
2. Include the following details in your report:
   - Steps to reproduce the issue (including a minimal `.kosl` code sample).
   - The impact of the vulnerability (e.g., Denial of Service, memory exhaustion, execution bypass).
   - The platform and architecture where the issue was reproduced.
3. We will acknowledge receipt of your report within 48 hours.
4. We aim to provide a fix or mitigation plan within 14 days of confirmation.

## Disclosure Policy

Once a fix has been developed and verified, we will:
1. Release a patched version and coordinate a security advisory.
2. Provide credit to the reporter in the release notes and advisory unless requested otherwise.