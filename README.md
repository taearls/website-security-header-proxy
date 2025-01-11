# Website Security Header Proxy

## Overview

This project contains the code for a [Cloudflare Worker](https://workers.cloudflare.com) built in Rust that acts as a shared middleware service for my websites.

It provides Content Security Policy definitions, and injects headers for security best practices.

## Security Headers

With this service, I'm able to get a perfect score on [securityheaders.com](https://securityheaders.com) when scanning my websites:

### Security Headers Scan Results

- [tylerearls.com](https://securityheaders.com/?q=tylerearls.com&followRedirects=on)
- [cuckooandthebirds.com](https://securityheaders.com/?q=cuckooandthebirds.com&followRedirects=on)
