#![feature(globs)]
extern crate libc;
extern crate picotcp;
use picotcp::pico_ip4;
use picotcp::pico_ip6;
use picotcp::pico_dhcp_server::*;


fn main() {
    /* Initialize stack */
    let pico = picotcp::stack::new();

    let my_ip_addr = pico_ip4::new("192.168.2.150");
    let my_netmask = pico_ip4::new("255.255.255.0");
    let my_ip6_addr = pico_ip6 { addr:[0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] }; // Constructor is still WIP...
    let my_6_netmask = pico_ip6 { addr:[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0,  0,  0,  0,  0,  0,  0,  0] };

    let dhcp_start = pico_ip4::new("192.168.2.1");
    let dhcp_end = pico_ip4::new("192.168.2.10");

    let pico_dev_eth = picotcp::tap_create("tap0");
    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr.clone(), my_netmask.clone());
    picotcp::ipv6_link_add(pico_dev_eth, my_ip6_addr.clone(), my_6_netmask.clone());
    
    println!("tap0: ip addr is {}", my_ip_addr.clone());
    println!("tap0: ip6 addr is {}", my_ip6_addr);
    
    let mut settings : pico_dhcp_server_setting = pico_dhcp_server_setting {
        pool_start: dhcp_start.clone(),
        pool_next: dhcp_start,
        pool_end: dhcp_end,
        lease_time: 0,
        dev: pico_dev_eth,
        s: 0,
        server_ip: my_ip_addr,
        netmask: my_netmask,
        flags: 0
    };

    dhcp_server_initiate(&mut settings);

    pico.stack_loop();
}

