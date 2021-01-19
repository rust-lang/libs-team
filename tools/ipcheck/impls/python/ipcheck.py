#!/usr/bin/python

import sys
import ipaddress
import json

input = sys.argv[1]
addr = ipaddress.ip_address(input)

output = json.dumps({
    'to_ipv4': str(addr.ipv4_mapped) if addr.version == 6 else "<unsupported>",
    'to_ipv6': "<unsupported>",
    'is_unspecified': addr.is_unspecified,
    'is_loopback': addr.is_loopback,
    'is_reserved': addr.is_reserved,
    'is_benchmarking': "<unsupported>",
    'is_documentation': "<unsupported>",
    'is_global': addr.is_global,
    'is_ietf_protocol_assignment': "<unsupported>",
    'is_shared': "<unsupported>",
    'is_unicast_link_local': addr.is_link_local,
    'is_unicast_site_local': addr.is_site_local if addr.version == 6 else "<unsupported>",
    'is_unique_local': "<unsupported>",
    'mc_scope_admin_local': "<unsupported>",
    'mc_scope_global': "<unsupported>",
    'mc_scope_iface_local': "<unsupported>",
    'mc_scope_link_local': "<unsupported>",
    'mc_scope_org_local': "<unsupported>",
    'mc_scope_realm_local': "<unsupported>",
    'mc_scope_reserved': "<unsupported>",
    'mc_scope_unassigned': "<unsupported>",
})

# normalize output
output = output.replace("\"None\"", "null")

print(output)
