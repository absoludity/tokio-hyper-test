This is just an attempt at producing a minimal reproduction of the issue that described at [Kubeapps #5407](https://github.com/vmware-tanzu/kubeapps/issues/5407).

Start the downstream http server in one terminal:

```bash
RUST_LOG=info cargo run --bin downstream
```

and the proxy server in another terminal:

```bash
RUST_LOG=info,mio=trace cargo run --bin proxy
```

and then hit the proxy server with 50 requests simultaneously (EDIT: they are not close enough - printing the `date -uIns` inside the `do` below shows that at it takes over 30ms to *begin* 20-30 curls... so need to find another way to do them simultanously here).

```bash
for i in {1..50}; do curl "http://127.0.0.1:3000/${i}" & ; done
```

Annotated logs after running the above, which show that requests begin completing as expected after 30ms or so, before the 33rd request has even begun being proxied (though as above, yet to confirm if this is because the curl command hasn't completed - I'll `time` it shortly.)

```
   Compiling tokio-hyper-test v0.1.0 (/home/michael/dev/vmware/tokio-hyper-test)
    Finished dev [unoptimized + debuginfo] target(s) in 4.14s
     Running `target/debug/proxy`
TRACE 2022-10-19T13:38:15.957770: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:15.957914: proxy: Starting server on 127.0.0.1:3000
```

First event sources registered at start at 23.415,

```
TRACE 2022-10-19T13:38:23.415126: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415364: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415530: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415689: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415776: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415857: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.415938: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416025: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416148: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416235: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416371: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416491: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416668: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416754: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.416881: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417002: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417085: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417212: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417333: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417420: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417575: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417680: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417804: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.417886: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418008: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418132: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418255: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418344: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418470: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418591: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418675: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418803: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.418923: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419006: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419132: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419250: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419335: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419461: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419592: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419674: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419762: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419860: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.419965: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420082: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420169: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420297: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420418: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420501: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420625: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.420744: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First request proxy started at 23.421 - 6ms later.

```
 INFO 2022-10-19T13:38:23.421615: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T13:38:23.422001: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.423119: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T13:38:23.423338: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.423634: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T13:38:23.423670: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T13:38:23.423677: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T13:38:23.423717: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T13:38:23.423718: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T13:38:23.424036: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424042: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.424052: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T13:38:23.424074: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T13:38:23.424191: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
TRACE 2022-10-19T13:38:23.424197: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424041: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.424144: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T13:38:23.424194: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
TRACE 2022-10-19T13:38:23.424336: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424038: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424359: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424373: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424469: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.424481: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.424561: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T13:38:23.424626: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
TRACE 2022-10-19T13:38:23.424733: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.424737: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T13:38:23.424742: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T13:38:23.424751: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
 INFO 2022-10-19T13:38:23.424907: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
TRACE 2022-10-19T13:38:23.424918: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.425485: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T13:38:23.425512: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.425538: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.425667: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
TRACE 2022-10-19T13:38:23.425696: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.425877: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
TRACE 2022-10-19T13:38:23.425930: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426092: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T13:38:23.423567: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T13:38:23.426240: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T13:38:23.426242: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
TRACE 2022-10-19T13:38:23.426275: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426413: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
TRACE 2022-10-19T13:38:23.426482: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426504: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426524: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426434: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T13:38:23.426426: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
 INFO 2022-10-19T13:38:23.426624: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T13:38:23.426648: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T13:38:23.426647: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426647: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426677: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
TRACE 2022-10-19T13:38:23.426708: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426747: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426787: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426789: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T13:38:23.426727: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
TRACE 2022-10-19T13:38:23.426804: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426841: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.426865: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426875: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T13:38:23.426886: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
 INFO 2022-10-19T13:38:23.426915: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
TRACE 2022-10-19T13:38:23.426941: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.426937: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T13:38:23.426955: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.427003: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T13:38:23.427065: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.427140: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
TRACE 2022-10-19T13:38:23.427261: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.427284: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.427290: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.427397: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T13:38:23.427416: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T13:38:23.427479: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
TRACE 2022-10-19T13:38:23.427555: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.427631: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.427693: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
 INFO 2022-10-19T13:38:23.427756: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T13:38:23.427823: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.427878: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.427948: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T13:38:23.428069: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.428159: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T13:38:23.428174: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
TRACE 2022-10-19T13:38:23.428232: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.428327: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.428373: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T13:38:23.428644: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:38:23.428779: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T13:38:23.428783: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
```

All started by 23.428 - 7ms later, and first result comes in at 23.454 - 33ms after they began.

```
 INFO 2022-10-19T13:38:23.454286: proxy: Finished proxy of http://127.0.0.1:3030/12
TRACE 2022-10-19T13:38:23.454534: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.456201: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T13:38:23.456421: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.456535: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T13:38:23.456710: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.456822: proxy: Finished proxy of http://127.0.0.1:3030/13
 INFO 2022-10-19T13:38:23.456853: proxy: Finished proxy of http://127.0.0.1:3030/3
 INFO 2022-10-19T13:38:23.456889: proxy: Finished proxy of http://127.0.0.1:3030/10
 INFO 2022-10-19T13:38:23.456889: proxy: Finished proxy of http://127.0.0.1:3030/6
 INFO 2022-10-19T13:38:23.456958: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T13:38:23.456989: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.457013: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.457043: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.457043: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.457132: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.457790: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T13:38:23.457943: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.458045: proxy: Finished proxy of http://127.0.0.1:3030/15
TRACE 2022-10-19T13:38:23.458171: mio::poll: registering event source with poller: token=Token(16777272), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458178: mio::poll: registering event source with poller: token=Token(16777276), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458176: mio::poll: registering event source with poller: token=Token(16777273), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458173: mio::poll: registering event source with poller: token=Token(16777275), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458197: mio::poll: registering event source with poller: token=Token(16777274), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458207: mio::poll: registering event source with poller: token=Token(16777279), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:38:23.458237: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.458525: proxy: Finished proxy of http://127.0.0.1:3030/18
TRACE 2022-10-19T13:38:23.458691: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.458831: proxy: Finished proxy of http://127.0.0.1:3030/19
 INFO 2022-10-19T13:38:23.458835: proxy: Finished proxy of http://127.0.0.1:3030/2
TRACE 2022-10-19T13:38:23.459004: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.459006: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.459487: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T13:38:23.459652: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.459797: proxy: Finished proxy of http://127.0.0.1:3030/39
 INFO 2022-10-19T13:38:23.459815: proxy: Finished proxy of http://127.0.0.1:3030/30
 INFO 2022-10-19T13:38:23.459820: proxy: Finished proxy of http://127.0.0.1:3030/9
 INFO 2022-10-19T13:38:23.459842: proxy: Finished proxy of http://127.0.0.1:3030/41
 INFO 2022-10-19T13:38:23.459864: proxy: Finished proxy of http://127.0.0.1:3030/20
TRACE 2022-10-19T13:38:23.459962: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.459963: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.459982: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.459990: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460021: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.460182: proxy: Finished proxy of http://127.0.0.1:3030/22
 INFO 2022-10-19T13:38:23.460184: proxy: Finished proxy of http://127.0.0.1:3030/27
 INFO 2022-10-19T13:38:23.460217: proxy: Finished proxy of http://127.0.0.1:3030/37
 INFO 2022-10-19T13:38:23.460247: proxy: Finished proxy of http://127.0.0.1:3030/38
TRACE 2022-10-19T13:38:23.460335: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460337: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460367: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460407: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.460445: proxy: Finished proxy of http://127.0.0.1:3030/11
 INFO 2022-10-19T13:38:23.460528: proxy: Finished proxy of http://127.0.0.1:3030/40
 INFO 2022-10-19T13:38:23.460557: proxy: Finished proxy of http://127.0.0.1:3030/17
TRACE 2022-10-19T13:38:23.460595: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.460605: proxy: Finished proxy of http://127.0.0.1:3030/49
 INFO 2022-10-19T13:38:23.460619: proxy: Finished proxy of http://127.0.0.1:3030/50
TRACE 2022-10-19T13:38:23.460679: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460707: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460760: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.460766: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.460892: proxy: Finished proxy of http://127.0.0.1:3030/36
 INFO 2022-10-19T13:38:23.460894: proxy: Finished proxy of http://127.0.0.1:3030/29
 INFO 2022-10-19T13:38:23.460926: proxy: Finished proxy of http://127.0.0.1:3030/44
 INFO 2022-10-19T13:38:23.460978: proxy: Finished proxy of http://127.0.0.1:3030/26
 INFO 2022-10-19T13:38:23.460985: proxy: Finished proxy of http://127.0.0.1:3030/14
TRACE 2022-10-19T13:38:23.461037: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461041: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461072: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461125: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461124: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.461246: proxy: Finished proxy of http://127.0.0.1:3030/42
 INFO 2022-10-19T13:38:23.461311: proxy: Finished proxy of http://127.0.0.1:3030/8
 INFO 2022-10-19T13:38:23.461332: proxy: Finished proxy of http://127.0.0.1:3030/43
 INFO 2022-10-19T13:38:23.461334: proxy: Finished proxy of http://127.0.0.1:3030/47
 INFO 2022-10-19T13:38:23.461368: proxy: Finished proxy of http://127.0.0.1:3030/34
 INFO 2022-10-19T13:38:23.461368: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T13:38:23.461391: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.461401: proxy: Finished proxy of http://127.0.0.1:3030/28
TRACE 2022-10-19T13:38:23.461456: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461477: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461481: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461511: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461511: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461548: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.460217: proxy: Finished proxy of http://127.0.0.1:3030/21
 INFO 2022-10-19T13:38:23.461636: proxy: Finished proxy of http://127.0.0.1:3030/23
 INFO 2022-10-19T13:38:23.461666: proxy: Finished proxy of http://127.0.0.1:3030/25
TRACE 2022-10-19T13:38:23.461712: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461782: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.461794: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.462254: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T13:38:23.462455: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:38:23.490559: proxy: Finished proxy of http://127.0.0.1:3030/31
 INFO 2022-10-19T13:38:23.490580: proxy: Finished proxy of http://127.0.0.1:3030/48
 INFO 2022-10-19T13:38:23.490559: proxy: Finished proxy of http://127.0.0.1:3030/32
 INFO 2022-10-19T13:38:23.490602: proxy: Finished proxy of http://127.0.0.1:3030/45
 INFO 2022-10-19T13:38:23.490559: proxy: Finished proxy of http://127.0.0.1:3030/46
 INFO 2022-10-19T13:38:23.490668: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T13:38:23.490767: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.490767: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.490767: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.490863: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.490767: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.490767: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.491747: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.491964: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.491992: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492063: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492067: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492093: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492104: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492163: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492165: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492181: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492209: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492244: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492245: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492259: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492289: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492322: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492331: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492349: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492366: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492407: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492419: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492438: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492444: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492500: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492502: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492516: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492521: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492581: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492581: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492595: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492643: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492662: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492664: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492673: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492722: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492739: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492743: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492749: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492799: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492831: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492836: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492843: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492889: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492907: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492914: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492917: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492964: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492981: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492990: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:38:23.492992: mio::poll: deregistering event source from poller
```

## Firing 50 queries at the proxy almost simultaneously

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/proxy`
TRACE 2022-10-19T13:45:07.654605: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:07.654776: proxy: Starting server on 127.0.0.1:3000
```

Starts registering source events at 15.069

```
TRACE 2022-10-19T13:45:15.068850: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069078: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069166: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069250: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069330: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069411: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069490: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069620: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069730: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069843: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.069956: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070115: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070208: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070318: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070401: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070527: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070601: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070734: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070809: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.070926: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071010: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071120: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071204: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071327: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071406: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071530: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071624: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071736: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071835: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.071933: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072040: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072135: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072240: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072337: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072433: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072530: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072631: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072727: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072832: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.072947: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073050: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073153: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073250: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073366: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073467: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073582: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073683: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073798: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.073896: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.074002: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

Starts first query fn at 15.076 - 7ms later.

```
 INFO 2022-10-19T13:45:15.076279: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T13:45:15.076590: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.076813: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T13:45:15.076873: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T13:45:15.076863: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
TRACE 2022-10-19T13:45:15.077037: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.076971: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T13:45:15.077008: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T13:45:15.077014: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T13:45:15.076893: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T13:45:15.077045: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
TRACE 2022-10-19T13:45:15.077419: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.077466: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.077501: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T13:45:15.077508: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.077464: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T13:45:15.077464: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
TRACE 2022-10-19T13:45:15.077658: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.077691: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T13:45:15.077804: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T13:45:15.077810: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
 INFO 2022-10-19T13:45:15.077753: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
 INFO 2022-10-19T13:45:15.077769: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
TRACE 2022-10-19T13:45:15.077919: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.077969: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.077782: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.078115: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T13:45:15.078215: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
TRACE 2022-10-19T13:45:15.078264: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.078298: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.078300: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.078300: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.078305: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.078349: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T13:45:15.078398: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
 INFO 2022-10-19T13:45:15.078625: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
TRACE 2022-10-19T13:45:15.078914: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.078952: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079103: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079401: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.079518: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T13:45:15.079518: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T13:45:15.079553: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T13:45:15.079646: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079690: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.079746: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
TRACE 2022-10-19T13:45:15.079748: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079752: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.079780: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T13:45:15.079815: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
TRACE 2022-10-19T13:45:15.079910: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079962: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.079987: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.080022: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.080161: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
TRACE 2022-10-19T13:45:15.080212: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.080342: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
TRACE 2022-10-19T13:45:15.080281: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.080414: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
 INFO 2022-10-19T13:45:15.080416: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
 INFO 2022-10-19T13:45:15.080477: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
 INFO 2022-10-19T13:45:15.080608: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T13:45:15.080626: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T13:45:15.080623: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
 INFO 2022-10-19T13:45:15.080632: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T13:45:15.080718: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T13:45:15.080734: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T13:45:15.080897: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.080899: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.080901: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.081080: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
TRACE 2022-10-19T13:45:15.081224: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081230: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081258: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081282: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081455: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081459: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.081574: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T13:45:15.081607: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
TRACE 2022-10-19T13:45:15.081796: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.081870: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.082043: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.082238: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T13:45:15.082252: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.082407: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.082844: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T13:45:15.082884: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T13:45:15.082886: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T13:45:15.083043: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.083106: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T13:45:15.083186: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T13:45:15.083179: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
 INFO 2022-10-19T13:45:15.083233: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T13:45:15.083278: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
```

First query finished at 15.084, 7ms later - double the time that it should take (3ms).

```
 INFO 2022-10-19T13:45:15.084343: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T13:45:15.084511: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.084726: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T13:45:15.084861: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.085462: proxy: Finished proxy of http://127.0.0.1:3030/3
TRACE 2022-10-19T13:45:15.085683: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.085693: mio::poll: registering event source with poller: token=Token(16777268), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.085694: mio::poll: registering event source with poller: token=Token(16777270), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.085931: proxy: Finished proxy of http://127.0.0.1:3030/14
 INFO 2022-10-19T13:45:15.086104: proxy: Finished proxy of http://127.0.0.1:3030/17
TRACE 2022-10-19T13:45:15.086114: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.086141: proxy: Finished proxy of http://127.0.0.1:3030/9
 INFO 2022-10-19T13:45:15.086151: proxy: Finished proxy of http://127.0.0.1:3030/2
 INFO 2022-10-19T13:45:15.086201: proxy: Finished proxy of http://127.0.0.1:3030/16
 INFO 2022-10-19T13:45:15.086205: proxy: Finished proxy of http://127.0.0.1:3030/13
 INFO 2022-10-19T13:45:15.086254: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T13:45:15.086257: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086289: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086291: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086346: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086344: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.086346: proxy: Finished proxy of http://127.0.0.1:3030/8
TRACE 2022-10-19T13:45:15.086402: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086493: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.086502: proxy: Finished proxy of http://127.0.0.1:3030/7
 INFO 2022-10-19T13:45:15.086506: proxy: Finished proxy of http://127.0.0.1:3030/15
 INFO 2022-10-19T13:45:15.086511: proxy: Finished proxy of http://127.0.0.1:3030/12
 INFO 2022-10-19T13:45:15.086531: proxy: Finished proxy of http://127.0.0.1:3030/10
 INFO 2022-10-19T13:45:15.086550: proxy: Finished proxy of http://127.0.0.1:3030/28
TRACE 2022-10-19T13:45:15.086647: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086650: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086656: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086668: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.086696: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.086736: proxy: Finished proxy of http://127.0.0.1:3030/19
 INFO 2022-10-19T13:45:15.086862: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T13:45:15.086897: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.087026: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.087524: proxy: Finished proxy of http://127.0.0.1:3030/22
 INFO 2022-10-19T13:45:15.087651: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T13:45:15.087703: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.087805: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.087980: proxy: Finished proxy of http://127.0.0.1:3030/32
 INFO 2022-10-19T13:45:15.088014: proxy: Finished proxy of http://127.0.0.1:3030/41
 INFO 2022-10-19T13:45:15.088036: proxy: Finished proxy of http://127.0.0.1:3030/23
 INFO 2022-10-19T13:45:15.088055: proxy: Finished proxy of http://127.0.0.1:3030/20
 INFO 2022-10-19T13:45:15.088080: proxy: Finished proxy of http://127.0.0.1:3030/31
 INFO 2022-10-19T13:45:15.088095: proxy: Finished proxy of http://127.0.0.1:3030/30
 INFO 2022-10-19T13:45:15.088108: proxy: Finished proxy of http://127.0.0.1:3030/26
TRACE 2022-10-19T13:45:15.088134: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088161: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088199: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088229: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088250: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088254: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.088347: proxy: Finished proxy of http://127.0.0.1:3030/25
 INFO 2022-10-19T13:45:15.088396: proxy: Finished proxy of http://127.0.0.1:3030/11
 INFO 2022-10-19T13:45:15.088477: proxy: Finished proxy of http://127.0.0.1:3030/6
 INFO 2022-10-19T13:45:15.088475: proxy: Finished proxy of http://127.0.0.1:3030/36
TRACE 2022-10-19T13:45:15.088176: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088507: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088544: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088601: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.088627: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.089214: proxy: Finished proxy of http://127.0.0.1:3030/27
 INFO 2022-10-19T13:45:15.089214: proxy: Finished proxy of http://127.0.0.1:3030/37
TRACE 2022-10-19T13:45:15.089368: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.089368: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.089657: proxy: Finished proxy of http://127.0.0.1:3030/29
TRACE 2022-10-19T13:45:15.089815: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.091428: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T13:45:15.091642: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.092204: proxy: Finished proxy of http://127.0.0.1:3030/42
 INFO 2022-10-19T13:45:15.092205: proxy: Finished proxy of http://127.0.0.1:3030/49
 INFO 2022-10-19T13:45:15.092240: proxy: Finished proxy of http://127.0.0.1:3030/43
 INFO 2022-10-19T13:45:15.092246: proxy: Finished proxy of http://127.0.0.1:3030/34
 INFO 2022-10-19T13:45:15.092262: proxy: Finished proxy of http://127.0.0.1:3030/45
 INFO 2022-10-19T13:45:15.092281: proxy: Finished proxy of http://127.0.0.1:3030/18
TRACE 2022-10-19T13:45:15.092379: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092385: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092397: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092402: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092403: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092428: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.092545: proxy: Finished proxy of http://127.0.0.1:3030/40
 INFO 2022-10-19T13:45:15.092611: proxy: Finished proxy of http://127.0.0.1:3030/44
 INFO 2022-10-19T13:45:15.092615: proxy: Finished proxy of http://127.0.0.1:3030/38
 INFO 2022-10-19T13:45:15.092628: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T13:45:15.092717: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092768: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092769: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.092778: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:45:15.092870: proxy: Finished proxy of http://127.0.0.1:3030/39
TRACE 2022-10-19T13:45:15.093018: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.134226: mio::poll: registering event source with poller: token=Token(16777300), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.134226: mio::poll: registering event source with poller: token=Token(16777308), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.134232: mio::poll: registering event source with poller: token=Token(16777306), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:45:15.134382: mio::poll: registering event source with poller: token=Token(16777310), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:45:15.139866: proxy: Finished proxy of http://127.0.0.1:3030/48
 INFO 2022-10-19T13:45:15.139867: proxy: Finished proxy of http://127.0.0.1:3030/50
 INFO 2022-10-19T13:45:15.139867: proxy: Finished proxy of http://127.0.0.1:3030/46
 INFO 2022-10-19T13:45:15.139866: proxy: Finished proxy of http://127.0.0.1:3030/47
```

Last query completed at 15.140, almost 70ms after the first query began, matching the `fire` output of 72ms. First query finished at 15.084, 7ms later - double the time that it should take (3ms).

```
TRACE 2022-10-19T13:45:15.140069: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.140070: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.140070: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.140070: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.140870: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142322: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142322: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142371: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142330: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142321: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142358: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142393: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142482: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142481: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142482: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142501: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142482: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142497: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142495: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142552: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142588: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142605: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142613: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142637: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142636: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142650: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142650: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142696: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142699: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142709: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142709: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142724: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142731: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142743: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142753: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142753: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142804: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142801: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142813: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142813: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142825: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142825: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142843: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142848: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142857: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142857: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142899: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142908: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142916: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142918: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142922: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142918: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142591: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:45:15.142957: mio::poll: deregistering event source from poller
```

## Firing 50 queries at the proxy with a 5ms gap between each

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/proxy`
TRACE 2022-10-19T13:59:41.791677: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:41.791828: proxy: Starting server on 127.0.0.1:3000
TRACE 2022-10-19T13:59:50.776492: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.776924: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.777191: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.777643: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.778047: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.778681: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.778962: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.779457: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.779733: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.780228: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.780650: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.781038: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.781475: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.781886: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.782361: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.782636: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.783198: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.783608: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.784015: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.784417: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.784876: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.785285: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.785655: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.786052: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.786487: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.786881: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.787271: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.787671: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.788070: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.788545: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.788942: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.789396: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.789836: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.790376: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.790669: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.791067: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.791404: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.791897: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.792443: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.792776: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.793278: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.793714: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.794214: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.794522: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.794973: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.795402: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.795797: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.796117: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.796616: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.796939: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First query begins at 50.804

```
 INFO 2022-10-19T13:59:50.803942: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T13:59:50.804607: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.805115: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
 INFO 2022-10-19T13:59:50.805238: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T13:59:50.805334: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T13:59:50.805469: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T13:59:50.805445: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T13:59:50.805705: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.806739: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.806809: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T13:59:50.806708: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T13:59:50.807046: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T13:59:50.807163: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.805545: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T13:59:50.805356: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T13:59:50.805616: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
TRACE 2022-10-19T13:59:50.807358: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.807401: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.806184: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.807465: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T13:59:50.807328: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
TRACE 2022-10-19T13:59:50.806090: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.807742: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T13:59:50.807748: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T13:59:50.806180: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T13:59:50.807748: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
TRACE 2022-10-19T13:59:50.807865: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.807903: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.807924: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.807929: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.808147: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.808201: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
TRACE 2022-10-19T13:59:50.807875: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.808229: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.808253: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.808289: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T13:59:50.808522: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
TRACE 2022-10-19T13:59:50.808547: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.808644: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.806220: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.808808: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T13:59:50.806180: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T13:59:50.810082: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T13:59:50.810103: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T13:59:50.810143: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.810352: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T13:59:50.810441: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T13:59:50.810782: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
TRACE 2022-10-19T13:59:50.810903: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.810908: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.810945: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.810912: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.811126: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.811278: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
 INFO 2022-10-19T13:59:50.811310: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
 INFO 2022-10-19T13:59:50.811363: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T13:59:50.810935: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.811549: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
TRACE 2022-10-19T13:59:50.811605: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.811708: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.811703: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.811735: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T13:59:50.811846: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T13:59:50.811848: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.811913: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.811952: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T13:59:50.811960: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
TRACE 2022-10-19T13:59:50.812014: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.812098: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T13:59:50.812116: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.812145: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T13:59:50.812246: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
TRACE 2022-10-19T13:59:50.812320: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.812320: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
 INFO 2022-10-19T13:59:50.812352: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T13:59:50.812379: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.812415: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.812518: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.812601: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.812684: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.812762: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
TRACE 2022-10-19T13:59:50.812892: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.812922: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T13:59:50.812934: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
TRACE 2022-10-19T13:59:50.813012: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.813042: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.813271: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
TRACE 2022-10-19T13:59:50.813521: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.813563: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.813796: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T13:59:50.814048: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T13:59:50.814136: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
 INFO 2022-10-19T13:59:50.814026: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
```

First query finishes at 50.814, 10ms later.`

```
 INFO 2022-10-19T13:59:50.814804: proxy: Finished proxy of http://127.0.0.1:3030/10
 INFO 2022-10-19T13:59:50.815025: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T13:59:50.815144: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.815442: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T13:59:50.815683: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.815775: proxy: Finished proxy of http://127.0.0.1:3030/3
 INFO 2022-10-19T13:59:50.815802: proxy: Finished proxy of http://127.0.0.1:3030/2
TRACE 2022-10-19T13:59:50.816010: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.816027: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.816273: proxy: Finished proxy of http://127.0.0.1:3030/11
TRACE 2022-10-19T13:59:50.816532: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.817061: proxy: Finished proxy of http://127.0.0.1:3030/14
 INFO 2022-10-19T13:59:50.817071: proxy: Finished proxy of http://127.0.0.1:3030/29
 INFO 2022-10-19T13:59:50.817160: proxy: Finished proxy of http://127.0.0.1:3030/22
 INFO 2022-10-19T13:59:50.817212: proxy: Finished proxy of http://127.0.0.1:3030/4
 INFO 2022-10-19T13:59:50.817273: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T13:59:50.817302: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.817380: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.817499: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.817427: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.817880: proxy: Finished proxy of http://127.0.0.1:3030/12
TRACE 2022-10-19T13:59:50.818179: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.818241: proxy: Finished proxy of http://127.0.0.1:3030/23
 INFO 2022-10-19T13:59:50.818344: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T13:59:50.818485: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.818508: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T13:59:50.818589: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.818645: proxy: Finished proxy of http://127.0.0.1:3030/15
TRACE 2022-10-19T13:59:50.818730: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.818749: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T13:59:50.818855: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.818953: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T13:59:50.818962: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.819086: proxy: Finished proxy of http://127.0.0.1:3030/13
TRACE 2022-10-19T13:59:50.819132: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.819135: proxy: Finished proxy of http://127.0.0.1:3030/18
TRACE 2022-10-19T13:59:50.819167: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.819249: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.819300: proxy: Finished proxy of http://127.0.0.1:3030/9
TRACE 2022-10-19T13:59:50.819327: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.819497: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.819451: proxy: Finished proxy of http://127.0.0.1:3030/19
 INFO 2022-10-19T13:59:50.819772: proxy: Finished proxy of http://127.0.0.1:3030/6
 INFO 2022-10-19T13:59:50.819778: proxy: Finished proxy of http://127.0.0.1:3030/40
TRACE 2022-10-19T13:59:50.819797: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.819934: proxy: Finished proxy of http://127.0.0.1:3030/47
 INFO 2022-10-19T13:59:50.819967: proxy: Finished proxy of http://127.0.0.1:3030/8
TRACE 2022-10-19T13:59:50.819976: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.819981: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.820000: proxy: Finished proxy of http://127.0.0.1:3030/17
 INFO 2022-10-19T13:59:50.820140: proxy: Finished proxy of http://127.0.0.1:3030/28
TRACE 2022-10-19T13:59:50.820140: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.820164: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.820198: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.820352: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.820914: proxy: Finished proxy of http://127.0.0.1:3030/39
 INFO 2022-10-19T13:59:50.820913: proxy: Finished proxy of http://127.0.0.1:3030/32
 INFO 2022-10-19T13:59:50.821064: proxy: Finished proxy of http://127.0.0.1:3030/50
TRACE 2022-10-19T13:59:50.821109: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.821115: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821116: proxy: Finished proxy of http://127.0.0.1:3030/26
 INFO 2022-10-19T13:59:50.821118: proxy: Finished proxy of http://127.0.0.1:3030/45
 INFO 2022-10-19T13:59:50.821169: proxy: Finished proxy of http://127.0.0.1:3030/43
TRACE 2022-10-19T13:59:50.821263: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.821319: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.821376: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.821419: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821420: proxy: Finished proxy of http://127.0.0.1:3030/41
 INFO 2022-10-19T13:59:50.821459: proxy: Finished proxy of http://127.0.0.1:3030/44
TRACE 2022-10-19T13:59:50.821617: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821628: proxy: Finished proxy of http://127.0.0.1:3030/30
 INFO 2022-10-19T13:59:50.821646: proxy: Finished proxy of http://127.0.0.1:3030/27
TRACE 2022-10-19T13:59:50.821654: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821668: proxy: Finished proxy of http://127.0.0.1:3030/31
 INFO 2022-10-19T13:59:50.821723: proxy: Finished proxy of http://127.0.0.1:3030/25
TRACE 2022-10-19T13:59:50.821835: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821852: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T13:59:50.821843: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.821864: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.821909: proxy: Finished proxy of http://127.0.0.1:3030/48
TRACE 2022-10-19T13:59:50.821916: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.822007: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.822107: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.822167: proxy: Finished proxy of http://127.0.0.1:3030/20
 INFO 2022-10-19T13:59:50.822176: proxy: Finished proxy of http://127.0.0.1:3030/42
TRACE 2022-10-19T13:59:50.822374: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.822378: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.823165: proxy: Finished proxy of http://127.0.0.1:3030/46
TRACE 2022-10-19T13:59:50.823367: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.850352: mio::poll: registering event source with poller: token=Token(16777299), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.850395: mio::poll: registering event source with poller: token=Token(16777293), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.850355: mio::poll: registering event source with poller: token=Token(16777300), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.850355: mio::poll: registering event source with poller: token=Token(16777309), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.850379: mio::poll: registering event source with poller: token=Token(16777310), interests=READABLE | WRITABLE
TRACE 2022-10-19T13:59:50.850352: mio::poll: registering event source with poller: token=Token(16777291), interests=READABLE | WRITABLE
 INFO 2022-10-19T13:59:50.855509: proxy: Finished proxy of http://127.0.0.1:3030/34
 INFO 2022-10-19T13:59:50.855511: proxy: Finished proxy of http://127.0.0.1:3030/37
 INFO 2022-10-19T13:59:50.855509: proxy: Finished proxy of http://127.0.0.1:3030/38
TRACE 2022-10-19T13:59:50.855726: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.855726: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.855735: mio::poll: deregistering event source from poller
 INFO 2022-10-19T13:59:50.855856: proxy: Finished proxy of http://127.0.0.1:3030/36
 INFO 2022-10-19T13:59:50.855856: proxy: Finished proxy of http://127.0.0.1:3030/49
 INFO 2022-10-19T13:59:50.855907: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T13:59:50.856031: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856031: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856080: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856610: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856626: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856728: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856819: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856836: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856870: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856891: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856911: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856928: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856929: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856941: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856939: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856972: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.856982: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857008: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857018: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857036: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857040: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857040: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857068: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857067: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857077: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857086: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857102: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857107: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857123: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857143: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857143: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857169: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857174: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857179: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857187: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857194: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857209: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857224: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857272: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857282: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857283: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857290: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857287: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857315: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857325: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857144: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857373: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857387: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857392: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857383: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857383: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857409: mio::poll: deregistering event source from poller
TRACE 2022-10-19T13:59:50.857287: mio::poll: deregistering event source from poller
```

```
   Compiling tokio-hyper-test v0.1.0 (/home/michael/dev/vmware/tokio-hyper-test)
    Finished dev [unoptimized + debuginfo] target(s) in 2.36s
     Running `target/debug/proxy`
TRACE 2022-10-19T14:25:50.499271: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:50.499438: proxy: Starting server on 127.0.0.1:3000
```

Starts polling at 55.623

```
TRACE 2022-10-19T14:25:55.623686: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.623899: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.623980: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624066: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624143: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624214: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624281: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624391: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624500: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624621: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624738: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624845: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.624955: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625063: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625185: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625295: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625407: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625521: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625663: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625797: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.625914: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626028: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626229: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626337: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626457: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626578: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626696: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626812: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.626930: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627049: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627169: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627285: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627406: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627527: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627651: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627764: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.627886: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628000: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628115: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628229: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628343: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628460: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628589: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628666: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628791: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.628909: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.629018: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.629188: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.629250: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.629369: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First query begins proxy at 55.631 - 8ms later

```
 INFO 2022-10-19T14:25:55.631504: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
 INFO 2022-10-19T14:25:55.641661: proxy: Blocked with CPU for 10ms for GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T14:25:55.642081: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.642534: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T14:25:55.642534: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T14:25:55.642593: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T14:25:55.642628: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T14:25:55.642626: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T14:25:55.642639: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T14:25:55.642639: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T14:25:55.642759: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T14:25:55.643728: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T14:25:55.645189: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
 INFO 2022-10-19T14:25:55.649185: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T14:25:55.649185: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T14:25:55.652716: proxy: Blocked with CPU for 10ms for GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T14:25:55.652753: proxy: Blocked with CPU for 10ms for GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T14:25:55.652715: proxy: Blocked with CPU for 10ms for GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T14:25:55.652766: proxy: Blocked with CPU for 10ms for GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T14:25:55.652778: proxy: Blocked with CPU for 10ms for GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T14:25:55.652778: proxy: Blocked with CPU for 10ms for GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T14:25:55.652813: proxy: Blocked with CPU for 10ms for GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T14:25:55.652730: proxy: Blocked with CPU for 10ms for GET /7 -> http://127.0.0.1:3030/7
TRACE 2022-10-19T14:25:55.653528: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.653703: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.653739: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.653774: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.653887: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T14:25:55.653902: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T14:25:55.653917: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T14:25:55.654074: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T14:25:55.653533: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.654423: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
TRACE 2022-10-19T14:25:55.653524: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.654939: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T14:25:55.655331: proxy: Blocked with CPU for 10ms for GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T14:25:55.655843: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.656030: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T14:25:55.658096: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.658276: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T14:25:55.659335: proxy: Blocked with CPU for 10ms for GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T14:25:55.659397: proxy: Blocked with CPU for 10ms for GET /2 -> http://127.0.0.1:3030/2
TRACE 2022-10-19T14:25:55.659681: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.659681: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.659880: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
 INFO 2022-10-19T14:25:55.659880: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T14:25:55.661535: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.661752: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T14:25:55.663965: proxy: Blocked with CPU for 10ms for GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T14:25:55.663971: proxy: Blocked with CPU for 10ms for GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T14:25:55.664198: proxy: Blocked with CPU for 10ms for GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T14:25:55.664291: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.664383: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.664492: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T14:25:55.664520: proxy: Blocked with CPU for 10ms for GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T14:25:55.664608: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T14:25:55.664512: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.664754: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.664913: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T14:25:55.664920: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
 INFO 2022-10-19T14:25:55.659335: proxy: Blocked with CPU for 10ms for GET /3 -> http://127.0.0.1:3030/3
TRACE 2022-10-19T14:25:55.665771: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.665935: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
 INFO 2022-10-19T14:25:55.666106: proxy: Blocked with CPU for 10ms for GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T14:25:55.666504: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.666659: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T14:25:55.669975: proxy: Blocked with CPU for 10ms for GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T14:25:55.670338: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.670572: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T14:25:55.670135: proxy: Blocked with CPU for 10ms for GET /44 -> http://127.0.0.1:3030/44
TRACE 2022-10-19T14:25:55.671063: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.671328: proxy: Blocked with CPU for 10ms for GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T14:25:55.671525: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.671575: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T14:25:55.671678: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T14:25:55.671822: proxy: Blocked with CPU for 10ms for GET /11 -> http://127.0.0.1:3030/11
TRACE 2022-10-19T14:25:55.672016: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.672176: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T14:25:55.674553: proxy: Blocked with CPU for 10ms for GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T14:25:55.674692: proxy: Blocked with CPU for 10ms for GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T14:25:55.674744: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.674896: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
TRACE 2022-10-19T14:25:55.674912: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.675045: proxy: Blocked with CPU for 10ms for GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T14:25:55.675105: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
 INFO 2022-10-19T14:25:55.675088: proxy: Blocked with CPU for 10ms for GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T14:25:55.675223: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.675389: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
TRACE 2022-10-19T14:25:55.675393: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.675580: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T14:25:55.676005: proxy: Blocked with CPU for 10ms for GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T14:25:55.676188: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.676332: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
 INFO 2022-10-19T14:25:55.676736: proxy: Blocked with CPU for 10ms for GET /35 -> http://127.0.0.1:3030/35
TRACE 2022-10-19T14:25:55.676992: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.677168: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T14:25:55.678075: proxy: Blocked with CPU for 10ms for GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T14:25:55.678009: proxy: Blocked with CPU for 10ms for GET /48 -> http://127.0.0.1:3030/48
TRACE 2022-10-19T14:25:55.678261: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.678270: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.678407: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
 INFO 2022-10-19T14:25:55.678419: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
 INFO 2022-10-19T14:25:55.680656: proxy: Blocked with CPU for 10ms for GET /32 -> http://127.0.0.1:3030/32
TRACE 2022-10-19T14:25:55.680870: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.681040: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T14:25:55.681651: proxy: Blocked with CPU for 10ms for GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T14:25:55.681735: proxy: Blocked with CPU for 10ms for GET /19 -> http://127.0.0.1:3030/19
TRACE 2022-10-19T14:25:55.682013: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.682186: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.682219: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T14:25:55.682360: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
 INFO 2022-10-19T14:25:55.685185: proxy: Blocked with CPU for 10ms for GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T14:25:55.685425: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.685447: proxy: Blocked with CPU for 10ms for GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T14:25:55.685610: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
TRACE 2022-10-19T14:25:55.685651: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.685642: proxy: Blocked with CPU for 10ms for GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T14:25:55.685820: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
TRACE 2022-10-19T14:25:55.685953: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.686165: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T14:25:55.686409: proxy: Blocked with CPU for 10ms for GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T14:25:55.686591: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.686742: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T14:25:55.687231: proxy: Blocked with CPU for 10ms for GET /38 -> http://127.0.0.1:3030/38
TRACE 2022-10-19T14:25:55.687648: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.687848: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T14:25:55.688276: proxy: Blocked with CPU for 10ms for GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T14:25:55.688466: proxy: Blocked with CPU for 10ms for GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T14:25:55.688582: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.688711: proxy: Blocked with CPU for 10ms for GET /23 -> http://127.0.0.1:3030/23
TRACE 2022-10-19T14:25:55.688884: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.688651: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.688663: proxy: Blocked with CPU for 10ms for GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T14:25:55.689585: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
```

First query finished proxy at 55.690 - 60ms later, *only after* all other queries have begun.

```
 INFO 2022-10-19T14:25:55.690276: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T14:25:55.690827: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.691132: proxy: Blocked with CPU for 10ms for GET /36 -> http://127.0.0.1:3030/36
TRACE 2022-10-19T14:25:55.691895: mio::poll: registering event source with poller: token=Token(16777268), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.692314: proxy: Blocked with CPU for 10ms for GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T14:25:55.692476: proxy: Blocked with CPU for 10ms for GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T14:25:55.692575: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.692670: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.695687: proxy: Blocked with CPU for 10ms for GET /37 -> http://127.0.0.1:3030/37
 INFO 2022-10-19T14:25:55.695893: proxy: Blocked with CPU for 10ms for GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T14:25:55.696262: proxy: Blocked with CPU for 10ms for GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T14:25:55.696799: proxy: Blocked with CPU for 10ms for GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T14:25:55.698226: proxy: Blocked with CPU for 10ms for GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T14:25:55.721851: proxy: Finished proxy of http://127.0.0.1:3030/18
 INFO 2022-10-19T14:25:55.721851: proxy: Finished proxy of http://127.0.0.1:3030/14
 INFO 2022-10-19T14:25:55.721851: proxy: Finished proxy of http://127.0.0.1:3030/44
 INFO 2022-10-19T14:25:55.722069: proxy: Finished proxy of http://127.0.0.1:3030/9
TRACE 2022-10-19T14:25:55.722079: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.722072: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.722078: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722121: proxy: Finished proxy of http://127.0.0.1:3030/40
 INFO 2022-10-19T14:25:55.722156: proxy: Finished proxy of http://127.0.0.1:3030/10
TRACE 2022-10-19T14:25:55.722234: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.722374: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.722273: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722458: proxy: Finished proxy of http://127.0.0.1:3030/50
 INFO 2022-10-19T14:25:55.722536: proxy: Finished proxy of http://127.0.0.1:3030/11
TRACE 2022-10-19T14:25:55.722622: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722668: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T14:25:55.722710: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722750: proxy: Finished proxy of http://127.0.0.1:3030/48
TRACE 2022-10-19T14:25:55.722820: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722870: proxy: Finished proxy of http://127.0.0.1:3030/5
 INFO 2022-10-19T14:25:55.722889: proxy: Finished proxy of http://127.0.0.1:3030/43
TRACE 2022-10-19T14:25:55.722913: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.722970: proxy: Finished proxy of http://127.0.0.1:3030/28
 INFO 2022-10-19T14:25:55.722983: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T14:25:55.723024: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.723054: proxy: Finished proxy of http://127.0.0.1:3030/34
TRACE 2022-10-19T14:25:55.723104: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.723140: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.723145: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.723181: proxy: Finished proxy of http://127.0.0.1:3030/32
TRACE 2022-10-19T14:25:55.723233: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.721851: proxy: Finished proxy of http://127.0.0.1:3030/13
TRACE 2022-10-19T14:25:55.723345: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.723484: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.723649: proxy: Finished proxy of http://127.0.0.1:3030/6
 INFO 2022-10-19T14:25:55.723793: proxy: Finished proxy of http://127.0.0.1:3030/26
 INFO 2022-10-19T14:25:55.723801: proxy: Finished proxy of http://127.0.0.1:3030/31
TRACE 2022-10-19T14:25:55.723807: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.723920: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.723967: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.724042: proxy: Finished proxy of http://127.0.0.1:3030/25
 INFO 2022-10-19T14:25:55.724093: proxy: Finished proxy of http://127.0.0.1:3030/20
TRACE 2022-10-19T14:25:55.724199: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.724216: proxy: Finished proxy of http://127.0.0.1:3030/3
TRACE 2022-10-19T14:25:55.724240: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.724381: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.724436: proxy: Finished proxy of http://127.0.0.1:3030/49
 INFO 2022-10-19T14:25:55.724477: proxy: Finished proxy of http://127.0.0.1:3030/12
TRACE 2022-10-19T14:25:55.724590: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.724605: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.724615: proxy: Finished proxy of http://127.0.0.1:3030/19
 INFO 2022-10-19T14:25:55.724786: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T14:25:55.724785: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.724949: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.726590: proxy: Finished proxy of http://127.0.0.1:3030/42
TRACE 2022-10-19T14:25:55.726805: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.726902: proxy: Finished proxy of http://127.0.0.1:3030/17
 INFO 2022-10-19T14:25:55.727034: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T14:25:55.727054: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.727185: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.727274: proxy: Finished proxy of http://127.0.0.1:3030/2
 INFO 2022-10-19T14:25:55.727397: proxy: Finished proxy of http://127.0.0.1:3030/45
TRACE 2022-10-19T14:25:55.727419: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.727544: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.727625: proxy: Finished proxy of http://127.0.0.1:3030/30
 INFO 2022-10-19T14:25:55.727753: proxy: Finished proxy of http://127.0.0.1:3030/41
TRACE 2022-10-19T14:25:55.727765: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.727898: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.727967: proxy: Finished proxy of http://127.0.0.1:3030/46
TRACE 2022-10-19T14:25:55.728115: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.728117: proxy: Finished proxy of http://127.0.0.1:3030/47
TRACE 2022-10-19T14:25:55.728268: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.728327: proxy: Finished proxy of http://127.0.0.1:3030/38
 INFO 2022-10-19T14:25:55.728352: proxy: Finished proxy of http://127.0.0.1:3030/23
 INFO 2022-10-19T14:25:55.728354: proxy: Finished proxy of http://127.0.0.1:3030/39
TRACE 2022-10-19T14:25:55.728474: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.728483: proxy: Finished proxy of http://127.0.0.1:3030/36
TRACE 2022-10-19T14:25:55.728505: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.728509: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.728628: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.728693: proxy: Finished proxy of http://127.0.0.1:3030/8
 INFO 2022-10-19T14:25:55.728722: proxy: Finished proxy of http://127.0.0.1:3030/33
 INFO 2022-10-19T14:25:55.728729: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T14:25:55.728842: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.728881: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.728913: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.729105: proxy: Finished proxy of http://127.0.0.1:3030/27
TRACE 2022-10-19T14:25:55.729254: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.730195: mio::poll: registering event source with poller: token=Token(16777311), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.730195: mio::poll: registering event source with poller: token=Token(16777279), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.730195: mio::poll: registering event source with poller: token=Token(16777309), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.730244: mio::poll: registering event source with poller: token=Token(16777276), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:25:55.730781: mio::poll: registering event source with poller: token=Token(33554484), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:25:55.762576: proxy: Finished proxy of http://127.0.0.1:3030/29
 INFO 2022-10-19T14:25:55.762595: proxy: Finished proxy of http://127.0.0.1:3030/22
 INFO 2022-10-19T14:25:55.762575: proxy: Finished proxy of http://127.0.0.1:3030/37
 INFO 2022-10-19T14:25:55.762647: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T14:25:55.762817: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.762816: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.762816: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.762819: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:25:55.763160: proxy: Finished proxy of http://127.0.0.1:3030/15
```

Last query finished proxy at 55.763, 70ms after the first completed and 130ms after the first began.

```
TRACE 2022-10-19T14:25:55.763362: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.763762: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.763947: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764034: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764055: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764140: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764171: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764231: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764282: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764313: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764392: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764407: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764490: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764506: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764576: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764620: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764660: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764747: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764741: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764833: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764850: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764920: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.764961: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765060: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765077: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765147: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765190: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765239: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765304: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765327: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765412: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765416: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765498: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765538: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765584: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765648: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765668: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765755: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765754: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765841: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765859: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765936: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.765966: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766030: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766093: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766145: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766161: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766186: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766205: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766208: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:25:55.766256: mio::poll: deregistering event source from poller
```

## Test with 100ms CPU before proxy request

```
   Compiling tokio-hyper-test v0.1.0 (/home/michael/dev/vmware/tokio-hyper-test)
    Finished dev [unoptimized + debuginfo] target(s) in 2.69s
     Running `target/debug/proxy`
TRACE 2022-10-19T14:40:52.780788: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:52.780951: proxy: Starting server on 127.0.0.1:3000
TRACE 2022-10-19T14:40:56.440302: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440512: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440597: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440691: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440773: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440854: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.440933: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441014: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441091: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441177: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441260: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441439: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441523: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441603: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441845: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.441976: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442254: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442335: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442407: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442486: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442567: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442691: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442766: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.442876: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443022: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443108: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443320: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443430: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443551: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443686: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443835: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.443915: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444061: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444175: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444304: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444427: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444521: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444660: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444804: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444885: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.444980: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445100: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445221: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445332: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445410: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445534: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445650: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445728: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445866: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.445983: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First query at 56.447

```
 INFO 2022-10-19T14:40:56.446898: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T14:40:56.547106: proxy: Blocked with CPU for 100ms for GET /14 -> http://127.0.0.1:3030/14
TRACE 2022-10-19T14:40:56.547757: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.548272: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T14:40:56.548280: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T14:40:56.548272: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T14:40:56.548396: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T14:40:56.548427: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T14:40:56.548650: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T14:40:56.548638: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
 INFO 2022-10-19T14:40:56.548660: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T14:40:56.548659: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T14:40:56.549014: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T14:40:56.548296: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T14:40:56.549969: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
 INFO 2022-10-19T14:40:56.648481: proxy: Blocked with CPU for 100ms for GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T14:40:56.648574: proxy: Blocked with CPU for 100ms for GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T14:40:56.648708: proxy: Blocked with CPU for 100ms for GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T14:40:56.648768: proxy: Blocked with CPU for 100ms for GET /1 -> http://127.0.0.1:3030/1
 INFO 2022-10-19T14:40:56.648806: proxy: Blocked with CPU for 100ms for GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T14:40:56.648530: proxy: Blocked with CPU for 100ms for GET /3 -> http://127.0.0.1:3030/3
TRACE 2022-10-19T14:40:56.649043: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.649079: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.649093: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.649147: proxy: Blocked with CPU for 100ms for GET /4 -> http://127.0.0.1:3030/4
TRACE 2022-10-19T14:40:56.649105: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.649197: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.649163: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.648809: proxy: Blocked with CPU for 100ms for GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T14:40:56.649306: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T14:40:56.648481: proxy: Blocked with CPU for 100ms for GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T14:40:56.648481: proxy: Blocked with CPU for 100ms for GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T14:40:56.649306: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
 INFO 2022-10-19T14:40:56.649958: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T14:40:56.650102: proxy: Blocked with CPU for 100ms for GET /2 -> http://127.0.0.1:3030/2
TRACE 2022-10-19T14:40:56.650257: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.650275: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.650457: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T14:40:56.649958: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T14:40:56.650639: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T14:40:56.650695: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.650938: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
 INFO 2022-10-19T14:40:56.652058: proxy: Blocked with CPU for 100ms for GET /6 -> http://127.0.0.1:3030/6
 INFO 2022-10-19T14:40:56.650137: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
TRACE 2022-10-19T14:40:56.652365: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.652551: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T14:40:56.650962: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.653028: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T14:40:56.649965: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
TRACE 2022-10-19T14:40:56.666503: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.666828: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T14:40:56.749446: proxy: Blocked with CPU for 100ms for GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T14:40:56.749480: proxy: Blocked with CPU for 100ms for GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T14:40:56.752057: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.752275: proxy: Blocked with CPU for 100ms for GET /45 -> http://127.0.0.1:3030/45
 INFO 2022-10-19T14:40:56.752610: proxy: Blocked with CPU for 100ms for GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T14:40:56.752956: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.753107: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.753166: proxy: Blocked with CPU for 100ms for GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T14:40:56.751046: proxy: Blocked with CPU for 100ms for GET /42 -> http://127.0.0.1:3030/42
 INFO 2022-10-19T14:40:56.753362: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T14:40:56.753688: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.753889: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
TRACE 2022-10-19T14:40:56.753934: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.754147: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T14:40:56.752182: proxy: Blocked with CPU for 100ms for GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T14:40:56.751617: proxy: Blocked with CPU for 100ms for GET /21 -> http://127.0.0.1:3030/21
TRACE 2022-10-19T14:40:56.754500: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.754750: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T14:40:56.755595: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.755830: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T14:40:56.750113: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.756155: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T14:40:56.750069: proxy: Blocked with CPU for 100ms for GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T14:40:56.753299: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
 INFO 2022-10-19T14:40:56.750541: proxy: Blocked with CPU for 100ms for GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T14:40:56.753195: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T14:40:56.760999: proxy: Blocked with CPU for 100ms for GET /28 -> http://127.0.0.1:3030/28
TRACE 2022-10-19T14:40:56.761055: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.761056: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.761368: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
 INFO 2022-10-19T14:40:56.761369: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T14:40:56.766928: proxy: Blocked with CPU for 100ms for GET /24 -> http://127.0.0.1:3030/24
TRACE 2022-10-19T14:40:56.769578: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.769874: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
TRACE 2022-10-19T14:40:56.770688: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.770971: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T14:40:56.853447: proxy: Blocked with CPU for 100ms for GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T14:40:56.853843: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.853955: proxy: Blocked with CPU for 100ms for GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T14:40:56.854096: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
TRACE 2022-10-19T14:40:56.854215: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.854430: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T14:40:56.854847: proxy: Blocked with CPU for 100ms for GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T14:40:56.855084: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.855265: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
 INFO 2022-10-19T14:40:56.855701: proxy: Blocked with CPU for 100ms for GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T14:40:56.855892: proxy: Blocked with CPU for 100ms for GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T14:40:56.856124: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.856178: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.856238: proxy: Blocked with CPU for 100ms for GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T14:40:56.856378: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T14:40:56.856478: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T14:40:56.860928: proxy: Blocked with CPU for 100ms for GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T14:40:56.861512: proxy: Blocked with CPU for 100ms for GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T14:40:56.861697: proxy: Blocked with CPU for 100ms for GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T14:40:56.861924: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.861947: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.862151: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T14:40:56.862201: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.862466: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
TRACE 2022-10-19T14:40:56.862808: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.862976: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T14:40:56.865408: proxy: Blocked with CPU for 100ms for GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T14:40:56.865694: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.862732: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T14:40:56.865899: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T14:40:56.871087: proxy: Blocked with CPU for 100ms for GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T14:40:56.871397: proxy: Blocked with CPU for 100ms for GET /36 -> http://127.0.0.1:3030/36
TRACE 2022-10-19T14:40:56.871484: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.871731: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T14:40:56.872027: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.872224: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T14:40:56.954195: proxy: Blocked with CPU for 100ms for GET /37 -> http://127.0.0.1:3030/37
 INFO 2022-10-19T14:40:56.954535: proxy: Blocked with CPU for 100ms for GET /15 -> http://127.0.0.1:3030/15
TRACE 2022-10-19T14:40:56.954796: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.955002: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.955034: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T14:40:56.955344: proxy: Blocked with CPU for 100ms for GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T14:40:56.955565: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
```

First query returned at 56.956 - 509ms later, *only after* all queries have begun.

```
 INFO 2022-10-19T14:40:56.956130: proxy: Finished proxy of http://127.0.0.1:3030/14
TRACE 2022-10-19T14:40:56.956369: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.962231: proxy: Blocked with CPU for 100ms for GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T14:40:56.962584: mio::poll: registering event source with poller: token=Token(16777268), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.965168: proxy: Blocked with CPU for 100ms for GET /46 -> http://127.0.0.1:3030/46
TRACE 2022-10-19T14:40:56.966252: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.967065: proxy: Blocked with CPU for 100ms for GET /35 -> http://127.0.0.1:3030/35
TRACE 2022-10-19T14:40:56.967548: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.968388: proxy: Blocked with CPU for 100ms for GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T14:40:56.968454: proxy: Blocked with CPU for 100ms for GET /17 -> http://127.0.0.1:3030/17
TRACE 2022-10-19T14:40:56.968688: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:56.968934: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:56.969665: proxy: Blocked with CPU for 100ms for GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T14:40:56.970102: proxy: Blocked with CPU for 100ms for GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T14:40:56.971820: proxy: Blocked with CPU for 100ms for GET /50 -> http://127.0.0.1:3030/50
 INFO 2022-10-19T14:40:56.972310: proxy: Blocked with CPU for 100ms for GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T14:40:56.987171: proxy: Finished proxy of http://127.0.0.1:3030/37
 INFO 2022-10-19T14:40:56.987171: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T14:40:56.987411: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.987411: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.987851: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T14:40:56.988043: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.988180: proxy: Finished proxy of http://127.0.0.1:3030/10
 INFO 2022-10-19T14:40:56.988196: proxy: Finished proxy of http://127.0.0.1:3030/11
 INFO 2022-10-19T14:40:56.988194: proxy: Finished proxy of http://127.0.0.1:3030/27
TRACE 2022-10-19T14:40:56.988370: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.988371: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.988373: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.989171: proxy: Finished proxy of http://127.0.0.1:3030/28
 INFO 2022-10-19T14:40:56.989171: proxy: Finished proxy of http://127.0.0.1:3030/23
 INFO 2022-10-19T14:40:56.989180: proxy: Finished proxy of http://127.0.0.1:3030/1
 INFO 2022-10-19T14:40:56.989202: proxy: Finished proxy of http://127.0.0.1:3030/26
 INFO 2022-10-19T14:40:56.989213: proxy: Finished proxy of http://127.0.0.1:3030/18
 INFO 2022-10-19T14:40:56.989244: proxy: Finished proxy of http://127.0.0.1:3030/3
TRACE 2022-10-19T14:40:56.989374: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.989395: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.989442: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.989486: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.989745: proxy: Finished proxy of http://127.0.0.1:3030/8
TRACE 2022-10-19T14:40:56.989917: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.989996: proxy: Finished proxy of http://127.0.0.1:3030/24
 INFO 2022-10-19T14:40:56.990104: proxy: Finished proxy of http://127.0.0.1:3030/9
TRACE 2022-10-19T14:40:56.990199: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.990229: proxy: Finished proxy of http://127.0.0.1:3030/34
TRACE 2022-10-19T14:40:56.990284: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.990366: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T14:40:56.990403: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.990546: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.990839: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.991053: proxy: Finished proxy of http://127.0.0.1:3030/19
 INFO 2022-10-19T14:40:56.991143: proxy: Finished proxy of http://127.0.0.1:3030/2
 INFO 2022-10-19T14:40:56.991287: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T14:40:56.991311: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.991490: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.991052: proxy: Finished proxy of http://127.0.0.1:3030/6
 INFO 2022-10-19T14:40:56.991052: proxy: Finished proxy of http://127.0.0.1:3030/31
TRACE 2022-10-19T14:40:56.991713: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.991738: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.991797: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.991899: proxy: Finished proxy of http://127.0.0.1:3030/12
 INFO 2022-10-19T14:40:56.992057: proxy: Finished proxy of http://127.0.0.1:3030/25
TRACE 2022-10-19T14:40:56.992079: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.992198: proxy: Finished proxy of http://127.0.0.1:3030/39
TRACE 2022-10-19T14:40:56.992264: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.992400: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.992397: proxy: Finished proxy of http://127.0.0.1:3030/40
 INFO 2022-10-19T14:40:56.992533: proxy: Finished proxy of http://127.0.0.1:3030/20
TRACE 2022-10-19T14:40:56.992562: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.992680: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T14:40:56.992706: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.992860: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.992977: proxy: Finished proxy of http://127.0.0.1:3030/43
TRACE 2022-10-19T14:40:56.993151: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.993235: proxy: Finished proxy of http://127.0.0.1:3030/48
 INFO 2022-10-19T14:40:56.993305: proxy: Finished proxy of http://127.0.0.1:3030/29
TRACE 2022-10-19T14:40:56.993416: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.993507: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.993620: proxy: Finished proxy of http://127.0.0.1:3030/45
 INFO 2022-10-19T14:40:56.993663: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T14:40:56.993836: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.993845: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.994029: proxy: Finished proxy of http://127.0.0.1:3030/41
TRACE 2022-10-19T14:40:56.994195: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.994352: proxy: Finished proxy of http://127.0.0.1:3030/15
 INFO 2022-10-19T14:40:56.994470: proxy: Finished proxy of http://127.0.0.1:3030/36
TRACE 2022-10-19T14:40:56.994585: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.994600: proxy: Finished proxy of http://127.0.0.1:3030/49
TRACE 2022-10-19T14:40:56.994727: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.994806: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.995119: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.995157: proxy: Finished proxy of http://127.0.0.1:3030/22
TRACE 2022-10-19T14:40:56.995369: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.995405: proxy: Finished proxy of http://127.0.0.1:3030/42
 INFO 2022-10-19T14:40:56.995665: proxy: Finished proxy of http://127.0.0.1:3030/30
TRACE 2022-10-19T14:40:56.995866: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:56.995981: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:56.999759: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T14:40:56.999981: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:57.000100: proxy: Finished proxy of http://127.0.0.1:3030/46
TRACE 2022-10-19T14:40:57.000308: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:57.000434: proxy: Finished proxy of http://127.0.0.1:3030/17
TRACE 2022-10-19T14:40:57.000654: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:57.000691: proxy: Finished proxy of http://127.0.0.1:3030/47
TRACE 2022-10-19T14:40:57.000884: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.006245: mio::poll: registering event source with poller: token=Token(16777311), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:57.006253: mio::poll: registering event source with poller: token=Token(16777308), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:57.006245: mio::poll: registering event source with poller: token=Token(16777310), interests=READABLE | WRITABLE
TRACE 2022-10-19T14:40:57.006262: mio::poll: registering event source with poller: token=Token(16777309), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:57.038638: proxy: Finished proxy of http://127.0.0.1:3030/38
 INFO 2022-10-19T14:40:57.038638: proxy: Finished proxy of http://127.0.0.1:3030/32
 INFO 2022-10-19T14:40:57.038638: proxy: Finished proxy of http://127.0.0.1:3030/50
 INFO 2022-10-19T14:40:57.038638: proxy: Finished proxy of http://127.0.0.1:3030/44
TRACE 2022-10-19T14:40:57.038860: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.038861: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.038860: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.039175: mio::poll: deregistering event source from poller
 INFO 2022-10-19T14:40:57.055130: proxy: Blocked with CPU for 100ms for GET /13 -> http://127.0.0.1:3030/13
TRACE 2022-10-19T14:40:57.055413: mio::poll: registering event source with poller: token=Token(33554525), interests=READABLE | WRITABLE
 INFO 2022-10-19T14:40:57.087834: proxy: Finished proxy of http://127.0.0.1:3030/13
TRACE 2022-10-19T14:40:57.088047: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089018: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089018: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089068: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089061: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089106: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089122: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089154: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089180: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089177: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089194: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089219: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089223: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089228: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089271: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089289: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089292: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089305: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089330: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089329: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089334: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089387: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089394: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089396: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089441: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089444: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089456: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089474: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089496: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089500: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089502: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089543: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089547: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089561: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089583: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089598: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089603: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089613: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089642: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089667: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089689: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089649: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089700: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089766: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089707: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089722: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089806: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089813: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089820: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089831: mio::poll: deregistering event source from poller
TRACE 2022-10-19T14:40:57.089838: mio::poll: deregistering event source from poller
```

## Trying with only 1 worker thread with 100ms CPU before each request

```
   Compiling tokio-hyper-test v0.1.0 (/home/michael/dev/vmware/tokio-hyper-test)
    Finished dev [unoptimized + debuginfo] target(s) in 2.69s
     Running `target/debug/proxy`
TRACE 2022-10-19T15:06:02.422563: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:02.422708: proxy: Starting server on 127.0.0.1:3000
TRACE 2022-10-19T15:06:08.938447: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.938613: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.938704: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.938764: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.938871: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.938971: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939029: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939096: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939218: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939296: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939425: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939517: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939602: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939743: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939831: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.939978: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940062: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940151: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940276: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940402: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940489: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940615: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940743: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940871: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.940951: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941078: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941200: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941283: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941410: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941536: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941627: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941751: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.941875: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942004: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942103: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942269: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942407: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942489: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942616: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942700: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942829: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.942965: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943047: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943174: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943259: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943386: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943471: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943606: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943737: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:06:08.943823: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First proxy request starts at 8.946

```
 INFO 2022-10-19T15:06:08.945930: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
 INFO 2022-10-19T15:06:09.046137: proxy: Blocked with CPU for 100ms for GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T15:06:09.046441: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.046998: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T15:06:09.147061: proxy: Blocked with CPU for 100ms for GET /2 -> http://127.0.0.1:3030/2
TRACE 2022-10-19T15:06:09.147398: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.147723: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T15:06:09.247823: proxy: Blocked with CPU for 100ms for GET /3 -> http://127.0.0.1:3030/3
TRACE 2022-10-19T15:06:09.248192: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.248399: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T15:06:09.348484: proxy: Blocked with CPU for 100ms for GET /4 -> http://127.0.0.1:3030/4
TRACE 2022-10-19T15:06:09.348791: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.349100: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T15:06:09.449184: proxy: Blocked with CPU for 100ms for GET /5 -> http://127.0.0.1:3030/5
TRACE 2022-10-19T15:06:09.449486: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.449722: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
 INFO 2022-10-19T15:06:09.549786: proxy: Blocked with CPU for 100ms for GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T15:06:09.550068: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.550244: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T15:06:09.650306: proxy: Blocked with CPU for 100ms for GET /7 -> http://127.0.0.1:3030/7
TRACE 2022-10-19T15:06:09.650584: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.650790: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T15:06:09.750856: proxy: Blocked with CPU for 100ms for GET /8 -> http://127.0.0.1:3030/8
TRACE 2022-10-19T15:06:09.751153: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.751371: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T15:06:09.851434: proxy: Blocked with CPU for 100ms for GET /9 -> http://127.0.0.1:3030/9
TRACE 2022-10-19T15:06:09.851714: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.851934: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T15:06:09.952009: proxy: Blocked with CPU for 100ms for GET /10 -> http://127.0.0.1:3030/10
TRACE 2022-10-19T15:06:09.952284: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:09.952481: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T15:06:10.052543: proxy: Blocked with CPU for 100ms for GET /11 -> http://127.0.0.1:3030/11
TRACE 2022-10-19T15:06:10.052825: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.053027: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
 INFO 2022-10-19T15:06:10.153094: proxy: Blocked with CPU for 100ms for GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T15:06:10.153375: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.153578: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T15:06:10.253643: proxy: Blocked with CPU for 100ms for GET /13 -> http://127.0.0.1:3030/13
TRACE 2022-10-19T15:06:10.254007: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.254308: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T15:06:10.354389: proxy: Blocked with CPU for 100ms for GET /14 -> http://127.0.0.1:3030/14
TRACE 2022-10-19T15:06:10.354763: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.355012: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T15:06:10.455076: proxy: Blocked with CPU for 100ms for GET /15 -> http://127.0.0.1:3030/15
TRACE 2022-10-19T15:06:10.455372: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.455577: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
 INFO 2022-10-19T15:06:10.555650: proxy: Blocked with CPU for 100ms for GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T15:06:10.555936: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.556126: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T15:06:10.656189: proxy: Blocked with CPU for 100ms for GET /17 -> http://127.0.0.1:3030/17
TRACE 2022-10-19T15:06:10.656487: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.656682: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T15:06:10.756753: proxy: Blocked with CPU for 100ms for GET /18 -> http://127.0.0.1:3030/18
TRACE 2022-10-19T15:06:10.757071: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.757280: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T15:06:10.857345: proxy: Blocked with CPU for 100ms for GET /19 -> http://127.0.0.1:3030/19
TRACE 2022-10-19T15:06:10.857634: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.857839: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
 INFO 2022-10-19T15:06:10.957904: proxy: Blocked with CPU for 100ms for GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T15:06:10.958215: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:10.958690: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T15:06:11.058758: proxy: Blocked with CPU for 100ms for GET /21 -> http://127.0.0.1:3030/21
TRACE 2022-10-19T15:06:11.059043: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.059217: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T15:06:11.159282: proxy: Blocked with CPU for 100ms for GET /22 -> http://127.0.0.1:3030/22
TRACE 2022-10-19T15:06:11.159632: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.159872: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T15:06:11.259949: proxy: Blocked with CPU for 100ms for GET /23 -> http://127.0.0.1:3030/23
TRACE 2022-10-19T15:06:11.260232: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.260414: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T15:06:11.360477: proxy: Blocked with CPU for 100ms for GET /24 -> http://127.0.0.1:3030/24
TRACE 2022-10-19T15:06:11.360754: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.360938: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T15:06:11.461002: proxy: Blocked with CPU for 100ms for GET /25 -> http://127.0.0.1:3030/25
TRACE 2022-10-19T15:06:11.461297: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.461505: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T15:06:11.561578: proxy: Blocked with CPU for 100ms for GET /26 -> http://127.0.0.1:3030/26
TRACE 2022-10-19T15:06:11.561860: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.562050: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T15:06:11.662139: proxy: Blocked with CPU for 100ms for GET /27 -> http://127.0.0.1:3030/27
TRACE 2022-10-19T15:06:11.662448: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.662631: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T15:06:11.762695: proxy: Blocked with CPU for 100ms for GET /28 -> http://127.0.0.1:3030/28
TRACE 2022-10-19T15:06:11.762983: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.763169: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T15:06:11.863244: proxy: Blocked with CPU for 100ms for GET /29 -> http://127.0.0.1:3030/29
TRACE 2022-10-19T15:06:11.863514: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.863683: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
 INFO 2022-10-19T15:06:11.963750: proxy: Blocked with CPU for 100ms for GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T15:06:11.964023: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:11.964200: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
 INFO 2022-10-19T15:06:12.064263: proxy: Blocked with CPU for 100ms for GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T15:06:12.064563: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.064747: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T15:06:12.164820: proxy: Blocked with CPU for 100ms for GET /32 -> http://127.0.0.1:3030/32
TRACE 2022-10-19T15:06:12.165102: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.165279: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
 INFO 2022-10-19T15:06:12.265343: proxy: Blocked with CPU for 100ms for GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T15:06:12.265628: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.265829: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
 INFO 2022-10-19T15:06:12.365908: proxy: Blocked with CPU for 100ms for GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T15:06:12.366186: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.366354: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T15:06:12.466417: proxy: Blocked with CPU for 100ms for GET /35 -> http://127.0.0.1:3030/35
TRACE 2022-10-19T15:06:12.466698: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.466881: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T15:06:12.566944: proxy: Blocked with CPU for 100ms for GET /36 -> http://127.0.0.1:3030/36
TRACE 2022-10-19T15:06:12.567224: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.567432: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
 INFO 2022-10-19T15:06:12.667495: proxy: Blocked with CPU for 100ms for GET /37 -> http://127.0.0.1:3030/37
TRACE 2022-10-19T15:06:12.667757: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.667936: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T15:06:12.767998: proxy: Blocked with CPU for 100ms for GET /38 -> http://127.0.0.1:3030/38
TRACE 2022-10-19T15:06:12.768271: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.768475: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
 INFO 2022-10-19T15:06:12.868538: proxy: Blocked with CPU for 100ms for GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T15:06:12.868841: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.869044: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T15:06:12.969115: proxy: Blocked with CPU for 100ms for GET /40 -> http://127.0.0.1:3030/40
TRACE 2022-10-19T15:06:12.969380: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:12.969551: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T15:06:13.069618: proxy: Blocked with CPU for 100ms for GET /41 -> http://127.0.0.1:3030/41
TRACE 2022-10-19T15:06:13.069979: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.070189: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
 INFO 2022-10-19T15:06:13.170254: proxy: Blocked with CPU for 100ms for GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T15:06:13.170534: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.170712: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
 INFO 2022-10-19T15:06:13.270776: proxy: Blocked with CPU for 100ms for GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T15:06:13.271082: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.271264: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T15:06:13.371330: proxy: Blocked with CPU for 100ms for GET /44 -> http://127.0.0.1:3030/44
TRACE 2022-10-19T15:06:13.371627: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.371854: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
 INFO 2022-10-19T15:06:13.471930: proxy: Blocked with CPU for 100ms for GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T15:06:13.498447: mio::poll: registering event source with poller: token=Token(96), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.498655: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T15:06:13.598714: proxy: Blocked with CPU for 100ms for GET /46 -> http://127.0.0.1:3030/46
TRACE 2022-10-19T15:06:13.598999: mio::poll: registering event source with poller: token=Token(97), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.599189: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T15:06:13.699252: proxy: Blocked with CPU for 100ms for GET /47 -> http://127.0.0.1:3030/47
TRACE 2022-10-19T15:06:13.699530: mio::poll: registering event source with poller: token=Token(98), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.699711: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T15:06:13.799780: proxy: Blocked with CPU for 100ms for GET /48 -> http://127.0.0.1:3030/48
TRACE 2022-10-19T15:06:13.800076: mio::poll: registering event source with poller: token=Token(99), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.800248: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T15:06:13.900311: proxy: Blocked with CPU for 100ms for GET /49 -> http://127.0.0.1:3030/49
TRACE 2022-10-19T15:06:13.900602: mio::poll: registering event source with poller: token=Token(100), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:06:13.900780: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
 INFO 2022-10-19T15:06:14.000841: proxy: Blocked with CPU for 100ms for GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T15:06:14.001107: mio::poll: registering event source with poller: token=Token(101), interests=READABLE | WRITABLE
```

First proxy request finished at 14.001, over 5s later, and all returned after another 70ms

```
 INFO 2022-10-19T15:06:14.001395: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T15:06:14.001550: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.004915: proxy: Finished proxy of http://127.0.0.1:3030/20
TRACE 2022-10-19T15:06:14.005063: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.034371: proxy: Finished proxy of http://127.0.0.1:3030/2
TRACE 2022-10-19T15:06:14.034969: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.035843: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T15:06:14.036446: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.037526: proxy: Finished proxy of http://127.0.0.1:3030/18
TRACE 2022-10-19T15:06:14.038088: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.038894: proxy: Finished proxy of http://127.0.0.1:3030/3
TRACE 2022-10-19T15:06:14.039475: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.040330: proxy: Finished proxy of http://127.0.0.1:3030/10
TRACE 2022-10-19T15:06:14.040868: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.041643: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T15:06:14.042227: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.042963: proxy: Finished proxy of http://127.0.0.1:3030/6
TRACE 2022-10-19T15:06:14.043465: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.044240: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T15:06:14.044736: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.045456: proxy: Finished proxy of http://127.0.0.1:3030/8
TRACE 2022-10-19T15:06:14.046023: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.046835: proxy: Finished proxy of http://127.0.0.1:3030/9
TRACE 2022-10-19T15:06:14.047192: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.047793: proxy: Finished proxy of http://127.0.0.1:3030/17
TRACE 2022-10-19T15:06:14.048135: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.048707: proxy: Finished proxy of http://127.0.0.1:3030/13
TRACE 2022-10-19T15:06:14.049057: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.049641: proxy: Finished proxy of http://127.0.0.1:3030/15
TRACE 2022-10-19T15:06:14.049985: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.050601: proxy: Finished proxy of http://127.0.0.1:3030/14
TRACE 2022-10-19T15:06:14.050933: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.051523: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T15:06:14.051799: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.052221: proxy: Finished proxy of http://127.0.0.1:3030/11
TRACE 2022-10-19T15:06:14.052489: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.052940: proxy: Finished proxy of http://127.0.0.1:3030/12
TRACE 2022-10-19T15:06:14.053192: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.053648: proxy: Finished proxy of http://127.0.0.1:3030/50
TRACE 2022-10-19T15:06:14.053901: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.054369: proxy: Finished proxy of http://127.0.0.1:3030/19
TRACE 2022-10-19T15:06:14.054634: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.055374: proxy: Finished proxy of http://127.0.0.1:3030/48
TRACE 2022-10-19T15:06:14.055676: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.056229: proxy: Finished proxy of http://127.0.0.1:3030/26
TRACE 2022-10-19T15:06:14.056591: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.057003: proxy: Finished proxy of http://127.0.0.1:3030/22
TRACE 2022-10-19T15:06:14.057249: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.057632: proxy: Finished proxy of http://127.0.0.1:3030/23
TRACE 2022-10-19T15:06:14.057858: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.058264: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T15:06:14.058476: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.058850: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T15:06:14.059127: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.059525: proxy: Finished proxy of http://127.0.0.1:3030/25
TRACE 2022-10-19T15:06:14.059759: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.060144: proxy: Finished proxy of http://127.0.0.1:3030/32
TRACE 2022-10-19T15:06:14.060354: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.060740: proxy: Finished proxy of http://127.0.0.1:3030/28
TRACE 2022-10-19T15:06:14.060973: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.061341: proxy: Finished proxy of http://127.0.0.1:3030/30
TRACE 2022-10-19T15:06:14.061588: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.061919: proxy: Finished proxy of http://127.0.0.1:3030/31
TRACE 2022-10-19T15:06:14.062148: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.062492: proxy: Finished proxy of http://127.0.0.1:3030/29
TRACE 2022-10-19T15:06:14.062688: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.063019: proxy: Finished proxy of http://127.0.0.1:3030/27
TRACE 2022-10-19T15:06:14.063261: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.063602: proxy: Finished proxy of http://127.0.0.1:3030/39
TRACE 2022-10-19T15:06:14.063805: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.064120: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T15:06:14.064320: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.064633: proxy: Finished proxy of http://127.0.0.1:3030/37
TRACE 2022-10-19T15:06:14.064831: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.065134: proxy: Finished proxy of http://127.0.0.1:3030/38
TRACE 2022-10-19T15:06:14.065331: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.065646: proxy: Finished proxy of http://127.0.0.1:3030/36
TRACE 2022-10-19T15:06:14.065837: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.066171: proxy: Finished proxy of http://127.0.0.1:3030/34
TRACE 2022-10-19T15:06:14.066370: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.066703: proxy: Finished proxy of http://127.0.0.1:3030/45
TRACE 2022-10-19T15:06:14.066875: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.067189: proxy: Finished proxy of http://127.0.0.1:3030/41
TRACE 2022-10-19T15:06:14.067374: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.067683: proxy: Finished proxy of http://127.0.0.1:3030/42
TRACE 2022-10-19T15:06:14.067855: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.068156: proxy: Finished proxy of http://127.0.0.1:3030/43
TRACE 2022-10-19T15:06:14.068338: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.068624: proxy: Finished proxy of http://127.0.0.1:3030/40
TRACE 2022-10-19T15:06:14.068806: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.069096: proxy: Finished proxy of http://127.0.0.1:3030/44
TRACE 2022-10-19T15:06:14.069275: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.069542: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T15:06:14.069718: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.069994: proxy: Finished proxy of http://127.0.0.1:3030/46
TRACE 2022-10-19T15:06:14.070197: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.070447: proxy: Finished proxy of http://127.0.0.1:3030/49
TRACE 2022-10-19T15:06:14.070602: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:06:14.070857: proxy: Finished proxy of http://127.0.0.1:3030/47
TRACE 2022-10-19T15:06:14.071037: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072266: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072409: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072519: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072635: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072739: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072841: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.072942: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073045: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073146: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073244: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073347: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073455: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073556: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073658: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073760: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073861: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.073962: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074076: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074186: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074277: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074365: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074453: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074540: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074628: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074713: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074800: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074887: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.074973: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075089: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075208: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075333: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075447: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075566: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075680: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075798: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.075916: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076034: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076153: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076269: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076389: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076506: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076628: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076736: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076834: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.076935: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.077043: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.077155: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.077262: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.077370: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:06:14.077478: mio::poll: deregistering event source from poller
```

## Trying with only 1 worker thread with 0ms CPU before each request

```
   Compiling tokio-hyper-test v0.1.0 (/home/michael/dev/vmware/tokio-hyper-test)
    Finished dev [unoptimized + debuginfo] target(s) in 2.71s
     Running `target/debug/proxy`
TRACE 2022-10-19T15:13:34.401464: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:34.401612: proxy: Starting server on 127.0.0.1:3000
TRACE 2022-10-19T15:13:38.816428: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816614: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816675: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816728: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816784: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816841: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.816996: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817063: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817220: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817297: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817459: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817567: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817712: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817775: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817892: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.817979: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818130: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818211: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818278: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818403: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818479: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818552: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818773: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818850: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818905: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.818962: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819070: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819181: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819268: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819392: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819524: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819626: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819776: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819916: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.819998: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820121: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820246: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820385: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820463: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820545: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820667: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820792: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820873: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.820997: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821122: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821203: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821327: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821480: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821561: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-19T15:13:38.821688: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
```

First query begins at 38.823

```
 INFO 2022-10-19T15:13:38.822802: proxy: Start proxy of GET /24 -> http://127.0.0.1:3030/24
 INFO 2022-10-19T15:13:38.822934: proxy: Blocked with CPU for 0ms for GET /24 -> http://127.0.0.1:3030/24
TRACE 2022-10-19T15:13:38.823179: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.823791: proxy: Start proxy of GET /1 -> http://127.0.0.1:3030/1
 INFO 2022-10-19T15:13:38.823832: proxy: Blocked with CPU for 0ms for GET /1 -> http://127.0.0.1:3030/1
TRACE 2022-10-19T15:13:38.823990: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.824348: proxy: Start proxy of GET /2 -> http://127.0.0.1:3030/2
 INFO 2022-10-19T15:13:38.824388: proxy: Blocked with CPU for 0ms for GET /2 -> http://127.0.0.1:3030/2
TRACE 2022-10-19T15:13:38.824540: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.824668: proxy: Start proxy of GET /3 -> http://127.0.0.1:3030/3
 INFO 2022-10-19T15:13:38.824706: proxy: Blocked with CPU for 0ms for GET /3 -> http://127.0.0.1:3030/3
TRACE 2022-10-19T15:13:38.824833: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.824973: proxy: Start proxy of GET /4 -> http://127.0.0.1:3030/4
 INFO 2022-10-19T15:13:38.825024: proxy: Blocked with CPU for 0ms for GET /4 -> http://127.0.0.1:3030/4
TRACE 2022-10-19T15:13:38.825212: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.825366: proxy: Start proxy of GET /5 -> http://127.0.0.1:3030/5
 INFO 2022-10-19T15:13:38.825418: proxy: Blocked with CPU for 0ms for GET /5 -> http://127.0.0.1:3030/5
TRACE 2022-10-19T15:13:38.825569: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.825709: proxy: Start proxy of GET /6 -> http://127.0.0.1:3030/6
 INFO 2022-10-19T15:13:38.825766: proxy: Blocked with CPU for 0ms for GET /6 -> http://127.0.0.1:3030/6
TRACE 2022-10-19T15:13:38.825937: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.826164: proxy: Start proxy of GET /45 -> http://127.0.0.1:3030/45
 INFO 2022-10-19T15:13:38.826209: proxy: Blocked with CPU for 0ms for GET /45 -> http://127.0.0.1:3030/45
TRACE 2022-10-19T15:13:38.826369: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.826491: proxy: Start proxy of GET /7 -> http://127.0.0.1:3030/7
 INFO 2022-10-19T15:13:38.826529: proxy: Blocked with CPU for 0ms for GET /7 -> http://127.0.0.1:3030/7
TRACE 2022-10-19T15:13:38.826652: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.826765: proxy: Start proxy of GET /8 -> http://127.0.0.1:3030/8
 INFO 2022-10-19T15:13:38.826803: proxy: Blocked with CPU for 0ms for GET /8 -> http://127.0.0.1:3030/8
TRACE 2022-10-19T15:13:38.826913: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.827026: proxy: Start proxy of GET /9 -> http://127.0.0.1:3030/9
 INFO 2022-10-19T15:13:38.827064: proxy: Blocked with CPU for 0ms for GET /9 -> http://127.0.0.1:3030/9
TRACE 2022-10-19T15:13:38.827173: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.827285: proxy: Start proxy of GET /10 -> http://127.0.0.1:3030/10
 INFO 2022-10-19T15:13:38.827324: proxy: Blocked with CPU for 0ms for GET /10 -> http://127.0.0.1:3030/10
TRACE 2022-10-19T15:13:38.827452: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.827567: proxy: Start proxy of GET /11 -> http://127.0.0.1:3030/11
 INFO 2022-10-19T15:13:38.827605: proxy: Blocked with CPU for 0ms for GET /11 -> http://127.0.0.1:3030/11
TRACE 2022-10-19T15:13:38.827718: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.827838: proxy: Start proxy of GET /12 -> http://127.0.0.1:3030/12
 INFO 2022-10-19T15:13:38.827876: proxy: Blocked with CPU for 0ms for GET /12 -> http://127.0.0.1:3030/12
TRACE 2022-10-19T15:13:38.827985: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.828095: proxy: Start proxy of GET /13 -> http://127.0.0.1:3030/13
 INFO 2022-10-19T15:13:38.828133: proxy: Blocked with CPU for 0ms for GET /13 -> http://127.0.0.1:3030/13
TRACE 2022-10-19T15:13:38.828247: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.828362: proxy: Start proxy of GET /14 -> http://127.0.0.1:3030/14
 INFO 2022-10-19T15:13:38.828400: proxy: Blocked with CPU for 0ms for GET /14 -> http://127.0.0.1:3030/14
TRACE 2022-10-19T15:13:38.828512: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.828853: proxy: Start proxy of GET /15 -> http://127.0.0.1:3030/15
 INFO 2022-10-19T15:13:38.828891: proxy: Blocked with CPU for 0ms for GET /15 -> http://127.0.0.1:3030/15
TRACE 2022-10-19T15:13:38.829002: mio::poll: registering event source with poller: token=Token(68), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.829118: proxy: Start proxy of GET /16 -> http://127.0.0.1:3030/16
 INFO 2022-10-19T15:13:38.829192: proxy: Blocked with CPU for 0ms for GET /16 -> http://127.0.0.1:3030/16
TRACE 2022-10-19T15:13:38.829335: mio::poll: registering event source with poller: token=Token(69), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.829460: proxy: Start proxy of GET /17 -> http://127.0.0.1:3030/17
 INFO 2022-10-19T15:13:38.829506: proxy: Blocked with CPU for 0ms for GET /17 -> http://127.0.0.1:3030/17
TRACE 2022-10-19T15:13:38.829653: mio::poll: registering event source with poller: token=Token(70), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.829780: proxy: Start proxy of GET /18 -> http://127.0.0.1:3030/18
 INFO 2022-10-19T15:13:38.829828: proxy: Blocked with CPU for 0ms for GET /18 -> http://127.0.0.1:3030/18
TRACE 2022-10-19T15:13:38.829980: mio::poll: registering event source with poller: token=Token(71), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.830378: proxy: Start proxy of GET /19 -> http://127.0.0.1:3030/19
 INFO 2022-10-19T15:13:38.830455: proxy: Blocked with CPU for 0ms for GET /19 -> http://127.0.0.1:3030/19
TRACE 2022-10-19T15:13:38.830686: mio::poll: registering event source with poller: token=Token(72), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.830962: proxy: Start proxy of GET /20 -> http://127.0.0.1:3030/20
 INFO 2022-10-19T15:13:38.831016: proxy: Blocked with CPU for 0ms for GET /20 -> http://127.0.0.1:3030/20
TRACE 2022-10-19T15:13:38.831355: mio::poll: registering event source with poller: token=Token(73), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.831565: proxy: Start proxy of GET /21 -> http://127.0.0.1:3030/21
 INFO 2022-10-19T15:13:38.831638: proxy: Blocked with CPU for 0ms for GET /21 -> http://127.0.0.1:3030/21
TRACE 2022-10-19T15:13:38.831843: mio::poll: registering event source with poller: token=Token(74), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.831985: proxy: Start proxy of GET /22 -> http://127.0.0.1:3030/22
 INFO 2022-10-19T15:13:38.832036: proxy: Blocked with CPU for 0ms for GET /22 -> http://127.0.0.1:3030/22
TRACE 2022-10-19T15:13:38.832212: mio::poll: registering event source with poller: token=Token(75), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.832719: proxy: Start proxy of GET /23 -> http://127.0.0.1:3030/23
 INFO 2022-10-19T15:13:38.832776: proxy: Blocked with CPU for 0ms for GET /23 -> http://127.0.0.1:3030/23
TRACE 2022-10-19T15:13:38.832949: mio::poll: registering event source with poller: token=Token(76), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.833090: proxy: Start proxy of GET /25 -> http://127.0.0.1:3030/25
 INFO 2022-10-19T15:13:38.833141: proxy: Blocked with CPU for 0ms for GET /25 -> http://127.0.0.1:3030/25
TRACE 2022-10-19T15:13:38.833296: mio::poll: registering event source with poller: token=Token(77), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.833451: proxy: Start proxy of GET /26 -> http://127.0.0.1:3030/26
 INFO 2022-10-19T15:13:38.833520: proxy: Blocked with CPU for 0ms for GET /26 -> http://127.0.0.1:3030/26
TRACE 2022-10-19T15:13:38.833707: mio::poll: registering event source with poller: token=Token(78), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.833855: proxy: Start proxy of GET /27 -> http://127.0.0.1:3030/27
 INFO 2022-10-19T15:13:38.833907: proxy: Blocked with CPU for 0ms for GET /27 -> http://127.0.0.1:3030/27
TRACE 2022-10-19T15:13:38.834109: mio::poll: registering event source with poller: token=Token(79), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.834341: proxy: Start proxy of GET /28 -> http://127.0.0.1:3030/28
 INFO 2022-10-19T15:13:38.834453: proxy: Blocked with CPU for 0ms for GET /28 -> http://127.0.0.1:3030/28
TRACE 2022-10-19T15:13:38.834643: mio::poll: registering event source with poller: token=Token(80), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.834802: proxy: Start proxy of GET /29 -> http://127.0.0.1:3030/29
 INFO 2022-10-19T15:13:38.834855: proxy: Blocked with CPU for 0ms for GET /29 -> http://127.0.0.1:3030/29
TRACE 2022-10-19T15:13:38.835018: mio::poll: registering event source with poller: token=Token(81), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.835170: proxy: Start proxy of GET /30 -> http://127.0.0.1:3030/30
 INFO 2022-10-19T15:13:38.835222: proxy: Blocked with CPU for 0ms for GET /30 -> http://127.0.0.1:3030/30
TRACE 2022-10-19T15:13:38.835397: mio::poll: registering event source with poller: token=Token(82), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.835830: proxy: Start proxy of GET /31 -> http://127.0.0.1:3030/31
 INFO 2022-10-19T15:13:38.835888: proxy: Blocked with CPU for 0ms for GET /31 -> http://127.0.0.1:3030/31
TRACE 2022-10-19T15:13:38.836063: mio::poll: registering event source with poller: token=Token(83), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.836237: proxy: Start proxy of GET /32 -> http://127.0.0.1:3030/32
 INFO 2022-10-19T15:13:38.836307: proxy: Blocked with CPU for 0ms for GET /32 -> http://127.0.0.1:3030/32
TRACE 2022-10-19T15:13:38.836486: mio::poll: registering event source with poller: token=Token(84), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.836626: proxy: Start proxy of GET /33 -> http://127.0.0.1:3030/33
 INFO 2022-10-19T15:13:38.836679: proxy: Blocked with CPU for 0ms for GET /33 -> http://127.0.0.1:3030/33
TRACE 2022-10-19T15:13:38.836867: mio::poll: registering event source with poller: token=Token(85), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.837026: proxy: Start proxy of GET /34 -> http://127.0.0.1:3030/34
 INFO 2022-10-19T15:13:38.837080: proxy: Blocked with CPU for 0ms for GET /34 -> http://127.0.0.1:3030/34
TRACE 2022-10-19T15:13:38.837256: mio::poll: registering event source with poller: token=Token(86), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.837416: proxy: Start proxy of GET /35 -> http://127.0.0.1:3030/35
 INFO 2022-10-19T15:13:38.837492: proxy: Blocked with CPU for 0ms for GET /35 -> http://127.0.0.1:3030/35
TRACE 2022-10-19T15:13:38.837728: mio::poll: registering event source with poller: token=Token(87), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.837894: proxy: Start proxy of GET /36 -> http://127.0.0.1:3030/36
 INFO 2022-10-19T15:13:38.837968: proxy: Blocked with CPU for 0ms for GET /36 -> http://127.0.0.1:3030/36
TRACE 2022-10-19T15:13:38.838162: mio::poll: registering event source with poller: token=Token(88), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.838317: proxy: Start proxy of GET /37 -> http://127.0.0.1:3030/37
 INFO 2022-10-19T15:13:38.838387: proxy: Blocked with CPU for 0ms for GET /37 -> http://127.0.0.1:3030/37
TRACE 2022-10-19T15:13:38.838599: mio::poll: registering event source with poller: token=Token(89), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.839029: proxy: Start proxy of GET /38 -> http://127.0.0.1:3030/38
 INFO 2022-10-19T15:13:38.839112: proxy: Blocked with CPU for 0ms for GET /38 -> http://127.0.0.1:3030/38
TRACE 2022-10-19T15:13:38.839309: mio::poll: registering event source with poller: token=Token(90), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.839471: proxy: Start proxy of GET /39 -> http://127.0.0.1:3030/39
 INFO 2022-10-19T15:13:38.839524: proxy: Blocked with CPU for 0ms for GET /39 -> http://127.0.0.1:3030/39
TRACE 2022-10-19T15:13:38.839679: mio::poll: registering event source with poller: token=Token(91), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.839831: proxy: Start proxy of GET /40 -> http://127.0.0.1:3030/40
 INFO 2022-10-19T15:13:38.839897: proxy: Blocked with CPU for 0ms for GET /40 -> http://127.0.0.1:3030/40
TRACE 2022-10-19T15:13:38.840054: mio::poll: registering event source with poller: token=Token(92), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.840202: proxy: Start proxy of GET /41 -> http://127.0.0.1:3030/41
 INFO 2022-10-19T15:13:38.840253: proxy: Blocked with CPU for 0ms for GET /41 -> http://127.0.0.1:3030/41
TRACE 2022-10-19T15:13:38.840400: mio::poll: registering event source with poller: token=Token(93), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.840547: proxy: Start proxy of GET /42 -> http://127.0.0.1:3030/42
 INFO 2022-10-19T15:13:38.840596: proxy: Blocked with CPU for 0ms for GET /42 -> http://127.0.0.1:3030/42
TRACE 2022-10-19T15:13:38.840741: mio::poll: registering event source with poller: token=Token(94), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.840867: proxy: Start proxy of GET /43 -> http://127.0.0.1:3030/43
 INFO 2022-10-19T15:13:38.840917: proxy: Blocked with CPU for 0ms for GET /43 -> http://127.0.0.1:3030/43
TRACE 2022-10-19T15:13:38.841065: mio::poll: registering event source with poller: token=Token(95), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.841210: proxy: Start proxy of GET /44 -> http://127.0.0.1:3030/44
 INFO 2022-10-19T15:13:38.841263: proxy: Blocked with CPU for 0ms for GET /44 -> http://127.0.0.1:3030/44
TRACE 2022-10-19T15:13:38.878292: mio::poll: registering event source with poller: token=Token(96), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.879017: proxy: Start proxy of GET /46 -> http://127.0.0.1:3030/46
 INFO 2022-10-19T15:13:38.879076: proxy: Blocked with CPU for 0ms for GET /46 -> http://127.0.0.1:3030/46
TRACE 2022-10-19T15:13:38.879324: mio::poll: registering event source with poller: token=Token(97), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.879550: proxy: Start proxy of GET /47 -> http://127.0.0.1:3030/47
 INFO 2022-10-19T15:13:38.879634: proxy: Blocked with CPU for 0ms for GET /47 -> http://127.0.0.1:3030/47
TRACE 2022-10-19T15:13:38.879821: mio::poll: registering event source with poller: token=Token(98), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.879985: proxy: Start proxy of GET /48 -> http://127.0.0.1:3030/48
 INFO 2022-10-19T15:13:38.880072: proxy: Blocked with CPU for 0ms for GET /48 -> http://127.0.0.1:3030/48
TRACE 2022-10-19T15:13:38.880219: mio::poll: registering event source with poller: token=Token(99), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.880348: proxy: Start proxy of GET /49 -> http://127.0.0.1:3030/49
 INFO 2022-10-19T15:13:38.880398: proxy: Blocked with CPU for 0ms for GET /49 -> http://127.0.0.1:3030/49
TRACE 2022-10-19T15:13:38.880541: mio::poll: registering event source with poller: token=Token(100), interests=READABLE | WRITABLE
 INFO 2022-10-19T15:13:38.880668: proxy: Start proxy of GET /50 -> http://127.0.0.1:3030/50
 INFO 2022-10-19T15:13:38.880719: proxy: Blocked with CPU for 0ms for GET /50 -> http://127.0.0.1:3030/50
TRACE 2022-10-19T15:13:38.880864: mio::poll: registering event source with poller: token=Token(101), interests=READABLE | WRITABLE
```

First query completes at 38.887, or 64ms later, after  at 38.823

```
 INFO 2022-10-19T15:13:38.887441: proxy: Finished proxy of http://127.0.0.1:3030/24
TRACE 2022-10-19T15:13:38.887617: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.887893: proxy: Finished proxy of http://127.0.0.1:3030/1
TRACE 2022-10-19T15:13:38.888039: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.888272: proxy: Finished proxy of http://127.0.0.1:3030/14
TRACE 2022-10-19T15:13:38.888482: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.888700: proxy: Finished proxy of http://127.0.0.1:3030/2
TRACE 2022-10-19T15:13:38.888852: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.889117: proxy: Finished proxy of http://127.0.0.1:3030/3
TRACE 2022-10-19T15:13:38.889263: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.889451: proxy: Finished proxy of http://127.0.0.1:3030/4
TRACE 2022-10-19T15:13:38.889628: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.889835: proxy: Finished proxy of http://127.0.0.1:3030/5
TRACE 2022-10-19T15:13:38.889958: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.910661: proxy: Finished proxy of http://127.0.0.1:3030/6
TRACE 2022-10-19T15:13:38.910867: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.912410: proxy: Finished proxy of http://127.0.0.1:3030/45
TRACE 2022-10-19T15:13:38.912557: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.912931: proxy: Finished proxy of http://127.0.0.1:3030/11
TRACE 2022-10-19T15:13:38.913118: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.913387: proxy: Finished proxy of http://127.0.0.1:3030/7
TRACE 2022-10-19T15:13:38.913531: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.913751: proxy: Finished proxy of http://127.0.0.1:3030/8
TRACE 2022-10-19T15:13:38.913921: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.914173: proxy: Finished proxy of http://127.0.0.1:3030/9
TRACE 2022-10-19T15:13:38.914336: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.914570: proxy: Finished proxy of http://127.0.0.1:3030/10
TRACE 2022-10-19T15:13:38.914741: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.915045: proxy: Finished proxy of http://127.0.0.1:3030/21
TRACE 2022-10-19T15:13:38.915182: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.915388: proxy: Finished proxy of http://127.0.0.1:3030/17
TRACE 2022-10-19T15:13:38.915538: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.915762: proxy: Finished proxy of http://127.0.0.1:3030/13
TRACE 2022-10-19T15:13:38.915931: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.916193: proxy: Finished proxy of http://127.0.0.1:3030/12
TRACE 2022-10-19T15:13:38.916345: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.916582: proxy: Finished proxy of http://127.0.0.1:3030/15
TRACE 2022-10-19T15:13:38.916787: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.917081: proxy: Finished proxy of http://127.0.0.1:3030/50
TRACE 2022-10-19T15:13:38.917275: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.917530: proxy: Finished proxy of http://127.0.0.1:3030/31
TRACE 2022-10-19T15:13:38.917711: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.917904: proxy: Finished proxy of http://127.0.0.1:3030/16
TRACE 2022-10-19T15:13:38.918025: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.918244: proxy: Finished proxy of http://127.0.0.1:3030/22
TRACE 2022-10-19T15:13:38.918372: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.918594: proxy: Finished proxy of http://127.0.0.1:3030/19
TRACE 2022-10-19T15:13:38.918750: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.918976: proxy: Finished proxy of http://127.0.0.1:3030/18
TRACE 2022-10-19T15:13:38.919155: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.919393: proxy: Finished proxy of http://127.0.0.1:3030/20
TRACE 2022-10-19T15:13:38.919544: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.919802: proxy: Finished proxy of http://127.0.0.1:3030/30
TRACE 2022-10-19T15:13:38.919944: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.920157: proxy: Finished proxy of http://127.0.0.1:3030/26
TRACE 2022-10-19T15:13:38.920296: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.920512: proxy: Finished proxy of http://127.0.0.1:3030/28
TRACE 2022-10-19T15:13:38.920649: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.920858: proxy: Finished proxy of http://127.0.0.1:3030/27
TRACE 2022-10-19T15:13:38.920993: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.921215: proxy: Finished proxy of http://127.0.0.1:3030/29
TRACE 2022-10-19T15:13:38.921400: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.921623: proxy: Finished proxy of http://127.0.0.1:3030/23
TRACE 2022-10-19T15:13:38.921787: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.922029: proxy: Finished proxy of http://127.0.0.1:3030/25
TRACE 2022-10-19T15:13:38.922244: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.922525: proxy: Finished proxy of http://127.0.0.1:3030/36
TRACE 2022-10-19T15:13:38.922672: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.922902: proxy: Finished proxy of http://127.0.0.1:3030/32
TRACE 2022-10-19T15:13:38.923044: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.923263: proxy: Finished proxy of http://127.0.0.1:3030/34
TRACE 2022-10-19T15:13:38.923402: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.923616: proxy: Finished proxy of http://127.0.0.1:3030/33
TRACE 2022-10-19T15:13:38.923750: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.923970: proxy: Finished proxy of http://127.0.0.1:3030/35
TRACE 2022-10-19T15:13:38.924097: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.924404: proxy: Finished proxy of http://127.0.0.1:3030/44
TRACE 2022-10-19T15:13:38.924540: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.924726: proxy: Finished proxy of http://127.0.0.1:3030/42
TRACE 2022-10-19T15:13:38.924854: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.925033: proxy: Finished proxy of http://127.0.0.1:3030/40
TRACE 2022-10-19T15:13:38.925194: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.925399: proxy: Finished proxy of http://127.0.0.1:3030/39
TRACE 2022-10-19T15:13:38.925538: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.925723: proxy: Finished proxy of http://127.0.0.1:3030/41
TRACE 2022-10-19T15:13:38.925883: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.926162: proxy: Finished proxy of http://127.0.0.1:3030/37
TRACE 2022-10-19T15:13:38.926318: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.926544: proxy: Finished proxy of http://127.0.0.1:3030/38
TRACE 2022-10-19T15:13:38.926682: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.926918: proxy: Finished proxy of http://127.0.0.1:3030/43
TRACE 2022-10-19T15:13:38.927132: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.927416: proxy: Finished proxy of http://127.0.0.1:3030/49
TRACE 2022-10-19T15:13:38.927551: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.927722: proxy: Finished proxy of http://127.0.0.1:3030/46
TRACE 2022-10-19T15:13:38.927842: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.928084: proxy: Finished proxy of http://127.0.0.1:3030/47
TRACE 2022-10-19T15:13:38.928253: mio::poll: deregistering event source from poller
 INFO 2022-10-19T15:13:38.928512: proxy: Finished proxy of http://127.0.0.1:3030/48
TRACE 2022-10-19T15:13:38.928663: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929034: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929220: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929351: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929451: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929551: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929635: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929718: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929799: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929881: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.929964: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930071: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930163: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930251: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930335: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930422: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930506: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930590: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930679: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930782: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930885: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.930984: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931084: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931185: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931285: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931369: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931450: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931590: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931677: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931765: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931848: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.931931: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932014: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932097: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932180: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932263: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932347: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932429: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932525: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932644: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932741: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932847: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.932932: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933016: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933100: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933186: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933270: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933356: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933442: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933528: mio::poll: deregistering event source from poller
TRACE 2022-10-19T15:13:38.933614: mio::poll: deregistering event source from poller
```
