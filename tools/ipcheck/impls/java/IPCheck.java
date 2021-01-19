import java.net.InetAddress;

public class IpCheck {
     public static void main(String []args) throws Exception {
        String input = args[0];
        InetAddress addr = InetAddress.getByName(input);

        String json = new StringBuilder()
            .append("{")
            .append("\"to_ipv4\":").append("\"<unsupported>\"").append(",")
            .append("\"to_ipv6\":").append("\"<unsupported>\"").append(",")
            .append("\"is_unspecified\":").append("\"<unsupported>\"").append(",")
            .append("\"is_loopback\":").append(addr.isLoopbackAddress()).append(",")
            .append("\"is_reserved\":").append("\"<unsupported>\"").append(",")
            .append("\"is_benchmarking\":").append("\"<unsupported>\"").append(",")
            .append("\"is_documentation\":").append("\"<unsupported>\"").append(",")
            .append("\"is_global\":").append("\"<unsupported>\"").append(",")
            .append("\"is_ietf_protocol_assignment\":").append("\"<unsupported>\"").append(",")
            .append("\"is_shared\":").append("\"<unsupported>\"").append(",")
            .append("\"is_unicast_link_local\":").append("\"<unsupported>\"").append(",")
            .append("\"is_unicast_site_local\":").append("\"<unsupported>\"").append(",")
            .append("\"is_unique_local\":").append("\"<unsupported>\"").append(",")
            .append("\"mc_scope_admin_local\":").append("\"<unsupported>\"").append(",")
            .append("\"mc_scope_global\":").append(addr.isMCGlobal()).append(",")
            .append("\"mc_scope_iface_local\":").append("\"<unsupported>\"").append(",")
            .append("\"mc_scope_link_local\":").append(addr.isMCLinkLocal()).append(",")
            .append("\"mc_scope_org_local\":").append(addr.isMCOrgLocal()).append(",")
            .append("\"mc_scope_realm_local\":").append("\"<unsupported>\"").append(",")
            .append("\"mc_scope_reserved\":").append("\"<unsupported>\"").append(",")
            .append("\"mc_scope_unassigned\":").append("\"<unsupported>\"")
            .append("}")
            .toString();

        System.out.print(json);
     }
}
