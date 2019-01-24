#![feature(await_macro)]
#![feature(async_await)]
#![feature(futures_api)] 

#[macro_use]
extern crate tokio;

use tokio::prelude::*;
use tokio::net::{UdpFramed, UdpSocket};

const INVITE: &'static str = r"INVITE sip:christoffer@192.168.0.230 SIP/2.0
Via: SIP/2.0/UDP 192.168.0.230:12345;rport
From: <sip:derp@192.168.0.230>;tag=1248235263
To: <sip:christoffer@192.168.0.230>
Call-ID: 6723636353
CSeq: 20 INVITE
Contact: <sip:derp@192.168.0.230>
Content-Type: application/sdp
Allow: INVITE, ACK, CANCEL, OPTIONS, BYE, REFER, NOTIFY, MESSAGE, SUBSCRIBE, INFO
Max-Forwards: 70
User-Agent: SuckSIP/0.0.1
Subject: Phone call
Content-Length:   445

v=0
o=christoffer 3367 2662 IN IP4 192.168.0.230
s=Talk
c=IN IP4 192.168.0.230
t=0 0
m=audio 7078 RTP/AVP 124 111 110 0 8 101
a=rtpmap:124 opus/48000
a=fmtp:124 useinbandfec=1; usedtx=1
a=rtpmap:111 speex/16000
a=fmtp:111 vbr=on
a=rtpmap:110 speex/8000
a=fmtp:110 vbr=on
a=rtpmap:101 telephone-event/8000
a=fmtp:101 0-11
m=video 9078 RTP/AVP 103 99
a=rtpmap:103 VP8/90000
a=rtpmap:99 MP4V-ES/90000
a=fmtp:99 profile-level-id=3
";

async fn derp() -> ()
{
    let srcaddr = "0.0.0.0:12345".parse().unwrap();
    let addr = "192.168.0.230:5060".parse().unwrap();
    let socket = match UdpSocket::bind(&srcaddr)
    {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to socket: {:?}", e);
            return;
        }
    };
    //let framed = UdpFramed::new(socket, tokio::codec::BytesCodec::new());

    let mut buffer = vec![0; 1530];

    socket.connect(&addr).unwrap();
    let send_result = await!(socket.send_dgram(INVITE, &addr));
    let socket = match send_result
    {
        Ok((socket, _)) => socket,
        Err(e) => {
            println!("Error sending: {:?}", e);
            return;
        }
    };

    println!("Sent! About to receive!");
    let result = await!(socket.recv_dgram(&mut buffer));

    println!("{:?}", result);
}

fn main()
{
    let future = derp();

    // let _ = tokio::executor::spawn(tokio_async_await::compat::backward::Compat::new(future));
    tokio::run_async(future);
}
