# UDP Protocol Example

This project demonstrates a simple UDP protocol implementation in Rust. It includes a basic example of sending and receiving UDP packets.

The source for this project is based on the tutorial from <https://youtu.be/sw3IsrKYmzk>.

## Testing the UDP Protocol

You can test the UDP protocol by sending a message using `nc` (netcat):

```bash
echo -n 'Print abc' | nc -u 127.0.0.1 4000
```

This command sends the message "Print abc" to the UDP server running on `127.0.0.1` at port `4000`.
