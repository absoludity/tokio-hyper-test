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
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/proxy`
TRACE 2022-10-18T17:10:02.684904: mio::poll: registering event source with poller: token=Token(1), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:02.685067: proxy: Starting server on 127.0.0.1:3000
```

First requests arrive at 6.598

```
TRACE 2022-10-18T17:10:06.598447: mio::poll: registering event source with poller: token=Token(2), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.598786: mio::poll: registering event source with poller: token=Token(3), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.598990: mio::poll: registering event source with poller: token=Token(4), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.599461: proxy: Start proxy of GET /3 -> <http://127.0.0.1:3030/3>
 INFO 2022-10-18T17:10:06.599462: proxy: Start proxy of GET /2 -> <http://127.0.0.1:3030/2>
TRACE 2022-10-18T17:10:06.599807: mio::poll: registering event source with poller: token=Token(5), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.600341: mio::poll: registering event source with poller: token=Token(6), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.600389: mio::poll: registering event source with poller: token=Token(7), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.600430: proxy: Start proxy of GET /1 -> <http://127.0.0.1:3030/1>
TRACE 2022-10-18T17:10:06.601161: mio::poll: registering event source with poller: token=Token(8), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.601293: proxy: Start proxy of GET /4 -> <http://127.0.0.1:3030/4>
TRACE 2022-10-18T17:10:06.602020: mio::poll: registering event source with poller: token=Token(9), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.602084: mio::poll: registering event source with poller: token=Token(10), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.602303: mio::poll: registering event source with poller: token=Token(11), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.602480: mio::poll: registering event source with poller: token=Token(12), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.602666: mio::poll: registering event source with poller: token=Token(13), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.602753: proxy: Start proxy of GET /6 -> <http://127.0.0.1:3030/6>
 INFO 2022-10-18T17:10:06.602755: proxy: Start proxy of GET /5 -> <http://127.0.0.1:3030/5>
TRACE 2022-10-18T17:10:06.603156: mio::poll: registering event source with poller: token=Token(14), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.603161: mio::poll: registering event source with poller: token=Token(15), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.603435: proxy: Start proxy of GET /7 -> <http://127.0.0.1:3030/7>
 INFO 2022-10-18T17:10:06.603477: proxy: Start proxy of GET /8 -> <http://127.0.0.1:3030/8>
TRACE 2022-10-18T17:10:06.603766: mio::poll: registering event source with poller: token=Token(16), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.604062: mio::poll: registering event source with poller: token=Token(17), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.604408: mio::poll: registering event source with poller: token=Token(18), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.604795: proxy: Start proxy of GET /9 -> <http://127.0.0.1:3030/9>
TRACE 2022-10-18T17:10:06.605120: mio::poll: registering event source with poller: token=Token(19), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.607768: mio::poll: registering event source with poller: token=Token(20), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.608316: proxy: Start proxy of GET /10 -> <http://127.0.0.1:3030/10>
TRACE 2022-10-18T17:10:06.608821: mio::poll: registering event source with poller: token=Token(21), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.610007: mio::poll: registering event source with poller: token=Token(22), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.610289: mio::poll: registering event source with poller: token=Token(23), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.610389: proxy: Start proxy of GET /11 -> <http://127.0.0.1:3030/11>
 INFO 2022-10-18T17:10:06.610647: proxy: Start proxy of GET /12 -> <http://127.0.0.1:3030/12>
TRACE 2022-10-18T17:10:06.610924: mio::poll: registering event source with poller: token=Token(24), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.611116: mio::poll: registering event source with poller: token=Token(25), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.611806: mio::poll: registering event source with poller: token=Token(26), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.612198: proxy: Start proxy of GET /13 -> <http://127.0.0.1:3030/13>
TRACE 2022-10-18T17:10:06.612538: mio::poll: registering event source with poller: token=Token(27), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.614911: mio::poll: registering event source with poller: token=Token(28), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.615081: mio::poll: registering event source with poller: token=Token(29), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.615412: proxy: Start proxy of GET /15 -> <http://127.0.0.1:3030/15>
 INFO 2022-10-18T17:10:06.615412: proxy: Start proxy of GET /14 -> <http://127.0.0.1:3030/14>
TRACE 2022-10-18T17:10:06.615702: mio::poll: registering event source with poller: token=Token(30), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.615702: mio::poll: registering event source with poller: token=Token(31), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.616294: mio::poll: registering event source with poller: token=Token(32), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.616630: proxy: Start proxy of GET /17 -> <http://127.0.0.1:3030/17>
TRACE 2022-10-18T17:10:06.616932: mio::poll: registering event source with poller: token=Token(33), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.617544: mio::poll: registering event source with poller: token=Token(34), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.617861: proxy: Start proxy of GET /18 -> <http://127.0.0.1:3030/18>
TRACE 2022-10-18T17:10:06.618126: mio::poll: registering event source with poller: token=Token(35), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.619087: mio::poll: registering event source with poller: token=Token(36), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.619404: proxy: Start proxy of GET /16 -> <http://127.0.0.1:3030/16>
TRACE 2022-10-18T17:10:06.619422: mio::poll: registering event source with poller: token=Token(37), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.620396: mio::poll: registering event source with poller: token=Token(38), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.620541: mio::poll: registering event source with poller: token=Token(39), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.620714: proxy: Start proxy of GET /19 -> <http://127.0.0.1:3030/19>
TRACE 2022-10-18T17:10:06.620937: mio::poll: registering event source with poller: token=Token(40), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.621808: proxy: Start proxy of GET /20 -> <http://127.0.0.1:3030/20>
TRACE 2022-10-18T17:10:06.622103: mio::poll: registering event source with poller: token=Token(41), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.623662: mio::poll: registering event source with poller: token=Token(42), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.623782: mio::poll: registering event source with poller: token=Token(43), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.623986: proxy: Start proxy of GET /21 -> <http://127.0.0.1:3030/21>
TRACE 2022-10-18T17:10:06.624237: mio::poll: registering event source with poller: token=Token(44), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.624570: mio::poll: registering event source with poller: token=Token(45), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.624825: proxy: Start proxy of GET /22 -> <http://127.0.0.1:3030/22>
TRACE 2022-10-18T17:10:06.625047: mio::poll: registering event source with poller: token=Token(46), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.625573: proxy: Start proxy of GET /23 -> <http://127.0.0.1:3030/23>
TRACE 2022-10-18T17:10:06.625773: mio::poll: registering event source with poller: token=Token(47), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.626175: mio::poll: registering event source with poller: token=Token(48), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.626585: proxy: Start proxy of GET /24 -> <http://127.0.0.1:3030/24>
TRACE 2022-10-18T17:10:06.626847: mio::poll: registering event source with poller: token=Token(49), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.627362: mio::poll: registering event source with poller: token=Token(50), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.627466: mio::poll: registering event source with poller: token=Token(51), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.627550: mio::poll: registering event source with poller: token=Token(52), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.627618: proxy: Start proxy of GET /25 -> <http://127.0.0.1:3030/25>
TRACE 2022-10-18T17:10:06.627630: mio::poll: registering event source with poller: token=Token(53), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.627658: proxy: Start proxy of GET /27 -> <http://127.0.0.1:3030/27>
 INFO 2022-10-18T17:10:06.627743: proxy: Start proxy of GET /28 -> <http://127.0.0.1:3030/28>
 INFO 2022-10-18T17:10:06.627823: proxy: Start proxy of GET /26 -> <http://127.0.0.1:3030/26>
TRACE 2022-10-18T17:10:06.627842: mio::poll: registering event source with poller: token=Token(54), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.627847: mio::poll: registering event source with poller: token=Token(55), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.627932: mio::poll: registering event source with poller: token=Token(56), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.628028: mio::poll: registering event source with poller: token=Token(57), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.631120: mio::poll: registering event source with poller: token=Token(58), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.631358: mio::poll: registering event source with poller: token=Token(59), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.631438: mio::poll: registering event source with poller: token=Token(60), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.631514: mio::poll: registering event source with poller: token=Token(61), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.631547: proxy: Start proxy of GET /29 -> <http://127.0.0.1:3030/29>
TRACE 2022-10-18T17:10:06.631772: mio::poll: registering event source with poller: token=Token(62), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.631956: proxy: Start proxy of GET /30 -> <http://127.0.0.1:3030/30>
TRACE 2022-10-18T17:10:06.632124: mio::poll: registering event source with poller: token=Token(63), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.632227: proxy: Start proxy of GET /31 -> <http://127.0.0.1:3030/31>
 INFO 2022-10-18T17:10:06.632284: proxy: Start proxy of GET /32 -> <http://127.0.0.1:3030/32>
TRACE 2022-10-18T17:10:06.632407: mio::poll: registering event source with poller: token=Token(64), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.632439: mio::poll: registering event source with poller: token=Token(65), interests=READABLE | WRITABLE
```

