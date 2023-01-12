# Chena
Chat Server With WebSockets Written in Rust from Scratch (No library, excluding Tokio of course)

## Todo List
What needs to be done
### Http server
- [X] create a tcp socket connection on port
- [X] bind a listen to server
- [X] create a loop for accepting different connections
- [X] for each connection read avilable sockets
- [X] read socket content
    - [X] handle new connection
        - [X] accept socket
        - [X] add socket to a list
    - [X] handel requests
        - [X] read request data
        - [X] parse http
- [ ] Accept CORS and option requests
- [X] Create simple html and resorce host
### handeling handshake
- [X] check for request endpoint (eg: /ws or /websocket or /socket.io)
- [X] check for valid handshake
    - [X] check for version (above 1.1)
    - [X] check if 'upgrade' header exist and is eq 'websocket'
    - [X] check if 'conection' header exist and is eq 'Upgrade'
    - [X] store web-socket-key from 'sec-websocket-key' header
- [X] generate new websocket key
    - [X] append magic uuid to key '258EAFA5-E914-47DA-95CA-C5AB0DC85B11'
    - [ ] hash new key with sha1
    - [ ] encode to base64
- [X] return a response ('HTTP/1.1 101 Switching Protocols\r\n'+'Upgrade: websocket\r\n'+'Connection: Upgrade\r\n'+Sec-WebSocket-Accept: `newkey`\r\n')
### setup testing
- [ ] Create a websocket connection with js library
### handle websocket frame
