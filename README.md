## create tun0 via


```shell
sudo ip tuntap add dev tun0 mode tun
sudo ip addr add 10.0.0.1/24 dev tun0
sudo ip link set tun0 up
```

## run
`cargo run`
and run `ping 10.0.0.2` to see packets.