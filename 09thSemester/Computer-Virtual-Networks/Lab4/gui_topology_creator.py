#!/usr/bin/env python3
"""
gui_topology_creator.py

Creating tree topology and saving it as JSON (.mn) file.

Usage example:
    python gui_topology_creator.py --branching 2 --levels 3 --hosts 2 --out topology.mn
"""

import json
import argparse
import math

BASE_APPLICATION = {
    "dpctl": "",
    "ipBase": "10.0.0.0/8",
    "netflow": {
        "nflowAddId": "0",
        "nflowTarget": "",
        "nflowTimeout": "600"
    },
    "openFlowVersions": {
        "ovsOf10": "1",
        "ovsOf11": "0",
        "ovsOf12": "0",
        "ovsOf13": "0"
    },
    "sflow": {
        "sflowHeader": "128",
        "sflowPolling": "30",
        "sflowSampling": "400",
        "sflowTarget": ""
    },
    "startCLI": "0",
    "switchType": "ovs",
    "terminalType": "xterm"
}

CONTROLLER_TEMPLATE = {
    "opts": {
        "controllerProtocol": "tcp",
        "controllerType": "ref",
        "hostname": "c0",
        "remoteIP": "127.0.0.1",
        "remotePort": 6633
    },
    "x": "0.0",
    "y": "0.0"
}


