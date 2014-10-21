extern crate libc;
extern crate picotcp;

use std::io::timer;
use std::time::Duration;

use picotcp::pico_ip4;

fn main() {

    let my_ip_addr = pico_ip4::new("192.168.2.150");
    let my_netmask = pico_ip4::new("255.255.255.0");

    picotcp::stack_init();

    let tun_name = "tun0";
    let tun_name_cstr = tun_name.to_c_str();

    let pico_dev_eth = picotcp::tun_create(tun_name_cstr.as_ptr());

    picotcp::ipv4_link_add(pico_dev_eth, my_ip_addr, my_netmask);
    println!("tun0: ip addr is {}", my_ip_addr);

    loop {
        picotcp::stack_tick();
        timer::sleep(Duration::milliseconds(1));
    }

}

