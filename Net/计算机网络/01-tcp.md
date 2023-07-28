
# tcp 篇

## TCP Lab 
### tcpdump and Wireshark
> 工欲善其事, 必先利其器.
*  Wireshark 除了可以抓包外, 还提供了可视化分析网络包的图形⻚面.
* tcpdump 常用在 Linux 服务器中抓取和分析网络包.

#### tcpdump
常用选项和过滤表达式:
**例如:**
```bash
ping -I eth1 -c 3 xxx
```
使用 tcpdump 抓包 ping 数据包
```bash
tcpdump -i eth1 icmp and host xxx -nn
```

```bash
# tcpdump 常用选项
# -i        tcpdump -i eth1        指定网络接口, 默认是0号接口(如eth0), any表示所有接口
# -nn       tcpdump -nn            不解析 IP 地址和端口号名称
# -c        tcpdump -c 5           限制抓取的网络包的个数
# -w        tcpdump -w a.pcap      保存到文件中
#
# tcpdump 常用过滤表达式
# host, src host, dst host              tcpdump -nn host 192.168.1.100              主机过滤
# port, src port, dst port              tcpdump -nn port 80                         端口过滤
# ip, ip6, arp, tcp, udp, icmp          tcpdump -nn host tcp                        协议过滤
# and, or, not                          tcpdump -nn host 192.168.1.100 and port 80  逻辑表达式
# tcp[tcoflages]                        tcpdump -nn "tcp[tcoflages] & tcp-syn !=0"  特定状态的 TCP 包
```
**输出格式:** 时间戳 协议 源地址.源端口 > 目的地址.目的端口 网络包详细信息

### [Lab1 - TCP 三次握手和四次挥手]
* 在一台可以外网访问的服务器上 `python3 -m http.server` 启动一个简单的 HTTP 服务器, 并在当前目录下的默认端口 8000 上监听传入的 HTTP 请求.
* tcpdump 抓包
```bash
sudo tcpdump -i any tcp and host <ip> and port 8000 -w http.pcap
```
然后客户端执行 `curl <url>`
* 使用 Wireshark 打开 http.pcap


Wireshark 可以用时序图的方式显示数据包交互的过程
* Statistics -> Flow Graph, Flow Type 选择 TCP Flows 看到整个过程中 TCP 流 的执行过程



## FAQ
### UDP 和 TCP 有什么区别呢? 分别的应用场景是?
《图解网络》 P128

UDP 经常用于包总较少的通信:
* 如 DNS, SNMP 等 
* 视频、音频等多媒体通信
* 广播通信
  
### TCP 三次握手过程? 

### TCP 为什么是三次握手? 不是两次、四次?

### TCP 四次挥手过程?

### 为什么挥手需要四次?

### 为什么需要 TIME_WAIT 状态?


### TCP 的重传机制 - 万一数据在传输过程中丢失了呢?
《图解网络》 P162
TCP 针对数据包丢失的情况, 用重传机制解决:
* 超时重传「以时间为驱动重传」超时周期可能相对较⻓ -> 快速重传
* 快速重传「以数据为驱动重传」重传时是重传之前一个还是所有? -> SACK
* SACK (Selective Acknowledgment 选择性确认) 是发出去的包丢了, 还是接收方回应的 ACK 包丢了? -> D-SACK
  * TCP 头部**选项**字段里加一个 SACK, 必须双方都要支持. Linux 下可通过 net.ipv4.tcp_sack 参数打开这个功能 (Linux 2.4 后默认打开)
* D-SACK (Duplicate SACK)
  * Linux 下可通过 net.ipv4.tcp_dsack 参数开启/关闭这个功能 (Linux 2.4 后默认打开)



### TCP 的滑动窗口
《图解网络》 P172

TCP 每发送一个数据, 都要进行一次确认应答. 上一个数据包收到了应答了, 再发送下一个.
* 滑动窗口: 无需等待确认应答, 可以继续发送数据.

### TCP 的流量控制

### TCP 的拥塞控制
《图解网络》 P188

防止过多数据注入到网络中, 使网络中的路由器和链路过载.

#### 怎么知道当前网络是否出现了拥塞呢?
只要「发送方」没有在规定时间内接收到 ACK 应答报文 (发生了超时重传) 就认为网络出现了用拥塞.


## 提升 TCP 的性能
操作系统提供了许多调节 TCP 的参数: `ls /proc/sys/net/ipv4/tcp*`
* Linux 系统中 `/proc` 目录 (in memory 的) 提供了对内核运行时状态的访问.
![TCP 内核参数](https://pic3.zhimg.com/80/v2-babe9549eeef43ccf515f5ca5bb6abd6_1440w.webp)

本章从三个⻆度阐述提升 TCP 的策略.

### TCP 三次握手的性能提升
在网络状态不佳、高并发或者遭遇 SYN 攻 击等场景中, 如果不能有效正确的调节三次握手中的参数, 就会影响性能.




### TCP 四次挥手的性能提升


### TCP 数据传输的性能提升

# See also
* 《图解网络》- tcp 篇 (小林 coding 著)