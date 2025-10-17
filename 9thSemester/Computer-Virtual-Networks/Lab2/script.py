#!/usr/bin/env python3
from mininet.net import Mininet
from mininet.node import Controller, OVSSwitch
from mininet.cli import CLI
from mininet.log import setLogLevel, info
import os

def run():
    net = Mininet(controller=Controller, switch=OVSSwitch)
    c0 = net.addController('c0')
    s1 = net.addSwitch('s1')
    h1 = net.addHost('h1', ip='10.0.0.1/24')
    h2 = net.addHost('h2', ip='10.0.0.2/24')
    h3 = net.addHost('h3', ip='10.0.0.3/24')

    net.addLink(h1, s1)
    net.addLink(h2, s1)
    net.addLink(h3, s1)

    net.start()

    info('*** Adding internal management port s1-mgmt with IP 10.0.0.254/24\n')
    os.system('ovs-vsctl add-port s1 s1-mgmt -- set interface s1-mgmt type=internal')

    os.system('ip addr add 10.0.0.254/24 dev s1-mgmt || true')
    os.system('ip link set s1-mgmt up || true')

    info('*** Starting HTTP servers on h2 and h3 (port 80)\n')
    h2.cmd('python3 -m http.server 80 &')
    h3.cmd('python3 -m http.server 80 &')

    info('*** Setup complete - entering CLI\n')
    CLI(net)
    net.stop()

if __name__ == "__main__":
    setLogLevel('info')
    run()