First results are returned at 6.632, ie. 632 - 598 = 34ms after they began, as would be expected/desired, while new requests (33 to 50) continue to be processed.

```
 INFO 2022-10-18T17:10:06.632972: proxy: Finished proxy of <http://127.0.0.1:3030/1>
TRACE 2022-10-18T17:10:06.633161: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.633597: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.633853: proxy: Finished proxy of <http://127.0.0.1:3030/3>
TRACE 2022-10-18T17:10:06.634034: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.634269: proxy: Finished proxy of <http://127.0.0.1:3030/2>
TRACE 2022-10-18T17:10:06.634422: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.634581: mio::poll: registering event source with poller: token=Token(16777223), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.634679: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.634812: proxy: Start proxy of GET /33 -> <http://127.0.0.1:3030/33>
TRACE 2022-10-18T17:10:06.635029: mio::poll: registering event source with poller: token=Token(16777219), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.635274: proxy: Finished proxy of <http://127.0.0.1:3030/6>
TRACE 2022-10-18T17:10:06.635411: mio::poll: registering event source with poller: token=Token(16777222), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.635457: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.635603: proxy: Start proxy of GET /36 -> <http://127.0.0.1:3030/36>
TRACE 2022-10-18T17:10:06.635781: mio::poll: registering event source with poller: token=Token(16777230), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636164: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.636234: mio::poll: registering event source with poller: token=Token(16777218), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636414: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.636428: proxy: Finished proxy of <http://127.0.0.1:3030/5>
 INFO 2022-10-18T17:10:06.636428: proxy: Finished proxy of <http://127.0.0.1:3030/7>
 INFO 2022-10-18T17:10:06.636509: proxy: Finished proxy of <http://127.0.0.1:3030/8>
TRACE 2022-10-18T17:10:06.636602: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.636604: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.636675: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.636602: proxy: Finished proxy of <http://127.0.0.1:3030/4>
 INFO 2022-10-18T17:10:06.636584: proxy: Start proxy of GET /37 -> <http://127.0.0.1:3030/37>
TRACE 2022-10-18T17:10:06.636805: mio::poll: registering event source with poller: token=Token(16777233), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636824: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.636841: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.636884: mio::poll: registering event source with poller: token=Token(16777228), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636900: mio::poll: registering event source with poller: token=Token(16777225), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636948: mio::poll: registering event source with poller: token=Token(16777232), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.636981: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.637029: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.637122: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.637130: proxy: Start proxy of GET /38 -> <http://127.0.0.1:3030/38>
 INFO 2022-10-18T17:10:06.637296: proxy: Start proxy of GET /39 -> <http://127.0.0.1:3030/39>
TRACE 2022-10-18T17:10:06.637344: mio::poll: registering event source with poller: token=Token(16777227), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.637455: mio::poll: registering event source with poller: token=Token(16777229), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.637461: proxy: Start proxy of GET /40 -> <http://127.0.0.1:3030/40>
TRACE 2022-10-18T17:10:06.637464: mio::poll: registering event source with poller: token=Token(16777220), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.637625: mio::poll: registering event source with poller: token=Token(16777231), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.637722: proxy: Start proxy of GET /35 -> <http://127.0.0.1:3030/35>
TRACE 2022-10-18T17:10:06.637881: mio::poll: registering event source with poller: token=Token(16777226), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.639811: proxy: Finished proxy of <http://127.0.0.1:3030/9>
TRACE 2022-10-18T17:10:06.640080: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.641146: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.641869: proxy: Finished proxy of <http://127.0.0.1:3030/10>
TRACE 2022-10-18T17:10:06.642112: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.642203: mio::poll: registering event source with poller: token=Token(16777237), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.642374: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.642529: proxy: Start proxy of GET /34 -> <http://127.0.0.1:3030/34>
TRACE 2022-10-18T17:10:06.642822: mio::poll: registering event source with poller: token=Token(16777236), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.644253: proxy: Finished proxy of <http://127.0.0.1:3030/11>
TRACE 2022-10-18T17:10:06.644487: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.644578: proxy: Finished proxy of <http://127.0.0.1:3030/12>
TRACE 2022-10-18T17:10:06.644746: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.644902: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.644972: proxy: Finished proxy of <http://127.0.0.1:3030/13>
TRACE 2022-10-18T17:10:06.645157: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.645507: mio::poll: registering event source with poller: token=Token(16777243), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.645543: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.645601: mio::poll: registering event source with poller: token=Token(16777238), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.645652: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.645675: mio::poll: registering event source with poller: token=Token(16777239), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.645747: mio::poll: registering event source with poller: token=Token(16777242), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.645931: proxy: Start proxy of GET /43 -> <http://127.0.0.1:3030/43>
TRACE 2022-10-18T17:10:06.646133: mio::poll: registering event source with poller: token=Token(16777240), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.646271: proxy: Start proxy of GET /45 -> <http://127.0.0.1:3030/45>
 INFO 2022-10-18T17:10:06.646281: proxy: Start proxy of GET /42 -> <http://127.0.0.1:3030/42>
 INFO 2022-10-18T17:10:06.646278: proxy: Start proxy of GET /44 -> <http://127.0.0.1:3030/44>
TRACE 2022-10-18T17:10:06.646428: mio::poll: registering event source with poller: token=Token(16777241), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.646450: mio::poll: registering event source with poller: token=Token(16777234), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.646467: mio::poll: registering event source with poller: token=Token(16777235), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.646663: mio::poll: registering event source with poller: token=Token(16777221), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.646931: proxy: Start proxy of GET /41 -> <http://127.0.0.1:3030/41>
TRACE 2022-10-18T17:10:06.647240: mio::poll: registering event source with poller: token=Token(16777224), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.648207: mio::poll: registering event source with poller: token=Token(66), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.648457: proxy: Start proxy of GET /46 -> <http://127.0.0.1:3030/46>
TRACE 2022-10-18T17:10:06.648687: mio::poll: registering event source with poller: token=Token(67), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.649041: proxy: Finished proxy of <http://127.0.0.1:3030/17>
TRACE 2022-10-18T17:10:06.649189: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.649308: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.650012: mio::poll: registering event source with poller: token=Token(16777248), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.650228: proxy: Start proxy of GET /50 -> <http://127.0.0.1:3030/50>
TRACE 2022-10-18T17:10:06.650387: mio::poll: registering event source with poller: token=Token(16777249), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.650709: proxy: Finished proxy of <http://127.0.0.1:3030/14>
 INFO 2022-10-18T17:10:06.650703: proxy: Finished proxy of <http://127.0.0.1:3030/18>
 INFO 2022-10-18T17:10:06.650719: proxy: Finished proxy of <http://127.0.0.1:3030/15>
TRACE 2022-10-18T17:10:06.650904: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.650905: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.650910: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.651002: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.651156: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.651270: mio::poll: registering event source with poller: token=Token(16777244), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.651340: mio::poll: registering event source with poller: token=Token(16777246), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.651468: proxy: Start proxy of GET /48 -> <http://127.0.0.1:3030/48>
 INFO 2022-10-18T17:10:06.651482: proxy: Start proxy of GET /49 -> <http://127.0.0.1:3030/49>
TRACE 2022-10-18T17:10:06.651658: mio::poll: registering event source with poller: token=Token(16777250), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.651656: mio::poll: registering event source with poller: token=Token(16777247), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.652007: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.652090: proxy: Finished proxy of <http://127.0.0.1:3030/16>
TRACE 2022-10-18T17:10:06.652256: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.652349: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.653869: proxy: Finished proxy of <http://127.0.0.1:3030/19>
 INFO 2022-10-18T17:10:06.653869: proxy: Finished proxy of <http://127.0.0.1:3030/20>
TRACE 2022-10-18T17:10:06.654080: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.654080: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.654210: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.654210: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.656240: mio::poll: registering event source with poller: token=Token(16777245), interests=READABLE | WRITABLE
 INFO 2022-10-18T17:10:06.656642: proxy: Start proxy of GET /47 -> <http://127.0.0.1:3030/47>
 INFO 2022-10-18T17:10:06.656764: proxy: Finished proxy of <http://127.0.0.1:3030/21>
TRACE 2022-10-18T17:10:06.656927: mio::poll: registering event source with poller: token=Token(16777253), interests=READABLE | WRITABLE
TRACE 2022-10-18T17:10:06.656966: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.657056: proxy: Finished proxy of <http://127.0.0.1:3030/23>
TRACE 2022-10-18T17:10:06.657080: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.657092: proxy: Finished proxy of <http://127.0.0.1:3030/22>
TRACE 2022-10-18T17:10:06.657250: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.657249: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.657360: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.657360: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.659915: proxy: Finished proxy of <http://127.0.0.1:3030/24>
 INFO 2022-10-18T17:10:06.659915: proxy: Finished proxy of <http://127.0.0.1:3030/27>
 INFO 2022-10-18T17:10:06.659920: proxy: Finished proxy of <http://127.0.0.1:3030/28>
 INFO 2022-10-18T17:10:06.659915: proxy: Finished proxy of <http://127.0.0.1:3030/26>
 INFO 2022-10-18T17:10:06.659915: proxy: Finished proxy of <http://127.0.0.1:3030/25>
TRACE 2022-10-18T17:10:06.660119: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660119: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660119: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660131: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660119: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660289: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.660289: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.661240: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.662161: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.663261: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.664094: proxy: Finished proxy of <http://127.0.0.1:3030/29>
TRACE 2022-10-18T17:10:06.664353: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.664485: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.666064: proxy: Finished proxy of <http://127.0.0.1:3030/31>
 INFO 2022-10-18T17:10:06.666065: proxy: Finished proxy of <http://127.0.0.1:3030/30>
 INFO 2022-10-18T17:10:06.666064: proxy: Finished proxy of <http://127.0.0.1:3030/32>
TRACE 2022-10-18T17:10:06.666241: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.666241: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.666241: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.666730: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.666740: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.666807: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.667958: proxy: Finished proxy of <http://127.0.0.1:3030/33>
TRACE 2022-10-18T17:10:06.668206: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.668357: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.669872: proxy: Finished proxy of <http://127.0.0.1:3030/37>
 INFO 2022-10-18T17:10:06.669872: proxy: Finished proxy of <http://127.0.0.1:3030/35>
 INFO 2022-10-18T17:10:06.669907: proxy: Finished proxy of <http://127.0.0.1:3030/40>
 INFO 2022-10-18T17:10:06.669924: proxy: Finished proxy of <http://127.0.0.1:3030/36>
 INFO 2022-10-18T17:10:06.669955: proxy: Finished proxy of <http://127.0.0.1:3030/39>
 INFO 2022-10-18T17:10:06.669952: proxy: Finished proxy of <http://127.0.0.1:3030/38>
TRACE 2022-10-18T17:10:06.670068: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670068: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670078: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670103: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670113: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670717: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670722: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670744: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670758: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670847: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.670068: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.673957: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.676046: proxy: Finished proxy of <http://127.0.0.1:3030/34>
TRACE 2022-10-18T17:10:06.676316: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.676451: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.678084: proxy: Finished proxy of <http://127.0.0.1:3030/44>
 INFO 2022-10-18T17:10:06.678084: proxy: Finished proxy of <http://127.0.0.1:3030/42>
 INFO 2022-10-18T17:10:06.678084: proxy: Finished proxy of <http://127.0.0.1:3030/43>
 INFO 2022-10-18T17:10:06.678084: proxy: Finished proxy of <http://127.0.0.1:3030/45>
TRACE 2022-10-18T17:10:06.678311: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.678311: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.678311: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.678311: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.678480: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.678758: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.678829: proxy: Finished proxy of <http://127.0.0.1:3030/41>
TRACE 2022-10-18T17:10:06.679031: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.679229: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.679567: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.679781: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.680712: proxy: Finished proxy of <http://127.0.0.1:3030/46>
TRACE 2022-10-18T17:10:06.680902: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.681056: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.682553: proxy: Finished proxy of <http://127.0.0.1:3030/50>
TRACE 2022-10-18T17:10:06.682920: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.683061: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.683317: proxy: Finished proxy of <http://127.0.0.1:3030/48>
 INFO 2022-10-18T17:10:06.683317: proxy: Finished proxy of <http://127.0.0.1:3030/49>
TRACE 2022-10-18T17:10:06.683513: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.683513: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.683636: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.683667: mio::poll: deregistering event source from poller
 INFO 2022-10-18T17:10:06.689368: proxy: Finished proxy of <http://127.0.0.1:3030/47>
TRACE 2022-10-18T17:10:06.689593: mio::poll: deregistering event source from poller
TRACE 2022-10-18T17:10:06.689751: mio::poll: deregistering event source from poller
```
