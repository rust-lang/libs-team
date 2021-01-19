#![feature(ip)]

use serde_json::json;
use std::net::{IpAddr, Ipv6MulticastScope};

fn main() {
    let input = std::env::args()
        .skip(1)
        .next()
        .expect("missing input argument");
    let addr: IpAddr = input.parse().expect("failed to parse IP");

    let data = json!({
        "to_ipv4": match addr {
            IpAddr::V4(addr) => json!(addr.to_string()),
            IpAddr::V6(addr) => json!(addr.to_ipv4().map(|addr| addr.to_string())),
        },
        "to_ipv6": match addr {
            IpAddr::V4(addr) => addr.to_ipv6_mapped().to_string(),
            IpAddr::V6(addr) => addr.to_string(),
        },
        "is_unspecified": addr.is_unspecified(),
        "is_loopback": addr.is_loopback(),
        "is_documentation": addr.is_documentation(),
        "is_shared": match addr {
            IpAddr::V4(addr) => addr.is_shared(),
            IpAddr::V6(_) => false,
        },
        "is_reserved": match addr {
            IpAddr::V4(addr) => addr.is_reserved(),
            IpAddr::V6(_) => false,
        },
        "is_ietf_protocol_assignment": match addr {
            IpAddr::V4(addr) => addr.is_ietf_protocol_assignment(),
            IpAddr::V6(_) => false,
        },
        "is_benchmarking": match addr {
            IpAddr::V4(addr) => addr.is_benchmarking(),
            IpAddr::V6(_) => false,
        },
        "is_global": addr.is_global(),
        "is_unicast_link_local": match addr {
            IpAddr::V4(addr) => addr.is_link_local(),
            IpAddr::V6(_) => false,
        },
        "is_unspecified": addr.is_unspecified(),
        "is_unique_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.is_unique_local(),
        },
        "is_unicast_site_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.is_unicast_site_local(),
        },
        "mc_scope_iface_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::InterfaceLocal),
        },
        "mc_scope_link_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::LinkLocal),
        },
        "mc_scope_realm_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::RealmLocal),
        },
        "mc_scope_admin_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::AdminLocal),
        },
        "mc_scope_org_local": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::OrganizationLocal),
        },
        "mc_scope_global": match addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => addr.multicast_scope() == Some(Ipv6MulticastScope::Global),
        },
        "mc_scope_reserved": "<unsupported>",
        "mc_scope_unassigned": "<unsupported>",
    });

    println!("{}", data);
}
