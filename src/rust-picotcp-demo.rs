extern crate libc;
extern crate picotcp;
use picotcp::pico_ip4;
use picotcp::pico_ip6;

fn main() {
    picotcp::stack_init();
    let my_ip_addr = pico_ip4::new("192.168.2.150");
    let my_netmask = pico_ip4::new("255.255.255.0");
    let my_ip6_addr = pico_ip6 { addr:[0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] }; // Constructor is still WIP...
    let my_6_netmask = pico_ip6 { addr:[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0,  0,  0,  0,  0,  0,  0,  0] };

    let pico_dev_eth = picotcp::tun_create("tun0");
    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr, my_netmask);
    picotcp::ipv6_link_add(pico_dev_eth, my_ip6_addr, my_6_netmask);
    
    println!("tun0: ip addr is {}", my_ip_addr);
    println!("tun0: ip6 addr is {}", my_ip6_addr);
    picotcp::stack_loop();
}

