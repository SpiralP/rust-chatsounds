window.SIDEBAR_ITEMS = {"struct":[["AcceptConn","Returns a value indicating whether or not this socket has been marked to accept connections with `listen(2)`."],["AlgSetAeadAuthSize",""],["AlgSetKey",""],["BindToDevice","Bind this socket to a particular device like “eth0”."],["Broadcast","Set or get the broadcast flag."],["Ip6tOriginalDst",""],["IpAddMembership","Join a multicast group"],["IpDropMembership","Leave a multicast group."],["IpFreebind","If enabled, this boolean option allows binding to an IP address that is nonlocal or does not (yet) exist."],["IpMulticastLoop","Set or read a boolean integer argument that determines whether sent multicast packets should be looped back to the local sockets."],["IpMulticastTtl","Set or read the time-to-live value of outgoing multicast packets for this socket."],["IpTransparent","Setting this boolean option enables transparent proxying on this socket."],["Ipv4PacketInfo","Pass an `IP_PKTINFO` ancillary message that contains a pktinfo structure that supplies some information about the incoming packet."],["Ipv4RecvErr","Enable extended reliable error message passing."],["Ipv4Ttl","Set or retrieve the current time-to-live field that is used in every packet sent from this socket."],["Ipv6AddMembership","Join an IPv6 multicast group."],["Ipv6DropMembership","Leave an IPv6 multicast group."],["Ipv6RecvErr","Control receiving of asynchronous error options."],["Ipv6RecvPacketInfo","Set delivery of the `IPV6_PKTINFO` control message on incoming datagrams."],["Ipv6Ttl","Set the unicast hop limit for the socket."],["Ipv6V6Only","The socket is restricted to sending and receiving IPv6 packets only."],["KeepAlive","Enable sending of keep-alive messages on connection-oriented sockets."],["Linger","When enabled,  a close(2) or shutdown(2) will not return until all queued messages for the socket have been successfully sent or the linger timeout has been reached."],["Mark","Set the mark for each packet sent through this socket (similar to the netfilter MARK target but socket-based)."],["OobInline","If this option is enabled, out-of-band data is directly placed into the receive data stream."],["OriginalDst",""],["PassCred","Enable or disable the receiving of the `SCM_CREDENTIALS` control message."],["PeerCredentials","Return the credentials of the foreign process connected to this socket."],["RcvBuf","Sets or gets the maximum socket receive buffer in bytes."],["RcvBufForce","Using this socket option, a privileged (`CAP_NET_ADMIN`) process can perform the same task as `SO_RCVBUF`, but the `rmem_max limit` can be overridden."],["ReceiveTimeout","Specify the receiving timeout until reporting an error."],["ReceiveTimestamp","Enable or disable the receiving of the `SO_TIMESTAMP` control message."],["ReceiveTimestampns","Enable or disable the receiving of the `SO_TIMESTAMPNS` control message."],["ReuseAddr","Enables local address reuse"],["ReusePort","Permits multiple AF_INET or AF_INET6 sockets to be bound to an identical socket address."],["RxqOvfl","Indicates that an unsigned 32-bit value ancillary message (cmsg) should be attached to received skbs indicating the number of packets dropped by the socket since its creation."],["SendTimeout","Specify the sending timeout until reporting an error."],["SndBuf","Sets or gets the maximum socket send buffer in bytes."],["SndBufForce","Using this socket option, a privileged (`CAP_NET_ADMIN`)  process can perform the same task as `SO_SNDBUF`, but the `wmem_max` limit can be overridden."],["SockType","Gets the socket type as an integer."],["SocketError","Get and clear the pending socket error."],["TcpCongestion","This option allows the caller to set the TCP congestion control algorithm to be used,  on a per-socket basis."],["TcpKeepCount","The maximum number of keepalive probes TCP should send before dropping the connection."],["TcpKeepIdle","The time (in seconds) the connection needs to remain idle before TCP starts sending keepalive probes"],["TcpKeepInterval","The time (in seconds) between individual keepalive probes."],["TcpMaxSeg","The maximum segment size for outgoing TCP packets."],["TcpNoDelay","Under most circumstances, TCP sends data when it is presented; when outstanding data has not yet been acknowledged, it gathers small amounts of output to be sent in a single packet once an acknowledgement is received.  For a small number of clients, such as window systems that send a stream of mouse events which receive no replies, this packetization may cause significant delays.  The boolean option TCP_NODELAY defeats this algorithm."],["TcpRepair",""],["TcpUserTimeout","Specifies the maximum amount of time in milliseconds that transmitted data may remain unacknowledged before TCP will forcibly close the corresponding connection"],["UdpGroSegment",""],["UdpGsoSegment",""]]};