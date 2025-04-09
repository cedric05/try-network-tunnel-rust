import socket

# Server configuration
UDP_IP = "192.168.1.139"      # Listen on all interfaces
UDP_PORT = 8081         # Choose your desired port

# Create a UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

# Bind the socket to the IP and port
sock.bind((UDP_IP, UDP_PORT))

print(f"🚀 Listening for UDP packets on {UDP_IP}:{UDP_PORT}")

# Loop to receive data
while True:
    data, addr = sock.recvfrom(65535)  # Max UDP packet size
    print(f"📩 Received {len(data)} bytes from {addr}")
    print(f"🔹 Data: {data!r}")
