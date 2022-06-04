# Kurz

Event streaming and key-value store built on gossiping

## Status

Heavily in-development and not at all suitable for production; hardly any features work as of the time of writing. The specification is included inside of the library's documentation.

### Implemented

- Message trait as the backbone of request/responses
- Actions and a ToAction trait
- Encryption automatically implemented for the Message trait
- Request and Response implementing ToAction and Message traits
- Kurz and Peer representing peers

### Roadmap

- [ ] Document new Kurz and Peer in specification
- [ ] Finish KeySend request
  - [x] Encoding
  - [x] Decoding
  - [ ] Handling incoming
  - [ ] Testing/examples
