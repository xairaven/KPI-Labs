#!/usr/bin/env python3
from mininet.net import Mininet
from mininet.node import Controller, OVSSwitch
from mininet.cli import CLI
from mininet.log import setLogLevel, info
import os

def run():
    net = Mininet(controller=Controller, switch=OVSSwitch)
    c0 = net.addController('c0')

    variant = 7
    for i in range(variant):
        index = i + 1
        switch = net.addSwitch(f"s{index}")
        connected_host = net.addHost(f"h{index}", ip=f"10.0.0.{index}/24")
        net.addLink(switch, connected_host)

        if index > 1:
            previous_switch = net.getNodeByName(f"s{index-1}")
            net.addLink(previous_switch, switch)

    net.start()

    info('*** Adding internal management port s1-mgmt with IP 10.0.0.254/24\n')
    os.system('ovs-vsctl add-port s1 s1-mgmt -- set interface s1-mgmt type=internal')

    os.system('ip addr add 10.0.0.254/24 dev s1-mgmt || true')
    os.system('ip link set s1-mgmt up || true')

    info('*** Setup complete - entering CLI\n')
    CLI(net)
    net.stop()

if __name__ == "__main__":
    setLogLevel('info')
    run()
