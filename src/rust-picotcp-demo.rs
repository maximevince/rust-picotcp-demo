//#![feature(globs)]

extern crate libc;
extern crate picotcp;

use std::io::timer;
use std::time::Duration;

use picotcp::pico_ip4;

fn main() {

    let ipaddr = "192.168.2.150";
    let ipaddr_cstr = ipaddr.to_c_str();
    let netmask = "255.255.255.0";
    let netmask_cstr = netmask.to_c_str();

    let mut my_ip_addr = pico_ip4 { addr: 0 };
    let mut my_netmask = pico_ip4 { addr: 0 };

    picotcp::stack_init();

    let tun_name = "tun0";
    let tun_name_cstr = tun_name.to_c_str();

    let pico_dev_eth = picotcp::tun_create(tun_name_cstr.as_ptr());

    picotcp::string_to_ipv4(ipaddr_cstr.as_ptr(), &mut my_ip_addr);
    picotcp::string_to_ipv4(netmask_cstr.as_ptr(), &mut my_netmask);

    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr, my_netmask);

    loop {
        picotcp::stack_tick();
        timer::sleep(Duration::milliseconds(1));
    }

}

