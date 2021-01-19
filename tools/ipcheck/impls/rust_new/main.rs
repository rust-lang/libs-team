#![feature(ip)]

use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};

fn main() {
    let input = std::env::args()
        .skip(1)
        .next()
        .expect("missing input argument");
    let addr: IpAddr = input.parse().expect("failed to parse IP");

    let data = json!({
        "to_ipv4": match addr {
            IpAddr::V4(addr) => json!(addr.to_string()),
            IpAddr::V6(addr) => json!(to_ipv4_new(&addr).map(|addr| addr.to_string())),
        },
        "to_ipv6": match addr {
            IpAddr::V4(addr) => addr.to_ipv6_mapped().to_string(),
            IpAddr::V6(addr) => addr.to_string(),
        },
        "is_unspecified": addr.is_unspecified(),
        "is_loopback": match addr {
            IpAddr::V4(addr) => addr.is_loopback(),
            // ipv6 Behavior changed (already stable)
            IpAddr::V6(addr) =>
                u128::from_be_bytes(addr.octets()) == u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets()) ||
                if let Some(v4_addr) = to_ipv4_new(&addr) { v4_addr.is_loopback() } else { false },
        },
        "is_documentation": match addr {
            IpAddr::V4(addr) => addr.is_documentation(),
            // ipv6 Behavior changed (already stable)
            IpAddr::V6(addr) =>
                ((addr.segments()[0] == 0x2001) && (addr.segments()[1] == 0xdb8)) ||
                if let Some(v4_addr) = to_ipv4_new(&addr) { v4_addr.is_documentation() } else { false }
        },
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
        "is_global": match addr {
            IpAddr::V4(addr) => addr.is_global(),
            // ipv6 Behavior changed
            IpAddr::V6(addr) =>
                (match addr.multicast_scope() {
                    Some(Ipv6MulticastScope::Global) => true,
                    None => addr.is_unicast_global(),
                    _ => false,
                }) ||
                if let Some(v4_addr) = to_ipv4_new(&addr) { v4_addr.is_global() } else { false },
        },
        "is_unicast_link_local": match addr {
            IpAddr::V4(addr) => addr.is_link_local(),
            // ipv6 Behavior changed
            IpAddr::V6(addr) =>
                (addr.segments()[0] & 0xffc0) == 0xfe80 ||
                if let Some(v4_addr) = to_ipv4_new(&addr) { v4_addr.is_link_local() } else { false },
        },
        "is_unspecified": match addr {
            IpAddr::V4(addr) => addr.is_unspecified(),
            // ipv6 Behavior changed (already stable)
            IpAddr::V6(addr) =>
                u128::from_be_bytes(addr.octets()) == u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets()) ||
                if let Some(v4_addr) = to_ipv4_new(&addr) { v4_addr.is_unspecified() } else { false },
        },
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
        "mc_scope_reserved": match addr {
            IpAddr::V4(_) => false,
            // ipv6 New behavior
            IpAddr::V6(addr) => if addr.is_multicast() {
                match addr.segments()[0] & 0x000f {
                    0 | 15 => true,
                    _ => false,
                }
            } else {
                false
            },
        },
        "mc_scope_unassigned": match addr {
            IpAddr::V4(_) => false,
            // ipv6 New behavior
            IpAddr::V6(addr) => if addr.is_multicast() {
                match addr.segments()[0] & 0x000f {
                    0 | 15 | 1 | 2 | 3 | 4 | 5 | 8 | 14 => false,
                    _ => true,
                }
            } else {
                false
            },
        },
    });

    fn to_ipv4_new(addr: &Ipv6Addr) -> Option<Ipv4Addr> {
        match addr.octets() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d] => {
                Some(Ipv4Addr::new(a, b, c, d))
            }
            _ => None,
        }
    }

    println!("{}", data);
}
