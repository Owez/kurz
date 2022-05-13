# Kurz

Cache and distributed key-value store based on gossip networking

## Status

Heavily in-development and not at all suitable for production; hardly any features work as of the time of writing.

### Implemented

- Message trait as the backbone of request/responses
- Actions and a ToAction trait
- Encryption automatically implemented for the Message trait

### Roadmap

- [ ] Structure called Kurz to represent ourself
- [ ] Request which mirrors Action and implements Request
- [ ] Response which mirrors Action and implements Request
- [ ] PingPong action implementation for Request and Response
