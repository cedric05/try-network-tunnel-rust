import socket
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.sendto(b"h"*3000, ("10.0.0.2", 1234))
s.close()