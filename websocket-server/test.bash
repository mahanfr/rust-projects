#!/bin/sh

# curl -X POST -H "Content-Type: application/json" -d '{"name": "linuxize", "email": "linuxize@example.com"}' http://127.0.0.1:8000
# curl -X GET http://127.0.0.1:8000
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" -H "Sec-WebSocket-Key: GgOUIVjs2Q9hkKrJaoMrGIQlrrs=" http://127.0.0.1:8000/ws