def build_tree(branching: int, levels: int, hosts_per_bottom: int):
    if levels < 1:
        raise ValueError("levels must be >= 1")
    if branching < 1:
        raise ValueError("branching must be >= 1")
    if hosts_per_bottom < 0:
        raise ValueError("hosts_per_bottom must be >= 0")

    controllers = [CONTROLLER_TEMPLATE.copy()]

    switches = []
    hosts = []
    links = []

    # Build tree structure and remember parent->children mapping
    level_nodes = []  # list of lists: level_nodes[0] = [root], ..., level_nodes[-1] = bottom switches
    parent_children = {}  # parent_name -> list of child switch names
    next_switch_id = 1

    # create root switch
    root_name = f"s{next_switch_id}"
    level_nodes.append([root_name])
    switch_by_name = {}
    switch_obj = {
        "number": str(next_switch_id),
        "opts": {
            "controllers": ["c0"],
            "hostname": root_name,
            "nodeNum": next_switch_id,
            "switchType": "default"
        },
        "x": "0.0",
        "y": "0.0"
    }
    switches.append(switch_obj)
    switch_by_name[root_name] = switch_obj
    next_switch_id += 1

    # create child switch levels and links (child -> parent)
    for lvl in range(1, levels):
        parents = level_nodes[lvl - 1]
        this_level = []
        for p in parents:
            for i in range(branching):
                name = f"s{next_switch_id}"
                this_level.append(name)
                obj = {
                    "number": str(next_switch_id),
                    "opts": {
                        "controllers": ["c0"],
                        "hostname": name,
                        "nodeNum": next_switch_id,
                        "switchType": "default"
                    },
                    "x": "0.0",
                    "y": "0.0"
                }
                switches.append(obj)
                switch_by_name[name] = obj
                # record parent->child
                parent_children.setdefault(p, []).append(name)
                # link child -> parent
                links.append({
                    "dest": p,
                    "opts": {},
                    "src": name
                })
                next_switch_id += 1
        level_nodes.append(this_level)

    # attach hosts to every switch in the last level (bottom)
    host_id = 1
    bottom_level = level_nodes[-1]
    for s in bottom_level:
        for i in range(hosts_per_bottom):
            hname = f"h{host_id}"
            hosts.append({
                "number": str(host_id),
                "opts": {
                    "hostname": hname,
                    "nodeNum": host_id,
                    "sched": "host"
                },
                "x": "0.0",  # will set numeric x immediately
                "y": "0.0"   # will set actual y after switch y computed
            })
            # link host -> its switch (src host, dest switch)
            links.append({
                "dest": s,
                "opts": {},
                "src": hname
            })
            host_id += 1

    # Coordinate policy parameters
    host_x_start = 10.0
    host_x_step = 15.0    # produces 10,20,30,...
    controller_y = 20.0
    switch_y_start = 80.0  # root level y
    y_spacing = 100.0      # distance between switch levels
    host_y_offset = 80.0   # hosts below the bottom switches

    # assign X to hosts in a simple left-to-right sequence: 10,20,30,...
    total_hosts = len(hosts)
    host_xs = [host_x_start + i * host_x_step for i in range(total_hosts)]
    for idx, h in enumerate(hosts):
        h["x"] = f"{host_xs[idx]:.1f}"
        # y left as placeholder; we'll compute after computing switch Y

    # compute X for bottom switches:
    switch_xs = {}  # name -> numeric x
    # hosts are ordered in the same sequence we created them: for each bottom switch it has hosts_per_bottom hosts sequentially
    if hosts_per_bottom > 0:
        # mapping: for each bottom switch in order, take next hosts_per_bottom host_xs
        hi = 0
        for s in bottom_level:
            slice_xs = host_xs[hi:hi + hosts_per_bottom]
            if slice_xs:
                sx = sum(slice_xs) / len(slice_xs)
            else:
                sx = host_x_start + hi * host_x_step
            switch_xs[s] = sx
            hi += hosts_per_bottom
    else:
        # no hosts attached: spread bottom switches evenly across a small span
        n = len(bottom_level)
        if n == 1:
            switch_xs[bottom_level[0]] = host_x_start
        else:
            span = (n - 1) * host_x_step * 2
            for i, s in enumerate(bottom_level):
                switch_xs[s] = host_x_start - span / 2 + i * (span / max(1, n - 1))

    # compute X for higher-level switches by averaging their children's X
    # process levels from bottom-1 up to root
    for lvl in range(levels - 2, -1, -1):
        for parent in level_nodes[lvl]:
            children = parent_children.get(parent, [])
            if not children:
                # isolated parent (no children) - keep its X if exists or set to 0
                sx = switch_xs.get(parent, host_x_start)
            else:
                child_xs = [switch_xs[child] for child in children]
                sx = sum(child_xs) / len(child_xs)
            switch_xs[parent] = sx

    # now set switch objects' x and y strings
    for lvl, nodes in enumerate(level_nodes):
        y = switch_y_start + lvl * y_spacing
        for name in nodes:
            sx = switch_xs.get(name, host_x_start)
            sw = switch_by_name[name]
            sw["x"] = f"{sx:.1f}"
            sw["y"] = f"{y:.1f}"

    # set hosts' y based on their parent switch Y (each host was sequentially assigned to bottom switches)
    if hosts_per_bottom > 0:
        hi = 0
        for s in bottom_level:
            parent_sw = switch_by_name[s]
            py = float(parent_sw["y"])
            for i in range(hosts_per_bottom):
                if hi >= len(hosts):
                    break
                hosts[hi]["y"] = f"{py + host_y_offset:.1f}"
                hi += 1
    else:
        # no hosts, nothing to update
        pass

    # controller coordinates: center above root
    root_x = switch_xs.get(level_nodes[0][0], host_x_start)
    controllers[0]["x"] = f"{root_x:.1f}"
    controllers[0]["y"] = f"{controller_y:.1f}"

    result = {
        "application": BASE_APPLICATION,
        "controllers": controllers,
        "hosts": hosts,
        "links": links,
        "switches": switches,
        "version": "2"
    }
    return result


def main():
    p = argparse.ArgumentParser(description="Build tree topology JSON")
    p.add_argument("--branching", "-b", type=int, default=2,
                   help="number of child switches per switch (branching factor)")
    p.add_argument("--levels", "-l", type=int, default=2,
                   help="number of switch levels (1 = root only)")
    p.add_argument("--hosts", "-s", dest="hosts", type=int, default=1,
                   help="number of hosts per bottom-level switch")
    p.add_argument("--out", "-o", type=str, default="topology.json",
                   help="output JSON filename")
    args = p.parse_args()

    topo = build_tree(args.branching, args.levels, args.hosts)

    with open(args.out, "w", encoding="utf-8") as f:
        json.dump(topo, f, ensure_ascii=False, indent=4)
    print(f"Saved topology to {args.out} (branching={args.branching}, levels={args.levels}, hosts_per_bottom={args.hosts})")


if __name__ == "__main__":
    main()


