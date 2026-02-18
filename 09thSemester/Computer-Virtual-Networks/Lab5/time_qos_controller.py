import datetime
from ryu.base import app_manager
from ryu.controller import ofp_event
from ryu.controller.handler import CONFIG_DISPATCHER, MAIN_DISPATCHER
from ryu.controller.handler import set_ev_cls
from ryu.ofproto import ofproto_v1_3
from ryu.lib import hub
from ryu.lib.packet import packet, ethernet, ether_types

class TimeBasedQoS(app_manager.RyuApp):
    OFP_VERSIONS = [ofproto_v1_3.OFP_VERSION]

    def __init__(self, *args, **kwargs):
        super(TimeBasedQoS, self).__init__(*args, **kwargs)
        self.mac_to_port = {}
        self.datapaths = {}
        self.meter_id = 1

        # QoS Config (kbps)
        self.BW_HIGH = 100000  # 100 Mbps
        self.BW_LOW = 1000     # 1 Mbps
        self.current_mode = None

        self.monitor_thread = hub.spawn(self._time_monitor)

    @set_ev_cls(ofp_event.EventOFPSwitchFeatures, CONFIG_DISPATCHER)
    def switch_features_handler(self, ev):
        datapath = ev.msg.datapath
        ofproto = datapath.ofproto
        parser = datapath.ofproto_parser

        self.datapaths[datapath.id] = datapath

        # Default flow: Send to Controller
        match = parser.OFPMatch()
        actions = [parser.OFPActionOutput(ofproto.OFPP_CONTROLLER,
                                          ofproto.OFPCML_NO_BUFFER)]
        self.add_flow(datapath, 0, match, actions, use_meter=False)

        # Init Meter
        is_work_hours = self._check_work_hours()
        rate = self.BW_HIGH if is_work_hours else self.BW_LOW
        self.current_mode = 'work' if is_work_hours else 'off'

        self.logger.info(f"Switch {datapath.id} connected. Init Rate: {rate} kbps")
        self.send_meter_mod(datapath, ofproto.OFPMC_ADD, rate)

    @set_ev_cls(ofp_event.EventOFPPacketIn, MAIN_DISPATCHER)
    def _packet_in_handler(self, ev):
        msg = ev.msg
        datapath = msg.datapath
        ofproto = datapath.ofproto
        parser = datapath.ofproto_parser
        in_port = msg.match['in_port']

        try:
            pkt = packet.Packet(msg.data)
            eth = pkt.get_protocols(ethernet.ethernet)[0]

            if eth.ethertype in [ether_types.ETH_TYPE_LLDP, ether_types.ETH_TYPE_IPV6]:
                return

            dst = eth.dst
            src = eth.src
            dpid = datapath.id

            self.mac_to_port.setdefault(dpid, {})
            self.mac_to_port[dpid][src] = in_port

            if dst in self.mac_to_port[dpid]:
                out_port = self.mac_to_port[dpid][dst]
            else:
                out_port = ofproto.OFPP_FLOOD

            actions = [parser.OFPActionOutput(out_port)]

            # Install Flow with QoS Meter
            if out_port != ofproto.OFPP_FLOOD:
                match = parser.OFPMatch(in_port=in_port, eth_dst=dst, eth_src=src)
                if msg.buffer_id != ofproto.OFP_NO_BUFFER:
                    self.add_flow(datapath, 1, match, actions, msg.buffer_id, use_meter=True)
                else:
                    self.add_flow(datapath, 1, match, actions, use_meter=True)

            data = None
            if msg.buffer_id == ofproto.OFP_NO_BUFFER:
                data = msg.data
            out = parser.OFPPacketOut(datapath=datapath, buffer_id=msg.buffer_id,
                                      in_port=in_port, actions=actions, data=data)
            datapath.send_msg(out)
        except:
            return

    def add_flow(self, datapath, priority, match, actions, buffer_id=None, use_meter=True):
        ofproto = datapath.ofproto
        parser = datapath.ofproto_parser
        inst = [parser.OFPInstructionActions(ofproto.OFPIT_APPLY_ACTIONS, actions)]

        if priority > 0 and use_meter:
            inst.append(parser.OFPInstructionMeter(self.meter_id))

        if buffer_id:
            mod = parser.OFPFlowMod(datapath=datapath, buffer_id=buffer_id,
                                    priority=priority, match=match, instructions=inst)
        else:
            mod = parser.OFPFlowMod(datapath=datapath, priority=priority,
                                    match=match, instructions=inst)
        datapath.send_msg(mod)

    def send_meter_mod(self, datapath, command, rate_kbps):
        ofproto = datapath.ofproto
        parser = datapath.ofproto_parser
        bands = [parser.OFPMeterBandDrop(rate=rate_kbps, burst_size=10)]
        req = parser.OFPMeterMod(datapath=datapath, command=command,
                                 flags=ofproto.OFPMF_KBPS, meter_id=self.meter_id, bands=bands)
        datapath.send_msg(req)

    def _check_work_hours(self):
        now = datetime.datetime.now()
        is_weekday = 0 <= now.weekday() <= 4
        is_working_time = 9 <= now.hour < 18

        # FOR TESTING PURPOSES
        IS_WORKING_HOURS_TEST = True

        return IS_WORKING_HOURS_TEST
        # return is_weekday and is_working_time

    def _time_monitor(self):
        while True:
            is_work_hours = self._check_work_hours()
            new_mode = 'work' if is_work_hours else 'off'

            if new_mode != self.current_mode:
                self.logger.info(f"Time Changed -> {new_mode}. Updating Meters...")
                self.current_mode = new_mode
                new_rate = self.BW_HIGH if is_work_hours else self.BW_LOW
                for dp in self.datapaths.values():
                    self.send_meter_mod(dp, dp.ofproto.OFPMC_MODIFY, new_rate)
            hub.sleep(10)

