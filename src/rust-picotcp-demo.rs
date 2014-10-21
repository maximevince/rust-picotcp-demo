extern crate libc;
extern crate picotcp;
use picotcp::pico_ip4;

fn main() {
    picotcp::stack_init();
    let my_ip_addr = pico_ip4::new("192.168.2.150");
    let my_netmask = pico_ip4::new("255.255.255.0");
    let pico_dev_eth = picotcp::tun_create("tun0");
    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr, my_netmask);
    println!("tun0: ip addr is {}", my_ip_addr);
    picotcp::stack_loop();
}

