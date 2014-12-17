#![feature(globs)]
extern crate libc;
extern crate picotcp;
use picotcp::pico_ip4;
use picotcp::pico_ip6;
use picotcp::socket::*;

fn socket_wakeup(ev: u16, sock: &pico_socket) {
    let mut p: u16  = 0;
    let mut a: pico_ip4 = pico_ip4{addr:0};
    match ev {
        PICO_SOCK_EV_CONN => {
            let _s: *mut pico_socket =  accept(sock, &mut a, &mut p);
            println!("Accepted connection from {}:{}", a, p);
        }
        PICO_SOCK_EV_RD => {
            let v = recv(sock);
            println!("Received {}", v);
            let w = send(sock, v.as_slice());
            println!("Sent back {} bytes", w);
        }
        PICO_SOCK_EV_WR => {
            println!("Ready to write.");
        }
        PICO_SOCK_EV_FIN=> {
            println!("Connection correctly terminated.");
        }
        PICO_SOCK_EV_CLOSE=> {
            println!("Peer closed connection. Closing...");
            close(sock);
        }
        _ =>  { 
            println!("Event ain't handled, I close this."); 
            close(sock);
        }
    }
}

fn main() {
    /* Initialize stack */
    let pico = picotcp::stack::new();

    let my_ip_addr = pico_ip4::new("192.168.2.150");
    let my_netmask = pico_ip4::new("255.255.255.0");
    let my_ip6_addr = pico_ip6 { addr:[0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] }; // Constructor is still WIP...
    let my_6_netmask = pico_ip6 { addr:[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0,  0,  0,  0,  0,  0,  0,  0] };

    let mut bind_address = pico_ip4::new("0.0.0.0");
    let mut bind_port:u16 =  7 as u16;

    let pico_dev_eth = picotcp::tun_create("tun0");
    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr.clone(), my_netmask.clone());
    picotcp::ipv6_link_add(pico_dev_eth, my_ip6_addr.clone(), my_6_netmask.clone());
    
    println!("tun0: ip addr is {}", my_ip_addr);
    println!("tun0: ip6 addr is {}", my_ip6_addr);

    let s: *mut pico_socket = picotcp::socket::socket(PICO_PROTO_IPV4,PICO_PROTO_TCP, socket_wakeup);
    let bind_tuple = bind(s, &mut bind_address, &mut bind_port);
    let mut r = bind_tuple.val0();
    let p = bind_tuple.val1();

    if r != 0 {
        panic!("Bind");
    }
    println!("Bound to port {}", p);

    r = listen(s, 10); 
    if r != 0 {
        panic!("listen");
    }


    pico.stack_loop();
}

