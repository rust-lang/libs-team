using System;
using System.Net;
using System.Text.Json;

class IPCheck
{
    static void Main(string[] args)
    {
        var input = args[0];
        var addr = IPAddress.Parse(input);

        if (addr.AddressFamily == System.Net.Sockets.AddressFamily.InterNetwork)
        {

            var data = new
            {
                to_ipv4 = addr.MapToIPv4().ToString(),
                to_ipv6 = addr.MapToIPv6().ToString(),
                is_unspecified = "<unsupported>",
                is_loopback = IPAddress.IsLoopback(addr),
                is_reserved = "<unsupported>",
                is_benchmarking = "<unsupported>",
                is_documentation = "<unsupported>",
                is_global = "<unsupported>",
                is_shared = "<unsupported>",
                is_unicast_link_local = "<unsupported>",
                is_unique_local = "<unsupported>",
                mc_scope_admin_local = "<unsupported>",
                mc_scope_global = "<unsupported>",
                mc_scope_iface_local = "<unsupported>",
                mc_scope_link_local = "<unsupported>",
                mc_scope_org_local = "<unsupported>",
                mc_scope_realm_local = "<unsupported>",
                mc_scope_reserved = "<unsupported>",
                mc_scope_unassigned = "<unsupported>"
            };

            Console.WriteLine("{0}", JsonSerializer.Serialize(data));
        }
        else
        {

            var data = new
            {
                to_ipv4 = addr.MapToIPv4().ToString(),
                to_ipv6 = addr.MapToIPv6().ToString(),
                is_unspecified = "<unsupported>",
                is_loopback = IPAddress.IsLoopback(addr),
                is_reserved = "<unsupported>",
                is_benchmarking = "<unsupported>",
                is_documentation = "<unsupported>",
                is_global = "<unsupported>",
                is_shared = "<unsupported>",
                is_unicast_link_local = addr.IsIPv6LinkLocal,
                is_unique_local = "<unsupported>",
                mc_scope_admin_local = "<unsupported>",
                mc_scope_global = "<unsupported>",
                mc_scope_iface_local = "<unsupported>",
                mc_scope_link_local = "<unsupported>",
                mc_scope_org_local = "<unsupported>",
                mc_scope_realm_local = "<unsupported>",
                mc_scope_reserved = "<unsupported>",
                mc_scope_unassigned = "<unsupported>"
            };

            Console.WriteLine("{0}", JsonSerializer.Serialize(data));
        }

    }
}
