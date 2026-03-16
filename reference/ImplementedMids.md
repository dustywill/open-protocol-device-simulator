# Fastening Protocol

Ujigami uses the following Open Protocol MIDs to communicate with fastening controllers:
MID | Description |	Sent By |	Notes
---|---|---|---
0001 | Communication Station | Ujigami | 
0002 | Communication Start ACK | Controller | 
0004 | Communication negative ACK | Controller | 
0005 | Communication positive ACK | Controller | 
0010 | Parameter set ID upload request | Ujigami | Optional – only needed for controllers (e.g. PF6000) that prevent MID 0018 without this request
0011 | Parameter set ID upload reply | Controller | Optional per above
0014 | Parameter set selected subscribe | Ujigami | 
0015 | Parameter set selected | Controller | Controller must send this packet immediately after it sends MID 0005 ACK to the MID 0014 subscription request to inform Ujigami which Pset is active at subscription time, and subsequently whenever a new Pset is activated
0016 | Parameter set selected ACK | Ujigami | 
0018 | Select Parameter set | Ujigami | 
0038 | Select Job | Ujigami | Optional – needed for controllers that mistake Pset for Job. All Job functionality (e.g. batch counting) should be disabled in the controller.
0042 | Disable tool | Ujigami | 
0043 | Enable tool | Ujigami | 
0060 | Last tightening result data subscribe | Ujigami | Or MID 0100 for multi-spindle controllers
0061 | Last tightening result data | Controller | 
0062 | Last tightening result data ACK | Ujigami | 
0082 | Set Time | Ujigami | Optional – used to ensure controller time is correct for MID 0060 tightening data, etc.
0100 | Multi-spindle result subscribe | Ujigami | Optional – only used by controllers with multi-spindle tools
0101 | Multi-spindle result | Controller | 
0102 | Multi-spindle result ACK | Ujigami | 
0214 | IO device status request | Ujigami | Deprecated – use Relay functions instead
0215 | IO device status reply | Controller | 
0216 | Relay function subscribe | Ujigami | Ujigami only uses relays 20 Tool start switch, and 22 Direction switch = CCW (i.e. reverse)
0217 | Relay function | Controller | 
0218 | Relay function ACK | Ujigami | 
9999 | Keep alive message | Both | Upon receipt, Controller mirrors this message back to Ujigami