# Security Policy

BoardClaw is pre-alpha planning and bootstrap code. Do not deploy it to control
real hardware without reviewing the implementation and adding physical safety
interlocks.

## Reporting Security Issues

Until a public security contact is established, open a private maintainer
channel or file a minimal public issue that asks for a security contact without
including exploit details.

## Security Principles

- The model is not trusted.
- Raw root shell access is not a feature.
- Write tools are gated by default.
- Physical safety cannot be delegated to prompts.
- High-risk actions should become explicit approval and receipt flows when that
  integration exists.
