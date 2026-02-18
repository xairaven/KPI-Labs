from mininet.topo import Topo
from mininet.net import Mininet
from mininet.node import RemoteController, OVSKernelSwitch
from mininet.cli import CLI
from mininet.log import setLogLevel, info
from mininet.link import TCLink

class SimpleDCTopo(Topo):
    """
    Simplified Data Center Topology (Tree based).
    No physical loops, so STP is NOT required.
    Structure: Core -> Aggregation -> Edge -> Hosts.
    """
    def build(self):
        # --- Create Switches ---
        # 1 Core Switch
        c1 = self.addSwitch('c1', dpid='0000000000000001',
                            protocols='OpenFlow13')

        # 4 Aggregation Switches
        aggrs = []
        for i in range(4):
            sw = self.addSwitch(f'a{i+1}', dpid=f'000000000000001{i+1}',
                                protocols='OpenFlow13')
            aggrs.append(sw)

        # 8 Edge Switches
        edges = []
        for i in range(8):
            sw = self.addSwitch(f'e{i+1}', dpid=f'000000000000002{i+1}',
                                protocols='OpenFlow13')
            edges.append(sw)

        # --- Create Links (No Loops) ---

        # 1. Connect Core to all Aggregation switches
        for a_sw in aggrs:
            self.addLink(c1, a_sw)

        # 2. Connect Aggregation to Edge (Simple Tree structure)
        # a1 -> e1, e2; a2 -> e3, e4 ...
        edge_idx = 0
        for a_sw in aggrs:
            self.addLink(a_sw, edges[edge_idx])
            self.addLink(a_sw, edges[edge_idx+1])
            edge_idx += 2

        # 3. Connect Hosts to Edge (2 Hosts per Edge)
        # e1 -> h1, h2; e2 -> h3, h4 ...
        host_count = 1
        for e_sw in edges:
            h1 = self.addHost(f'h{host_count}')
            h2 = self.addHost(f'h{host_count+1}')
            self.addLink(e_sw, h1)
            self.addLink(e_sw, h2)
            host_count += 2

def run_topology():
    topo = SimpleDCTopo()
    net = Mininet(topo=topo,
                  controller=RemoteController,
                  switch=OVSKernelSwitch,
                  link=TCLink,
                  autoSetMacs=True)

    info("**** Starting Network ****\n")
    net.start()

    # Static ARP is helpful to speed up discovery
    info("**** Enabling Static ARP ****\n")
    net.staticArp()

    info("**** Network Ready (No loops, full connectivity) ****\n")
    CLI(net)

    info("**** Stopping Network ****\n")
    net.stop()

if __name__ == '__main__':
    setLogLevel('info')
    run_topology()

