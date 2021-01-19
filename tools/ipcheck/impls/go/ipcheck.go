package main
import "net"
import "fmt"
import "log"
import "os"
import "strings"
import "encoding/json"

func main() {
    // note: only capitalized fields are public and are serialized
    type Data struct {
        ToIpv4 string
        ToIpv6 string
        IsUnspecified bool
        IsLoopback bool
        IsReserved string
        IsBenchmarking string
        IsDocumentation string
        IsGlobal bool
        IsIetfProtocolAssignment string
        IsShared string
        IsUnicastLinkLocal bool
        IsUnicastSiteLocal string
        IsUniqueLocal string
        McScopeAdminLocal string
        McScopeGlobal string
        McScopeIfaceLocal bool
        McScopeLinkLocal bool
        McScopeOrgLocal string
        McScopeRealmLocal string
        McScopeReserved string
        McScopeUnassigned string
    }

    addr := os.Args[1]
    ip := net.ParseIP(addr)

    json, err := json.Marshal(Data {
        "<unsupported>", // ToIpv4
        "<unsupported>", // ToIpv6
        ip.IsUnspecified(), // IsUnspecified
        ip.IsLoopback(), // IsLoopback
        "<unsupported>", // IsReserved
        "<unsupported>", // IsBenchmarking
        "<unsupported>", // IsDocumentation
        ip.IsGlobalUnicast(), // IsGlobal
        "<unsupported>", // IsIetfProtocolAssignment
        "<unsupported>", // IsShared
        ip.IsLinkLocalUnicast(), // IsUnicastLinkLocal
        "<unsupported>", // IsUnicastSiteLocal
        "<unsupported>", // IsUniqueLocal
        "<unsupported>", // McScopeAdminLocal
        "<unsupported>", // McScopeGlobal
        ip.IsInterfaceLocalMulticast(), // McScopeIfaceLocal
        ip.IsLinkLocalMulticast(), // McScopeLinkLocal
        "<unsupported>", // McScopeOrgLocal
        "<unsupported>", // McScopeRealmLocal
        "<unsupported>", // McScopeReserved
        "<unsupported>", // McScopeUnassigned
    })

    if err != nil {
        log.Fatal(err)
        os.Exit(1)
    }

    output := string(json)

    // normalize field names
    output = strings.Replace(output, "ToIpv4", "to_ipv4", 1)
    output = strings.Replace(output, "ToIpv6", "to_ipv6", 1)
    output = strings.Replace(output, "IsUnspecified", "is_unspecified", 1)
    output = strings.Replace(output, "IsLoopback", "is_loopback", 1)
    output = strings.Replace(output, "IsReserved", "is_reserved", 1)
    output = strings.Replace(output, "IsBenchmarking", "is_benchmarking", 1)
    output = strings.Replace(output, "IsDocumentation", "is_documentation", 1)
    output = strings.Replace(output, "IsGlobal", "is_global", 1)
    output = strings.Replace(output, "IsIetfProtocolAssignment", "is_ietf_protocol_assignment", 1)
    output = strings.Replace(output, "IsShared", "is_shared", 1)
    output = strings.Replace(output, "IsUnicastLinkLocal", "is_unicast_link_local", 1)
    output = strings.Replace(output, "IsUnicastSiteLocal", "is_unicast_site_local", 1)
    output = strings.Replace(output, "IsUniqueLocal", "is_unique_local", 1)
    output = strings.Replace(output, "McScopeAdminLocal", "mc_scope_admin_local", 1)
    output = strings.Replace(output, "McScopeGlobal", "mc_scope_global", 1)
    output = strings.Replace(output, "McScopeIfaceLocal", "mc_scope_iface_local", 1)
    output = strings.Replace(output, "McScopeLinkLocal", "mc_scope_link_local", 1)
    output = strings.Replace(output, "McScopeOrgLocal", "mc_scope_org_local", 1)
    output = strings.Replace(output, "McScopeRealmLocal", "mc_scope_realm_local", 1)
    output = strings.Replace(output, "McScopeReserved", "mc_scope_reserved", 1)
    output = strings.Replace(output, "McScopeUnassigned", "mc_scope_unassigned", 1)

    // normalize null
    output = strings.Replace(output, "\"\\u003cnil\\u003e\"", "null", -1)
    output = strings.Replace(output, "\"\"", "null", -1)

    fmt.Println(output)
}
