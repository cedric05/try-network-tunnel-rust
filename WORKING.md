Here’s a cleaner and more structured rewrite of your setup instructions:

---

## 🔧 Network Tunneling Setup Between Two Groups (Server ↔ Client)

### 🧠 Assumptions

1. **Server Gateway**
   - Private IP: `192.168.0.10`
   - Network: `192.168.0.0/16`
   - Also has a **public IP** accessible from outside.

2. **Client Gateway**
   - Private IP: `172.30.0.15`
   - Network: `172.30.0.0/16`

🎯 **Goal:** Machines in both networks should be able to communicate with each other over a secure tunnel using TUN interfaces.

---

## 🖥️ Server-Side Setup (on gateway: `192.168.0.10`)

```bash
# 1. Create a TUN interface
sudo ip tuntap add dev tun0 mode tun

# 2. Assign an IP for the remote (client) side network
sudo ip addr add 172.30.0.0/16 dev tun0

# 3. Bring up the TUN interface
sudo ip link set tun0 up

# 4. Enable IP forwarding
sudo sysctl -w net.ipv4.ip_forward=1

# 5. Start tunnel in server mode (listening on public IP)
./target/release/tunnel server --bind-ip <public-ip> --port 8080
```

🖧 On **each machine** in the **server-side network**:
```bash
sudo ip route add 172.30.0.0/16 via 192.168.0.10 dev eth0
```

---

## 🖥️ Client-Side Setup (on gateway: `172.30.0.15`)

```bash
# 1. Create a TUN interface
sudo ip tuntap add dev tun0 mode tun

# 2. Assign an IP for the remote (server) side network
sudo ip addr add 192.168.0.0/16 dev tun0

# 3. Bring up the TUN interface
sudo ip link set tun0 up

# 4. Enable IP forwarding
sudo sysctl -w net.ipv4.ip_forward=1

# 5. Start tunnel in client mode (connects to server's public IP)
./target/release/tunnel client --bind-ip <server-public-ip> --port 8080
```

🖧 On **each machine** in the **client-side network**:
```bash
sudo ip route add 192.168.0.0/16 via 172.30.0.15 dev eth0
```

---

## ✅ Outcome

- Packets from **192.168.0.0/16** (server network) destined to **172.30.0.0/16** (client network) are routed through the **server gateway**.
- Packets from **client machines** are routed through the **client gateway**.
- The tunnel allows IP-layer connectivity using the TUN interface as a bridge between the networks.

