 # TODO

## 1: Skeleton Commit
- [x] Create project skeleton with server, client, and transport
- [x] Define Messages (ClientMessage, ServerMessage)
- [x] Configure Cargo for both binaries

---

## 2: Server networking skeleton
- [ ] Create TCP listener
- [ ] Accept incoming connections
- [ ] Spawn per-connection tasks
- [ ] Handle client disconnects

---

## 3: Connection framing & message parsing
- [ ] Parse client messages
- [ ] Detect EOF and cleanup connection

---

## 4: Server state Management
- [ ] Add shared server state (connected users)
- [ ] Store per-user message channels

---

## 5: User lifecycle management
- [ ] Implement JOIN handling
- [ ] Enforce unique usernames
- [ ] Cleanup state on LEAVE or disconnect

---

## 6: Message broadcasting
- [ ] Implement SEND handling
- [ ] Broadcast messages to all users except sender
- Ensure non-blocking

---

## 7: Client networking
- [ ] Connect to server on startup
- [ ] Send JOIN message
- [ ] Spawn async read/write tasks

---

## 8: Client CLI interaction
- [ ] Implement CLI prompt
- [ ] Parse `send <msg>` command
- [ ] Implement `leave` command
- [ ] Exit on disconnect

---

## 9: Error handling
- [ ] Handle server-side IO errors gracefully
- [ ] Handle client-side disconnects
- [ ] Prevent panics on malformed input

---

## 10: Testing
- [ ] Add unit tests for transport
- [ ] Add integration tests for server-client interaction

---

## 11: CI & polish
- [ ] Add GitHub Actions CI because hooks are bad for me
- [ ] Enforce formatting, clippy, and tests
- [ ] Final cleanup and documentation
