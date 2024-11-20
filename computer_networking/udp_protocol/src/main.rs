use std::net::UdpSocket;

fn main() {
    // Bind the UDP socket to the address "0.0.0.0:4000"
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();

    // Print the local address the socket is bound to
    println!("Listening on {}", socket.local_addr().unwrap());

    // Create a buffer to store incoming data
    let mut buffer = [0; 1024];

    loop {
        // Clear the buffer before receiving new data
        buffer.iter_mut().for_each(|byte| *byte = 0);

        // Receive data from the socket
        let (size, source) = socket.recv_from(&mut buffer).unwrap();

        // Convert the received data to a string
        let request = String::from_utf8_lossy(&buffer[..size]);

        // Print the received data
        println!("Received {} bytes from {}: {}", size, source, request);

        // Define a response message
        let response = "Hello from the server..!";

        // Send the response back to the source
        socket.send_to(response.as_bytes(), source).unwrap();
    }
}
