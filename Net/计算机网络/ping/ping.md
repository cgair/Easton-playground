# 主动探查的查询报文
Source: 192.168.1.10
Target: 192.168.1.31
## [Q] 局域网互相 ping 的情况: 在 Source 运行 ping 192.168.1.31, 发生了什么?
Internet Control Message Protocol
    Type: 8 (Echo (ping) request)
    Code: 0
    Checksum: 0x0ed3 [correct]
    [Checksum Status: Good]
    Identifier (BE): 1 (0x0001)
    Identifier (LE): 256 (0x0100)
    Sequence Number (BE): 1 (0x0001)
    Sequence Number (LE): 256 (0x0100)
    [Response frame: 5]
    Timestamp from icmp data: Sep 18, 2022 14:26:19.000000000 CST
    [Timestamp from icmp data (relative): 0.932621000 seconds]
    Data (48 bytes)

Internet Control Message Protocol
    Type: 0 (Echo (ping) reply)
    Code: 0
    Checksum: 0x16d3 [correct]
    [Checksum Status: Good]
    Identifier (BE): 1 (0x0001)
    Identifier (LE): 256 (0x0100)
    Sequence Number (BE): 1 (0x0001)
    Sequence Number (LE): 256 (0x0100)
    [Request frame: 4]
    [Response time: 0.277 ms]
    Timestamp from icmp data: Sep 18, 2022 14:26:19.000000000 CST
    [Timestamp from icmp data (relative): 0.932898000 seconds]
    Data (48 bytes)

## [Q] 跨路由、跨网关的过程是什么样子的?
