# Kurz

Cache and distributed key-value store based on gossip networking

## Status

Heavily in-development and not at all suitable for production; hardly any features work as of the time of writing. The specification is included inside of the library's documentation.

### Implemented

- Message trait as the backbone of request/responses
- Actions and a ToAction trait
- Encryption automatically implemented for the Message trait
- Requests implementing ToAction and Message traits

### Roadmap

- [ ] Structure called Kurz to represent ourself
- [ ] Structure called Peer to represent others
- [ ] Document new Kurz and Peer in specification
- [ ] Response which mirrors Action and implements Request
  - [ ] Implement
  - [ ] Document
- [ ] PingPong action implementation for Request and Response
