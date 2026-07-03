#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceHint { pub port: u16, pub proto: &'static str, pub name: &'static str, pub risk: &'static str }

pub const SERVICE_HINTS: &[ServiceHint] = &[
    ServiceHint { port: 20, proto: "tcp", name: "ftp-data", risk: "standard" },
    ServiceHint { port: 21, proto: "tcp", name: "ftp", risk: "standard" },
    ServiceHint { port: 22, proto: "tcp", name: "ssh", risk: "admin" },
    ServiceHint { port: 23, proto: "tcp", name: "telnet", risk: "admin" },
    ServiceHint { port: 25, proto: "tcp", name: "smtp", risk: "standard" },
    ServiceHint { port: 53, proto: "tcp", name: "dns", risk: "standard" },
    ServiceHint { port: 53, proto: "udp", name: "dns", risk: "standard" },
    ServiceHint { port: 67, proto: "tcp", name: "dhcp-server", risk: "standard" },
    ServiceHint { port: 67, proto: "udp", name: "dhcp-server", risk: "standard" },
    ServiceHint { port: 68, proto: "tcp", name: "dhcp-client", risk: "standard" },
    ServiceHint { port: 68, proto: "udp", name: "dhcp-client", risk: "standard" },
    ServiceHint { port: 69, proto: "tcp", name: "tftp", risk: "standard" },
    ServiceHint { port: 69, proto: "udp", name: "tftp", risk: "standard" },
    ServiceHint { port: 80, proto: "tcp", name: "http", risk: "standard" },
    ServiceHint { port: 110, proto: "tcp", name: "pop3", risk: "standard" },
    ServiceHint { port: 123, proto: "tcp", name: "ntp", risk: "standard" },
    ServiceHint { port: 123, proto: "udp", name: "ntp", risk: "standard" },
    ServiceHint { port: 135, proto: "tcp", name: "msrpc", risk: "standard" },
    ServiceHint { port: 137, proto: "tcp", name: "netbios-ns", risk: "standard" },
    ServiceHint { port: 138, proto: "tcp", name: "netbios-dgm", risk: "standard" },
    ServiceHint { port: 139, proto: "tcp", name: "netbios-ssn", risk: "standard" },
    ServiceHint { port: 143, proto: "tcp", name: "imap", risk: "standard" },
    ServiceHint { port: 161, proto: "tcp", name: "snmp", risk: "standard" },
    ServiceHint { port: 161, proto: "udp", name: "snmp", risk: "standard" },
    ServiceHint { port: 162, proto: "tcp", name: "snmptrap", risk: "standard" },
    ServiceHint { port: 162, proto: "udp", name: "snmptrap", risk: "standard" },
    ServiceHint { port: 389, proto: "tcp", name: "ldap", risk: "standard" },
    ServiceHint { port: 443, proto: "tcp", name: "https", risk: "standard" },
    ServiceHint { port: 445, proto: "tcp", name: "smb", risk: "standard" },
    ServiceHint { port: 465, proto: "tcp", name: "smtps", risk: "standard" },
    ServiceHint { port: 514, proto: "tcp", name: "syslog", risk: "standard" },
    ServiceHint { port: 514, proto: "udp", name: "syslog", risk: "standard" },
    ServiceHint { port: 587, proto: "tcp", name: "smtp-submit", risk: "standard" },
    ServiceHint { port: 636, proto: "tcp", name: "ldaps", risk: "standard" },
    ServiceHint { port: 853, proto: "tcp", name: "dns-over-tls", risk: "standard" },
    ServiceHint { port: 993, proto: "tcp", name: "imaps", risk: "standard" },
    ServiceHint { port: 995, proto: "tcp", name: "pop3s", risk: "standard" },
    ServiceHint { port: 1433, proto: "tcp", name: "mssql", risk: "standard" },
    ServiceHint { port: 1521, proto: "tcp", name: "oracle", risk: "standard" },
    ServiceHint { port: 2049, proto: "tcp", name: "nfs", risk: "standard" },
    ServiceHint { port: 3306, proto: "tcp", name: "mysql", risk: "standard" },
    ServiceHint { port: 3389, proto: "tcp", name: "rdp", risk: "admin" },
    ServiceHint { port: 5432, proto: "tcp", name: "postgres", risk: "standard" },
    ServiceHint { port: 5900, proto: "tcp", name: "vnc", risk: "admin" },
    ServiceHint { port: 6379, proto: "tcp", name: "redis", risk: "admin" },
    ServiceHint { port: 8080, proto: "tcp", name: "http-alt", risk: "standard" },
    ServiceHint { port: 8443, proto: "tcp", name: "https-alt", risk: "standard" },
    ServiceHint { port: 9200, proto: "tcp", name: "elasticsearch", risk: "admin" },
    ServiceHint { port: 9300, proto: "tcp", name: "elasticsearch-transport", risk: "admin" },
    ServiceHint { port: 11211, proto: "tcp", name: "memcached", risk: "admin" },
    ServiceHint { port: 27017, proto: "tcp", name: "mongodb", risk: "admin" },
    ServiceHint { port: 1, proto: "tcp", name: "allocated-port-1", risk: "privileged" },
    ServiceHint { port: 2, proto: "tcp", name: "allocated-port-2", risk: "privileged" },
    ServiceHint { port: 3, proto: "tcp", name: "allocated-port-3", risk: "privileged" },
    ServiceHint { port: 4, proto: "tcp", name: "allocated-port-4", risk: "privileged" },
    ServiceHint { port: 5, proto: "tcp", name: "allocated-port-5", risk: "privileged" },
    ServiceHint { port: 6, proto: "tcp", name: "allocated-port-6", risk: "privileged" },
    ServiceHint { port: 7, proto: "tcp", name: "allocated-port-7", risk: "privileged" },
    ServiceHint { port: 8, proto: "tcp", name: "allocated-port-8", risk: "privileged" },
    ServiceHint { port: 9, proto: "tcp", name: "allocated-port-9", risk: "privileged" },
    ServiceHint { port: 10, proto: "tcp", name: "allocated-port-10", risk: "privileged" },
    ServiceHint { port: 11, proto: "tcp", name: "allocated-port-11", risk: "privileged" },
    ServiceHint { port: 12, proto: "tcp", name: "allocated-port-12", risk: "privileged" },
    ServiceHint { port: 13, proto: "tcp", name: "allocated-port-13", risk: "privileged" },
    ServiceHint { port: 14, proto: "tcp", name: "allocated-port-14", risk: "privileged" },
    ServiceHint { port: 15, proto: "tcp", name: "allocated-port-15", risk: "privileged" },
    ServiceHint { port: 16, proto: "tcp", name: "allocated-port-16", risk: "privileged" },
    ServiceHint { port: 17, proto: "tcp", name: "allocated-port-17", risk: "privileged" },
    ServiceHint { port: 18, proto: "tcp", name: "allocated-port-18", risk: "privileged" },
    ServiceHint { port: 19, proto: "tcp", name: "allocated-port-19", risk: "privileged" },
    ServiceHint { port: 24, proto: "tcp", name: "allocated-port-24", risk: "privileged" },
    ServiceHint { port: 26, proto: "tcp", name: "allocated-port-26", risk: "privileged" },
    ServiceHint { port: 27, proto: "tcp", name: "allocated-port-27", risk: "privileged" },
    ServiceHint { port: 28, proto: "tcp", name: "allocated-port-28", risk: "privileged" },
    ServiceHint { port: 29, proto: "tcp", name: "allocated-port-29", risk: "privileged" },
    ServiceHint { port: 30, proto: "tcp", name: "allocated-port-30", risk: "privileged" },
    ServiceHint { port: 31, proto: "tcp", name: "allocated-port-31", risk: "privileged" },
    ServiceHint { port: 32, proto: "tcp", name: "allocated-port-32", risk: "privileged" },
    ServiceHint { port: 33, proto: "tcp", name: "allocated-port-33", risk: "privileged" },
    ServiceHint { port: 34, proto: "tcp", name: "allocated-port-34", risk: "privileged" },
    ServiceHint { port: 35, proto: "tcp", name: "allocated-port-35", risk: "privileged" },
    ServiceHint { port: 36, proto: "tcp", name: "allocated-port-36", risk: "privileged" },
    ServiceHint { port: 37, proto: "tcp", name: "allocated-port-37", risk: "privileged" },
    ServiceHint { port: 38, proto: "tcp", name: "allocated-port-38", risk: "privileged" },
    ServiceHint { port: 39, proto: "tcp", name: "allocated-port-39", risk: "privileged" },
    ServiceHint { port: 40, proto: "tcp", name: "allocated-port-40", risk: "privileged" },
    ServiceHint { port: 41, proto: "tcp", name: "allocated-port-41", risk: "privileged" },
    ServiceHint { port: 42, proto: "tcp", name: "allocated-port-42", risk: "privileged" },
    ServiceHint { port: 43, proto: "tcp", name: "allocated-port-43", risk: "privileged" },
    ServiceHint { port: 44, proto: "tcp", name: "allocated-port-44", risk: "privileged" },
    ServiceHint { port: 45, proto: "tcp", name: "allocated-port-45", risk: "privileged" },
    ServiceHint { port: 46, proto: "tcp", name: "allocated-port-46", risk: "privileged" },
    ServiceHint { port: 47, proto: "tcp", name: "allocated-port-47", risk: "privileged" },
    ServiceHint { port: 48, proto: "tcp", name: "allocated-port-48", risk: "privileged" },
    ServiceHint { port: 49, proto: "tcp", name: "allocated-port-49", risk: "privileged" },
    ServiceHint { port: 50, proto: "tcp", name: "allocated-port-50", risk: "privileged" },
    ServiceHint { port: 51, proto: "tcp", name: "allocated-port-51", risk: "privileged" },
    ServiceHint { port: 52, proto: "tcp", name: "allocated-port-52", risk: "privileged" },
    ServiceHint { port: 54, proto: "tcp", name: "allocated-port-54", risk: "privileged" },
    ServiceHint { port: 55, proto: "tcp", name: "allocated-port-55", risk: "privileged" },
    ServiceHint { port: 56, proto: "tcp", name: "allocated-port-56", risk: "privileged" },
    ServiceHint { port: 57, proto: "tcp", name: "allocated-port-57", risk: "privileged" },
    ServiceHint { port: 58, proto: "tcp", name: "allocated-port-58", risk: "privileged" },
    ServiceHint { port: 59, proto: "tcp", name: "allocated-port-59", risk: "privileged" },
    ServiceHint { port: 60, proto: "tcp", name: "allocated-port-60", risk: "privileged" },
    ServiceHint { port: 61, proto: "tcp", name: "allocated-port-61", risk: "privileged" },
    ServiceHint { port: 62, proto: "tcp", name: "allocated-port-62", risk: "privileged" },
    ServiceHint { port: 63, proto: "tcp", name: "allocated-port-63", risk: "privileged" },
    ServiceHint { port: 64, proto: "tcp", name: "allocated-port-64", risk: "privileged" },
    ServiceHint { port: 65, proto: "tcp", name: "allocated-port-65", risk: "privileged" },
    ServiceHint { port: 66, proto: "tcp", name: "allocated-port-66", risk: "privileged" },
    ServiceHint { port: 70, proto: "tcp", name: "allocated-port-70", risk: "privileged" },
    ServiceHint { port: 71, proto: "tcp", name: "allocated-port-71", risk: "privileged" },
    ServiceHint { port: 72, proto: "tcp", name: "allocated-port-72", risk: "privileged" },
    ServiceHint { port: 73, proto: "tcp", name: "allocated-port-73", risk: "privileged" },
    ServiceHint { port: 74, proto: "tcp", name: "allocated-port-74", risk: "privileged" },
    ServiceHint { port: 75, proto: "tcp", name: "allocated-port-75", risk: "privileged" },
    ServiceHint { port: 76, proto: "tcp", name: "allocated-port-76", risk: "privileged" },
    ServiceHint { port: 77, proto: "tcp", name: "allocated-port-77", risk: "privileged" },
    ServiceHint { port: 78, proto: "tcp", name: "allocated-port-78", risk: "privileged" },
    ServiceHint { port: 79, proto: "tcp", name: "allocated-port-79", risk: "privileged" },
    ServiceHint { port: 81, proto: "tcp", name: "allocated-port-81", risk: "privileged" },
    ServiceHint { port: 82, proto: "tcp", name: "allocated-port-82", risk: "privileged" },
    ServiceHint { port: 83, proto: "tcp", name: "allocated-port-83", risk: "privileged" },
    ServiceHint { port: 84, proto: "tcp", name: "allocated-port-84", risk: "privileged" },
    ServiceHint { port: 85, proto: "tcp", name: "allocated-port-85", risk: "privileged" },
    ServiceHint { port: 86, proto: "tcp", name: "allocated-port-86", risk: "privileged" },
    ServiceHint { port: 87, proto: "tcp", name: "allocated-port-87", risk: "privileged" },
    ServiceHint { port: 88, proto: "tcp", name: "allocated-port-88", risk: "privileged" },
    ServiceHint { port: 89, proto: "tcp", name: "allocated-port-89", risk: "privileged" },
    ServiceHint { port: 90, proto: "tcp", name: "allocated-port-90", risk: "privileged" },
    ServiceHint { port: 91, proto: "tcp", name: "allocated-port-91", risk: "privileged" },
    ServiceHint { port: 92, proto: "tcp", name: "allocated-port-92", risk: "privileged" },
    ServiceHint { port: 93, proto: "tcp", name: "allocated-port-93", risk: "privileged" },
    ServiceHint { port: 94, proto: "tcp", name: "allocated-port-94", risk: "privileged" },
    ServiceHint { port: 95, proto: "tcp", name: "allocated-port-95", risk: "privileged" },
    ServiceHint { port: 96, proto: "tcp", name: "allocated-port-96", risk: "privileged" },
    ServiceHint { port: 97, proto: "tcp", name: "allocated-port-97", risk: "privileged" },
    ServiceHint { port: 98, proto: "tcp", name: "allocated-port-98", risk: "privileged" },
    ServiceHint { port: 99, proto: "tcp", name: "allocated-port-99", risk: "privileged" },
    ServiceHint { port: 100, proto: "tcp", name: "allocated-port-100", risk: "privileged" },
    ServiceHint { port: 101, proto: "tcp", name: "allocated-port-101", risk: "privileged" },
    ServiceHint { port: 102, proto: "tcp", name: "allocated-port-102", risk: "privileged" },
    ServiceHint { port: 103, proto: "tcp", name: "allocated-port-103", risk: "privileged" },
    ServiceHint { port: 104, proto: "tcp", name: "allocated-port-104", risk: "privileged" },
    ServiceHint { port: 105, proto: "tcp", name: "allocated-port-105", risk: "privileged" },
    ServiceHint { port: 106, proto: "tcp", name: "allocated-port-106", risk: "privileged" },
    ServiceHint { port: 107, proto: "tcp", name: "allocated-port-107", risk: "privileged" },
    ServiceHint { port: 108, proto: "tcp", name: "allocated-port-108", risk: "privileged" },
    ServiceHint { port: 109, proto: "tcp", name: "allocated-port-109", risk: "privileged" },
    ServiceHint { port: 111, proto: "tcp", name: "allocated-port-111", risk: "privileged" },
    ServiceHint { port: 112, proto: "tcp", name: "allocated-port-112", risk: "privileged" },
    ServiceHint { port: 113, proto: "tcp", name: "allocated-port-113", risk: "privileged" },
    ServiceHint { port: 114, proto: "tcp", name: "allocated-port-114", risk: "privileged" },
    ServiceHint { port: 115, proto: "tcp", name: "allocated-port-115", risk: "privileged" },
    ServiceHint { port: 116, proto: "tcp", name: "allocated-port-116", risk: "privileged" },
    ServiceHint { port: 117, proto: "tcp", name: "allocated-port-117", risk: "privileged" },
    ServiceHint { port: 118, proto: "tcp", name: "allocated-port-118", risk: "privileged" },
    ServiceHint { port: 119, proto: "tcp", name: "allocated-port-119", risk: "privileged" },
    ServiceHint { port: 120, proto: "tcp", name: "allocated-port-120", risk: "privileged" },
    ServiceHint { port: 121, proto: "tcp", name: "allocated-port-121", risk: "privileged" },
    ServiceHint { port: 122, proto: "tcp", name: "allocated-port-122", risk: "privileged" },
    ServiceHint { port: 124, proto: "tcp", name: "allocated-port-124", risk: "privileged" },
    ServiceHint { port: 125, proto: "tcp", name: "allocated-port-125", risk: "privileged" },
    ServiceHint { port: 126, proto: "tcp", name: "allocated-port-126", risk: "privileged" },
    ServiceHint { port: 127, proto: "tcp", name: "allocated-port-127", risk: "privileged" },
    ServiceHint { port: 132, proto: "tcp", name: "allocated-port-132", risk: "privileged" },
    ServiceHint { port: 133, proto: "tcp", name: "allocated-port-133", risk: "privileged" },
    ServiceHint { port: 140, proto: "tcp", name: "allocated-port-140", risk: "privileged" },
    ServiceHint { port: 147, proto: "tcp", name: "allocated-port-147", risk: "privileged" },
    ServiceHint { port: 154, proto: "tcp", name: "allocated-port-154", risk: "privileged" },
    ServiceHint { port: 165, proto: "tcp", name: "allocated-port-165", risk: "privileged" },
    ServiceHint { port: 168, proto: "tcp", name: "allocated-port-168", risk: "privileged" },
    ServiceHint { port: 175, proto: "tcp", name: "allocated-port-175", risk: "privileged" },
    ServiceHint { port: 176, proto: "tcp", name: "allocated-port-176", risk: "privileged" },
    ServiceHint { port: 182, proto: "tcp", name: "allocated-port-182", risk: "privileged" },
    ServiceHint { port: 187, proto: "tcp", name: "allocated-port-187", risk: "privileged" },
    ServiceHint { port: 189, proto: "tcp", name: "allocated-port-189", risk: "privileged" },
    ServiceHint { port: 196, proto: "tcp", name: "allocated-port-196", risk: "privileged" },
    ServiceHint { port: 198, proto: "tcp", name: "allocated-port-198", risk: "privileged" },
    ServiceHint { port: 203, proto: "tcp", name: "allocated-port-203", risk: "privileged" },
    ServiceHint { port: 209, proto: "tcp", name: "allocated-port-209", risk: "privileged" },
    ServiceHint { port: 210, proto: "tcp", name: "allocated-port-210", risk: "privileged" },
    ServiceHint { port: 217, proto: "tcp", name: "allocated-port-217", risk: "privileged" },
    ServiceHint { port: 220, proto: "tcp", name: "allocated-port-220", risk: "privileged" },
    ServiceHint { port: 224, proto: "tcp", name: "allocated-port-224", risk: "privileged" },
    ServiceHint { port: 231, proto: "tcp", name: "allocated-port-231", risk: "privileged" },
    ServiceHint { port: 238, proto: "tcp", name: "allocated-port-238", risk: "privileged" },
    ServiceHint { port: 242, proto: "tcp", name: "allocated-port-242", risk: "privileged" },
    ServiceHint { port: 245, proto: "tcp", name: "allocated-port-245", risk: "privileged" },
    ServiceHint { port: 252, proto: "tcp", name: "allocated-port-252", risk: "privileged" },
    ServiceHint { port: 253, proto: "tcp", name: "allocated-port-253", risk: "privileged" },
    ServiceHint { port: 259, proto: "tcp", name: "allocated-port-259", risk: "privileged" },
    ServiceHint { port: 264, proto: "tcp", name: "allocated-port-264", risk: "privileged" },
    ServiceHint { port: 266, proto: "tcp", name: "allocated-port-266", risk: "privileged" },
    ServiceHint { port: 273, proto: "tcp", name: "allocated-port-273", risk: "privileged" },
    ServiceHint { port: 275, proto: "tcp", name: "allocated-port-275", risk: "privileged" },
    ServiceHint { port: 280, proto: "tcp", name: "allocated-port-280", risk: "privileged" },
    ServiceHint { port: 286, proto: "tcp", name: "allocated-port-286", risk: "privileged" },
    ServiceHint { port: 287, proto: "tcp", name: "allocated-port-287", risk: "privileged" },
    ServiceHint { port: 294, proto: "tcp", name: "allocated-port-294", risk: "privileged" },
    ServiceHint { port: 297, proto: "tcp", name: "allocated-port-297", risk: "privileged" },
    ServiceHint { port: 301, proto: "tcp", name: "allocated-port-301", risk: "privileged" },
    ServiceHint { port: 308, proto: "tcp", name: "allocated-port-308", risk: "privileged" },
    ServiceHint { port: 315, proto: "tcp", name: "allocated-port-315", risk: "privileged" },
    ServiceHint { port: 319, proto: "tcp", name: "allocated-port-319", risk: "privileged" },
    ServiceHint { port: 322, proto: "tcp", name: "allocated-port-322", risk: "privileged" },
    ServiceHint { port: 329, proto: "tcp", name: "allocated-port-329", risk: "privileged" },
    ServiceHint { port: 330, proto: "tcp", name: "allocated-port-330", risk: "privileged" },
    ServiceHint { port: 336, proto: "tcp", name: "allocated-port-336", risk: "privileged" },
    ServiceHint { port: 341, proto: "tcp", name: "allocated-port-341", risk: "privileged" },
    ServiceHint { port: 343, proto: "tcp", name: "allocated-port-343", risk: "privileged" },
    ServiceHint { port: 350, proto: "tcp", name: "allocated-port-350", risk: "privileged" },
    ServiceHint { port: 352, proto: "tcp", name: "allocated-port-352", risk: "privileged" },
    ServiceHint { port: 357, proto: "tcp", name: "allocated-port-357", risk: "privileged" },
    ServiceHint { port: 363, proto: "tcp", name: "allocated-port-363", risk: "privileged" },
    ServiceHint { port: 364, proto: "tcp", name: "allocated-port-364", risk: "privileged" },
    ServiceHint { port: 371, proto: "tcp", name: "allocated-port-371", risk: "privileged" },
    ServiceHint { port: 374, proto: "tcp", name: "allocated-port-374", risk: "privileged" },
    ServiceHint { port: 378, proto: "tcp", name: "allocated-port-378", risk: "privileged" },
    ServiceHint { port: 385, proto: "tcp", name: "allocated-port-385", risk: "privileged" },
    ServiceHint { port: 392, proto: "tcp", name: "allocated-port-392", risk: "privileged" },
    ServiceHint { port: 396, proto: "tcp", name: "allocated-port-396", risk: "privileged" },
    ServiceHint { port: 399, proto: "tcp", name: "allocated-port-399", risk: "privileged" },
    ServiceHint { port: 406, proto: "tcp", name: "allocated-port-406", risk: "privileged" },
    ServiceHint { port: 407, proto: "tcp", name: "allocated-port-407", risk: "privileged" },
    ServiceHint { port: 413, proto: "tcp", name: "allocated-port-413", risk: "privileged" },
    ServiceHint { port: 418, proto: "tcp", name: "allocated-port-418", risk: "privileged" },
    ServiceHint { port: 420, proto: "tcp", name: "allocated-port-420", risk: "privileged" },
    ServiceHint { port: 427, proto: "tcp", name: "allocated-port-427", risk: "privileged" },
    ServiceHint { port: 429, proto: "tcp", name: "allocated-port-429", risk: "privileged" },
    ServiceHint { port: 434, proto: "tcp", name: "allocated-port-434", risk: "privileged" },
    ServiceHint { port: 440, proto: "tcp", name: "allocated-port-440", risk: "privileged" },
    ServiceHint { port: 441, proto: "tcp", name: "allocated-port-441", risk: "privileged" },
    ServiceHint { port: 448, proto: "tcp", name: "allocated-port-448", risk: "privileged" },
    ServiceHint { port: 451, proto: "tcp", name: "allocated-port-451", risk: "privileged" },
    ServiceHint { port: 455, proto: "tcp", name: "allocated-port-455", risk: "privileged" },
    ServiceHint { port: 462, proto: "tcp", name: "allocated-port-462", risk: "privileged" },
    ServiceHint { port: 469, proto: "tcp", name: "allocated-port-469", risk: "privileged" },
    ServiceHint { port: 473, proto: "tcp", name: "allocated-port-473", risk: "privileged" },
    ServiceHint { port: 476, proto: "tcp", name: "allocated-port-476", risk: "privileged" },
    ServiceHint { port: 483, proto: "tcp", name: "allocated-port-483", risk: "privileged" },
    ServiceHint { port: 484, proto: "tcp", name: "allocated-port-484", risk: "privileged" },
    ServiceHint { port: 490, proto: "tcp", name: "allocated-port-490", risk: "privileged" },
    ServiceHint { port: 495, proto: "tcp", name: "allocated-port-495", risk: "privileged" },
    ServiceHint { port: 497, proto: "tcp", name: "allocated-port-497", risk: "privileged" },
    ServiceHint { port: 504, proto: "tcp", name: "allocated-port-504", risk: "privileged" },
    ServiceHint { port: 506, proto: "tcp", name: "allocated-port-506", risk: "privileged" },
    ServiceHint { port: 511, proto: "tcp", name: "allocated-port-511", risk: "privileged" },
    ServiceHint { port: 517, proto: "tcp", name: "allocated-port-517", risk: "privileged" },
    ServiceHint { port: 518, proto: "tcp", name: "allocated-port-518", risk: "privileged" },
    ServiceHint { port: 525, proto: "tcp", name: "allocated-port-525", risk: "privileged" },
    ServiceHint { port: 528, proto: "tcp", name: "allocated-port-528", risk: "privileged" },
    ServiceHint { port: 532, proto: "tcp", name: "allocated-port-532", risk: "privileged" },
    ServiceHint { port: 539, proto: "tcp", name: "allocated-port-539", risk: "privileged" },
    ServiceHint { port: 546, proto: "tcp", name: "allocated-port-546", risk: "privileged" },
    ServiceHint { port: 550, proto: "tcp", name: "allocated-port-550", risk: "privileged" },
    ServiceHint { port: 553, proto: "tcp", name: "allocated-port-553", risk: "privileged" },
    ServiceHint { port: 560, proto: "tcp", name: "allocated-port-560", risk: "privileged" },
    ServiceHint { port: 561, proto: "tcp", name: "allocated-port-561", risk: "privileged" },
    ServiceHint { port: 567, proto: "tcp", name: "allocated-port-567", risk: "privileged" },
    ServiceHint { port: 572, proto: "tcp", name: "allocated-port-572", risk: "privileged" },
    ServiceHint { port: 574, proto: "tcp", name: "allocated-port-574", risk: "privileged" },
    ServiceHint { port: 581, proto: "tcp", name: "allocated-port-581", risk: "privileged" },
    ServiceHint { port: 583, proto: "tcp", name: "allocated-port-583", risk: "privileged" },
    ServiceHint { port: 588, proto: "tcp", name: "allocated-port-588", risk: "privileged" },
    ServiceHint { port: 594, proto: "tcp", name: "allocated-port-594", risk: "privileged" },
    ServiceHint { port: 595, proto: "tcp", name: "allocated-port-595", risk: "privileged" },
    ServiceHint { port: 602, proto: "tcp", name: "allocated-port-602", risk: "privileged" },
    ServiceHint { port: 605, proto: "tcp", name: "allocated-port-605", risk: "privileged" },
    ServiceHint { port: 609, proto: "tcp", name: "allocated-port-609", risk: "privileged" },
    ServiceHint { port: 616, proto: "tcp", name: "allocated-port-616", risk: "privileged" },
    ServiceHint { port: 623, proto: "tcp", name: "allocated-port-623", risk: "privileged" },
    ServiceHint { port: 627, proto: "tcp", name: "allocated-port-627", risk: "privileged" },
    ServiceHint { port: 630, proto: "tcp", name: "allocated-port-630", risk: "privileged" },
    ServiceHint { port: 637, proto: "tcp", name: "allocated-port-637", risk: "privileged" },
    ServiceHint { port: 638, proto: "tcp", name: "allocated-port-638", risk: "privileged" },
    ServiceHint { port: 644, proto: "tcp", name: "allocated-port-644", risk: "privileged" },
    ServiceHint { port: 649, proto: "tcp", name: "allocated-port-649", risk: "privileged" },
    ServiceHint { port: 651, proto: "tcp", name: "allocated-port-651", risk: "privileged" },
    ServiceHint { port: 658, proto: "tcp", name: "allocated-port-658", risk: "privileged" },
    ServiceHint { port: 660, proto: "tcp", name: "allocated-port-660", risk: "privileged" },
    ServiceHint { port: 665, proto: "tcp", name: "allocated-port-665", risk: "privileged" },
    ServiceHint { port: 671, proto: "tcp", name: "allocated-port-671", risk: "privileged" },
    ServiceHint { port: 672, proto: "tcp", name: "allocated-port-672", risk: "privileged" },
    ServiceHint { port: 679, proto: "tcp", name: "allocated-port-679", risk: "privileged" },
    ServiceHint { port: 682, proto: "tcp", name: "allocated-port-682", risk: "privileged" },
    ServiceHint { port: 686, proto: "tcp", name: "allocated-port-686", risk: "privileged" },
    ServiceHint { port: 693, proto: "tcp", name: "allocated-port-693", risk: "privileged" },
    ServiceHint { port: 700, proto: "tcp", name: "allocated-port-700", risk: "privileged" },
    ServiceHint { port: 704, proto: "tcp", name: "allocated-port-704", risk: "privileged" },
    ServiceHint { port: 707, proto: "tcp", name: "allocated-port-707", risk: "privileged" },
    ServiceHint { port: 714, proto: "tcp", name: "allocated-port-714", risk: "privileged" },
    ServiceHint { port: 715, proto: "tcp", name: "allocated-port-715", risk: "privileged" },
    ServiceHint { port: 721, proto: "tcp", name: "allocated-port-721", risk: "privileged" },
    ServiceHint { port: 726, proto: "tcp", name: "allocated-port-726", risk: "privileged" },
    ServiceHint { port: 728, proto: "tcp", name: "allocated-port-728", risk: "privileged" },
    ServiceHint { port: 735, proto: "tcp", name: "allocated-port-735", risk: "privileged" },
    ServiceHint { port: 737, proto: "tcp", name: "allocated-port-737", risk: "privileged" },
    ServiceHint { port: 742, proto: "tcp", name: "allocated-port-742", risk: "privileged" },
    ServiceHint { port: 748, proto: "tcp", name: "allocated-port-748", risk: "privileged" },
    ServiceHint { port: 749, proto: "tcp", name: "allocated-port-749", risk: "privileged" },
    ServiceHint { port: 756, proto: "tcp", name: "allocated-port-756", risk: "privileged" },
    ServiceHint { port: 759, proto: "tcp", name: "allocated-port-759", risk: "privileged" },
    ServiceHint { port: 763, proto: "tcp", name: "allocated-port-763", risk: "privileged" },
    ServiceHint { port: 770, proto: "tcp", name: "allocated-port-770", risk: "privileged" },
    ServiceHint { port: 777, proto: "tcp", name: "allocated-port-777", risk: "privileged" },
    ServiceHint { port: 781, proto: "tcp", name: "allocated-port-781", risk: "privileged" },
    ServiceHint { port: 784, proto: "tcp", name: "allocated-port-784", risk: "privileged" },
    ServiceHint { port: 791, proto: "tcp", name: "allocated-port-791", risk: "privileged" },
    ServiceHint { port: 792, proto: "tcp", name: "allocated-port-792", risk: "privileged" },
    ServiceHint { port: 798, proto: "tcp", name: "allocated-port-798", risk: "privileged" },
    ServiceHint { port: 803, proto: "tcp", name: "allocated-port-803", risk: "privileged" },
    ServiceHint { port: 805, proto: "tcp", name: "allocated-port-805", risk: "privileged" },
    ServiceHint { port: 812, proto: "tcp", name: "allocated-port-812", risk: "privileged" },
    ServiceHint { port: 814, proto: "tcp", name: "allocated-port-814", risk: "privileged" },
    ServiceHint { port: 819, proto: "tcp", name: "allocated-port-819", risk: "privileged" },
    ServiceHint { port: 825, proto: "tcp", name: "allocated-port-825", risk: "privileged" },
    ServiceHint { port: 826, proto: "tcp", name: "allocated-port-826", risk: "privileged" },
    ServiceHint { port: 833, proto: "tcp", name: "allocated-port-833", risk: "privileged" },
    ServiceHint { port: 836, proto: "tcp", name: "allocated-port-836", risk: "privileged" },
    ServiceHint { port: 840, proto: "tcp", name: "allocated-port-840", risk: "privileged" },
    ServiceHint { port: 847, proto: "tcp", name: "allocated-port-847", risk: "privileged" },
    ServiceHint { port: 854, proto: "tcp", name: "allocated-port-854", risk: "privileged" },
    ServiceHint { port: 858, proto: "tcp", name: "allocated-port-858", risk: "privileged" },
    ServiceHint { port: 861, proto: "tcp", name: "allocated-port-861", risk: "privileged" },
    ServiceHint { port: 868, proto: "tcp", name: "allocated-port-868", risk: "privileged" },
    ServiceHint { port: 869, proto: "tcp", name: "allocated-port-869", risk: "privileged" },
    ServiceHint { port: 875, proto: "tcp", name: "allocated-port-875", risk: "privileged" },
    ServiceHint { port: 880, proto: "tcp", name: "allocated-port-880", risk: "privileged" },
    ServiceHint { port: 882, proto: "tcp", name: "allocated-port-882", risk: "privileged" },
    ServiceHint { port: 889, proto: "tcp", name: "allocated-port-889", risk: "privileged" },
    ServiceHint { port: 891, proto: "tcp", name: "allocated-port-891", risk: "privileged" },
    ServiceHint { port: 896, proto: "tcp", name: "allocated-port-896", risk: "privileged" },
    ServiceHint { port: 902, proto: "tcp", name: "allocated-port-902", risk: "privileged" },
    ServiceHint { port: 903, proto: "tcp", name: "allocated-port-903", risk: "privileged" },
    ServiceHint { port: 910, proto: "tcp", name: "allocated-port-910", risk: "privileged" },
    ServiceHint { port: 913, proto: "tcp", name: "allocated-port-913", risk: "privileged" },
    ServiceHint { port: 917, proto: "tcp", name: "allocated-port-917", risk: "privileged" },
    ServiceHint { port: 924, proto: "tcp", name: "allocated-port-924", risk: "privileged" },
    ServiceHint { port: 931, proto: "tcp", name: "allocated-port-931", risk: "privileged" },
    ServiceHint { port: 935, proto: "tcp", name: "allocated-port-935", risk: "privileged" },
    ServiceHint { port: 938, proto: "tcp", name: "allocated-port-938", risk: "privileged" },
    ServiceHint { port: 945, proto: "tcp", name: "allocated-port-945", risk: "privileged" },
    ServiceHint { port: 946, proto: "tcp", name: "allocated-port-946", risk: "privileged" },
    ServiceHint { port: 952, proto: "tcp", name: "allocated-port-952", risk: "privileged" },
    ServiceHint { port: 957, proto: "tcp", name: "allocated-port-957", risk: "privileged" },
    ServiceHint { port: 959, proto: "tcp", name: "allocated-port-959", risk: "privileged" },
    ServiceHint { port: 966, proto: "tcp", name: "allocated-port-966", risk: "privileged" },
    ServiceHint { port: 968, proto: "tcp", name: "allocated-port-968", risk: "privileged" },
    ServiceHint { port: 973, proto: "tcp", name: "allocated-port-973", risk: "privileged" },
    ServiceHint { port: 979, proto: "tcp", name: "allocated-port-979", risk: "privileged" },
    ServiceHint { port: 980, proto: "tcp", name: "allocated-port-980", risk: "privileged" },
    ServiceHint { port: 987, proto: "tcp", name: "allocated-port-987", risk: "privileged" },
    ServiceHint { port: 990, proto: "tcp", name: "allocated-port-990", risk: "privileged" },
    ServiceHint { port: 994, proto: "tcp", name: "allocated-port-994", risk: "privileged" },
    ServiceHint { port: 1001, proto: "tcp", name: "allocated-port-1001", risk: "privileged" },
    ServiceHint { port: 1008, proto: "tcp", name: "allocated-port-1008", risk: "privileged" },
    ServiceHint { port: 1012, proto: "tcp", name: "allocated-port-1012", risk: "privileged" },
    ServiceHint { port: 1015, proto: "tcp", name: "allocated-port-1015", risk: "privileged" },
    ServiceHint { port: 1022, proto: "tcp", name: "allocated-port-1022", risk: "privileged" },
    ServiceHint { port: 1023, proto: "tcp", name: "allocated-port-1023", risk: "privileged" },
    ServiceHint { port: 1029, proto: "tcp", name: "allocated-port-1029", risk: "registered" },
    ServiceHint { port: 1034, proto: "tcp", name: "allocated-port-1034", risk: "registered" },
    ServiceHint { port: 1036, proto: "tcp", name: "allocated-port-1036", risk: "registered" },
    ServiceHint { port: 1043, proto: "tcp", name: "allocated-port-1043", risk: "registered" },
    ServiceHint { port: 1045, proto: "tcp", name: "allocated-port-1045", risk: "registered" },
    ServiceHint { port: 1050, proto: "tcp", name: "allocated-port-1050", risk: "registered" },
    ServiceHint { port: 1056, proto: "tcp", name: "allocated-port-1056", risk: "registered" },
    ServiceHint { port: 1057, proto: "tcp", name: "allocated-port-1057", risk: "registered" },
    ServiceHint { port: 1064, proto: "tcp", name: "allocated-port-1064", risk: "registered" },
    ServiceHint { port: 1067, proto: "tcp", name: "allocated-port-1067", risk: "registered" },
    ServiceHint { port: 1071, proto: "tcp", name: "allocated-port-1071", risk: "registered" },
    ServiceHint { port: 1078, proto: "tcp", name: "allocated-port-1078", risk: "registered" },
    ServiceHint { port: 1085, proto: "tcp", name: "allocated-port-1085", risk: "registered" },
    ServiceHint { port: 1089, proto: "tcp", name: "allocated-port-1089", risk: "registered" },
    ServiceHint { port: 1092, proto: "tcp", name: "allocated-port-1092", risk: "registered" },
    ServiceHint { port: 1099, proto: "tcp", name: "allocated-port-1099", risk: "registered" },
    ServiceHint { port: 1100, proto: "tcp", name: "allocated-port-1100", risk: "registered" },
    ServiceHint { port: 1106, proto: "tcp", name: "allocated-port-1106", risk: "registered" },
    ServiceHint { port: 1111, proto: "tcp", name: "allocated-port-1111", risk: "registered" },
    ServiceHint { port: 1113, proto: "tcp", name: "allocated-port-1113", risk: "registered" },
    ServiceHint { port: 1120, proto: "tcp", name: "allocated-port-1120", risk: "registered" },
    ServiceHint { port: 1122, proto: "tcp", name: "allocated-port-1122", risk: "registered" },
    ServiceHint { port: 1127, proto: "tcp", name: "allocated-port-1127", risk: "registered" },
    ServiceHint { port: 1133, proto: "tcp", name: "allocated-port-1133", risk: "registered" },
    ServiceHint { port: 1134, proto: "tcp", name: "allocated-port-1134", risk: "registered" },
    ServiceHint { port: 1141, proto: "tcp", name: "allocated-port-1141", risk: "registered" },
    ServiceHint { port: 1144, proto: "tcp", name: "allocated-port-1144", risk: "registered" },
    ServiceHint { port: 1148, proto: "tcp", name: "allocated-port-1148", risk: "registered" },
    ServiceHint { port: 1155, proto: "tcp", name: "allocated-port-1155", risk: "registered" },
    ServiceHint { port: 1162, proto: "tcp", name: "allocated-port-1162", risk: "registered" },
    ServiceHint { port: 1166, proto: "tcp", name: "allocated-port-1166", risk: "registered" },
    ServiceHint { port: 1169, proto: "tcp", name: "allocated-port-1169", risk: "registered" },
    ServiceHint { port: 1176, proto: "tcp", name: "allocated-port-1176", risk: "registered" },
    ServiceHint { port: 1177, proto: "tcp", name: "allocated-port-1177", risk: "registered" },
    ServiceHint { port: 1183, proto: "tcp", name: "allocated-port-1183", risk: "registered" },
    ServiceHint { port: 1188, proto: "tcp", name: "allocated-port-1188", risk: "registered" },
    ServiceHint { port: 1190, proto: "tcp", name: "allocated-port-1190", risk: "registered" },
    ServiceHint { port: 1197, proto: "tcp", name: "allocated-port-1197", risk: "registered" },
    ServiceHint { port: 1199, proto: "tcp", name: "allocated-port-1199", risk: "registered" },
    ServiceHint { port: 1204, proto: "tcp", name: "allocated-port-1204", risk: "registered" },
    ServiceHint { port: 1210, proto: "tcp", name: "allocated-port-1210", risk: "registered" },
    ServiceHint { port: 1211, proto: "tcp", name: "allocated-port-1211", risk: "registered" },
    ServiceHint { port: 1218, proto: "tcp", name: "allocated-port-1218", risk: "registered" },
    ServiceHint { port: 1221, proto: "tcp", name: "allocated-port-1221", risk: "registered" },
    ServiceHint { port: 1225, proto: "tcp", name: "allocated-port-1225", risk: "registered" },
    ServiceHint { port: 1232, proto: "tcp", name: "allocated-port-1232", risk: "registered" },
    ServiceHint { port: 1239, proto: "tcp", name: "allocated-port-1239", risk: "registered" },
    ServiceHint { port: 1243, proto: "tcp", name: "allocated-port-1243", risk: "registered" },
    ServiceHint { port: 1246, proto: "tcp", name: "allocated-port-1246", risk: "registered" },
    ServiceHint { port: 1253, proto: "tcp", name: "allocated-port-1253", risk: "registered" },
    ServiceHint { port: 1254, proto: "tcp", name: "allocated-port-1254", risk: "registered" },
    ServiceHint { port: 1260, proto: "tcp", name: "allocated-port-1260", risk: "registered" },
    ServiceHint { port: 1265, proto: "tcp", name: "allocated-port-1265", risk: "registered" },
    ServiceHint { port: 1267, proto: "tcp", name: "allocated-port-1267", risk: "registered" },
    ServiceHint { port: 1274, proto: "tcp", name: "allocated-port-1274", risk: "registered" },
    ServiceHint { port: 1276, proto: "tcp", name: "allocated-port-1276", risk: "registered" },
    ServiceHint { port: 1281, proto: "tcp", name: "allocated-port-1281", risk: "registered" },
    ServiceHint { port: 1287, proto: "tcp", name: "allocated-port-1287", risk: "registered" },
    ServiceHint { port: 1288, proto: "tcp", name: "allocated-port-1288", risk: "registered" },
    ServiceHint { port: 1295, proto: "tcp", name: "allocated-port-1295", risk: "registered" },
    ServiceHint { port: 1298, proto: "tcp", name: "allocated-port-1298", risk: "registered" },
    ServiceHint { port: 1302, proto: "tcp", name: "allocated-port-1302", risk: "registered" },
    ServiceHint { port: 1309, proto: "tcp", name: "allocated-port-1309", risk: "registered" },
    ServiceHint { port: 1316, proto: "tcp", name: "allocated-port-1316", risk: "registered" },
    ServiceHint { port: 1320, proto: "tcp", name: "allocated-port-1320", risk: "registered" },
    ServiceHint { port: 1323, proto: "tcp", name: "allocated-port-1323", risk: "registered" },
    ServiceHint { port: 1330, proto: "tcp", name: "allocated-port-1330", risk: "registered" },
    ServiceHint { port: 1331, proto: "tcp", name: "allocated-port-1331", risk: "registered" },
    ServiceHint { port: 1337, proto: "tcp", name: "allocated-port-1337", risk: "registered" },
    ServiceHint { port: 1342, proto: "tcp", name: "allocated-port-1342", risk: "registered" },
    ServiceHint { port: 1344, proto: "tcp", name: "allocated-port-1344", risk: "registered" },
    ServiceHint { port: 1351, proto: "tcp", name: "allocated-port-1351", risk: "registered" },
    ServiceHint { port: 1353, proto: "tcp", name: "allocated-port-1353", risk: "registered" },
    ServiceHint { port: 1358, proto: "tcp", name: "allocated-port-1358", risk: "registered" },
    ServiceHint { port: 1364, proto: "tcp", name: "allocated-port-1364", risk: "registered" },
    ServiceHint { port: 1365, proto: "tcp", name: "allocated-port-1365", risk: "registered" },
    ServiceHint { port: 1372, proto: "tcp", name: "allocated-port-1372", risk: "registered" },
    ServiceHint { port: 1375, proto: "tcp", name: "allocated-port-1375", risk: "registered" },
    ServiceHint { port: 1379, proto: "tcp", name: "allocated-port-1379", risk: "registered" },
    ServiceHint { port: 1386, proto: "tcp", name: "allocated-port-1386", risk: "registered" },
    ServiceHint { port: 1393, proto: "tcp", name: "allocated-port-1393", risk: "registered" },
    ServiceHint { port: 1397, proto: "tcp", name: "allocated-port-1397", risk: "registered" },
    ServiceHint { port: 1400, proto: "tcp", name: "allocated-port-1400", risk: "registered" },
    ServiceHint { port: 1407, proto: "tcp", name: "allocated-port-1407", risk: "registered" },
    ServiceHint { port: 1408, proto: "tcp", name: "allocated-port-1408", risk: "registered" },
    ServiceHint { port: 1414, proto: "tcp", name: "allocated-port-1414", risk: "registered" },
    ServiceHint { port: 1419, proto: "tcp", name: "allocated-port-1419", risk: "registered" },
    ServiceHint { port: 1421, proto: "tcp", name: "allocated-port-1421", risk: "registered" },
    ServiceHint { port: 1428, proto: "tcp", name: "allocated-port-1428", risk: "registered" },
    ServiceHint { port: 1430, proto: "tcp", name: "allocated-port-1430", risk: "registered" },
    ServiceHint { port: 1435, proto: "tcp", name: "allocated-port-1435", risk: "registered" },
    ServiceHint { port: 1441, proto: "tcp", name: "allocated-port-1441", risk: "registered" },
    ServiceHint { port: 1442, proto: "tcp", name: "allocated-port-1442", risk: "registered" },
    ServiceHint { port: 1449, proto: "tcp", name: "allocated-port-1449", risk: "registered" },
    ServiceHint { port: 1452, proto: "tcp", name: "allocated-port-1452", risk: "registered" },
    ServiceHint { port: 1456, proto: "tcp", name: "allocated-port-1456", risk: "registered" },
    ServiceHint { port: 1463, proto: "tcp", name: "allocated-port-1463", risk: "registered" },
    ServiceHint { port: 1470, proto: "tcp", name: "allocated-port-1470", risk: "registered" },
    ServiceHint { port: 1474, proto: "tcp", name: "allocated-port-1474", risk: "registered" },
    ServiceHint { port: 1477, proto: "tcp", name: "allocated-port-1477", risk: "registered" },
    ServiceHint { port: 1484, proto: "tcp", name: "allocated-port-1484", risk: "registered" },
    ServiceHint { port: 1485, proto: "tcp", name: "allocated-port-1485", risk: "registered" },
    ServiceHint { port: 1491, proto: "tcp", name: "allocated-port-1491", risk: "registered" },
    ServiceHint { port: 1496, proto: "tcp", name: "allocated-port-1496", risk: "registered" },
    ServiceHint { port: 1498, proto: "tcp", name: "allocated-port-1498", risk: "registered" },
    ServiceHint { port: 1505, proto: "tcp", name: "allocated-port-1505", risk: "registered" },
    ServiceHint { port: 1507, proto: "tcp", name: "allocated-port-1507", risk: "registered" },
    ServiceHint { port: 1512, proto: "tcp", name: "allocated-port-1512", risk: "registered" },
    ServiceHint { port: 1518, proto: "tcp", name: "allocated-port-1518", risk: "registered" },
    ServiceHint { port: 1519, proto: "tcp", name: "allocated-port-1519", risk: "registered" },
    ServiceHint { port: 1526, proto: "tcp", name: "allocated-port-1526", risk: "registered" },
    ServiceHint { port: 1529, proto: "tcp", name: "allocated-port-1529", risk: "registered" },
    ServiceHint { port: 1533, proto: "tcp", name: "allocated-port-1533", risk: "registered" },
    ServiceHint { port: 1540, proto: "tcp", name: "allocated-port-1540", risk: "registered" },
    ServiceHint { port: 1547, proto: "tcp", name: "allocated-port-1547", risk: "registered" },
    ServiceHint { port: 1551, proto: "tcp", name: "allocated-port-1551", risk: "registered" },
    ServiceHint { port: 1554, proto: "tcp", name: "allocated-port-1554", risk: "registered" },
    ServiceHint { port: 1561, proto: "tcp", name: "allocated-port-1561", risk: "registered" },
    ServiceHint { port: 1562, proto: "tcp", name: "allocated-port-1562", risk: "registered" },
    ServiceHint { port: 1568, proto: "tcp", name: "allocated-port-1568", risk: "registered" },
    ServiceHint { port: 1573, proto: "tcp", name: "allocated-port-1573", risk: "registered" },
    ServiceHint { port: 1575, proto: "tcp", name: "allocated-port-1575", risk: "registered" },
    ServiceHint { port: 1582, proto: "tcp", name: "allocated-port-1582", risk: "registered" },
    ServiceHint { port: 1584, proto: "tcp", name: "allocated-port-1584", risk: "registered" },
    ServiceHint { port: 1589, proto: "tcp", name: "allocated-port-1589", risk: "registered" },
    ServiceHint { port: 1595, proto: "tcp", name: "allocated-port-1595", risk: "registered" },
    ServiceHint { port: 1596, proto: "tcp", name: "allocated-port-1596", risk: "registered" },
    ServiceHint { port: 1603, proto: "tcp", name: "allocated-port-1603", risk: "registered" },
    ServiceHint { port: 1606, proto: "tcp", name: "allocated-port-1606", risk: "registered" },
    ServiceHint { port: 1610, proto: "tcp", name: "allocated-port-1610", risk: "registered" },
    ServiceHint { port: 1617, proto: "tcp", name: "allocated-port-1617", risk: "registered" },
    ServiceHint { port: 1624, proto: "tcp", name: "allocated-port-1624", risk: "registered" },
    ServiceHint { port: 1628, proto: "tcp", name: "allocated-port-1628", risk: "registered" },
    ServiceHint { port: 1631, proto: "tcp", name: "allocated-port-1631", risk: "registered" },
    ServiceHint { port: 1638, proto: "tcp", name: "allocated-port-1638", risk: "registered" },
    ServiceHint { port: 1639, proto: "tcp", name: "allocated-port-1639", risk: "registered" },
    ServiceHint { port: 1645, proto: "tcp", name: "allocated-port-1645", risk: "registered" },
    ServiceHint { port: 1650, proto: "tcp", name: "allocated-port-1650", risk: "registered" },
    ServiceHint { port: 1652, proto: "tcp", name: "allocated-port-1652", risk: "registered" },
    ServiceHint { port: 1659, proto: "tcp", name: "allocated-port-1659", risk: "registered" },
    ServiceHint { port: 1661, proto: "tcp", name: "allocated-port-1661", risk: "registered" },
    ServiceHint { port: 1666, proto: "tcp", name: "allocated-port-1666", risk: "registered" },
    ServiceHint { port: 1672, proto: "tcp", name: "allocated-port-1672", risk: "registered" },
    ServiceHint { port: 1673, proto: "tcp", name: "allocated-port-1673", risk: "registered" },
    ServiceHint { port: 1680, proto: "tcp", name: "allocated-port-1680", risk: "registered" },
    ServiceHint { port: 1683, proto: "tcp", name: "allocated-port-1683", risk: "registered" },
    ServiceHint { port: 1687, proto: "tcp", name: "allocated-port-1687", risk: "registered" },
    ServiceHint { port: 1694, proto: "tcp", name: "allocated-port-1694", risk: "registered" },
    ServiceHint { port: 1701, proto: "tcp", name: "allocated-port-1701", risk: "registered" },
    ServiceHint { port: 1705, proto: "tcp", name: "allocated-port-1705", risk: "registered" },
    ServiceHint { port: 1708, proto: "tcp", name: "allocated-port-1708", risk: "registered" },
    ServiceHint { port: 1715, proto: "tcp", name: "allocated-port-1715", risk: "registered" },
    ServiceHint { port: 1716, proto: "tcp", name: "allocated-port-1716", risk: "registered" },
    ServiceHint { port: 1722, proto: "tcp", name: "allocated-port-1722", risk: "registered" },
    ServiceHint { port: 1727, proto: "tcp", name: "allocated-port-1727", risk: "registered" },
    ServiceHint { port: 1729, proto: "tcp", name: "allocated-port-1729", risk: "registered" },
    ServiceHint { port: 1736, proto: "tcp", name: "allocated-port-1736", risk: "registered" },
    ServiceHint { port: 1738, proto: "tcp", name: "allocated-port-1738", risk: "registered" },
    ServiceHint { port: 1743, proto: "tcp", name: "allocated-port-1743", risk: "registered" },
    ServiceHint { port: 1749, proto: "tcp", name: "allocated-port-1749", risk: "registered" },
    ServiceHint { port: 1750, proto: "tcp", name: "allocated-port-1750", risk: "registered" },
    ServiceHint { port: 1757, proto: "tcp", name: "allocated-port-1757", risk: "registered" },
    ServiceHint { port: 1760, proto: "tcp", name: "allocated-port-1760", risk: "registered" },
    ServiceHint { port: 1764, proto: "tcp", name: "allocated-port-1764", risk: "registered" },
    ServiceHint { port: 1771, proto: "tcp", name: "allocated-port-1771", risk: "registered" },
    ServiceHint { port: 1778, proto: "tcp", name: "allocated-port-1778", risk: "registered" },
    ServiceHint { port: 1782, proto: "tcp", name: "allocated-port-1782", risk: "registered" },
    ServiceHint { port: 1785, proto: "tcp", name: "allocated-port-1785", risk: "registered" },
    ServiceHint { port: 1792, proto: "tcp", name: "allocated-port-1792", risk: "registered" },
    ServiceHint { port: 1793, proto: "tcp", name: "allocated-port-1793", risk: "registered" },
    ServiceHint { port: 1799, proto: "tcp", name: "allocated-port-1799", risk: "registered" },
    ServiceHint { port: 1804, proto: "tcp", name: "allocated-port-1804", risk: "registered" },
    ServiceHint { port: 1806, proto: "tcp", name: "allocated-port-1806", risk: "registered" },
    ServiceHint { port: 1813, proto: "tcp", name: "allocated-port-1813", risk: "registered" },
    ServiceHint { port: 1815, proto: "tcp", name: "allocated-port-1815", risk: "registered" },
    ServiceHint { port: 1820, proto: "tcp", name: "allocated-port-1820", risk: "registered" },
    ServiceHint { port: 1826, proto: "tcp", name: "allocated-port-1826", risk: "registered" },
    ServiceHint { port: 1827, proto: "tcp", name: "allocated-port-1827", risk: "registered" },
    ServiceHint { port: 1834, proto: "tcp", name: "allocated-port-1834", risk: "registered" },
    ServiceHint { port: 1837, proto: "tcp", name: "allocated-port-1837", risk: "registered" },
    ServiceHint { port: 1841, proto: "tcp", name: "allocated-port-1841", risk: "registered" },
    ServiceHint { port: 1848, proto: "tcp", name: "allocated-port-1848", risk: "registered" },
    ServiceHint { port: 1855, proto: "tcp", name: "allocated-port-1855", risk: "registered" },
    ServiceHint { port: 1859, proto: "tcp", name: "allocated-port-1859", risk: "registered" },
    ServiceHint { port: 1862, proto: "tcp", name: "allocated-port-1862", risk: "registered" },
    ServiceHint { port: 1869, proto: "tcp", name: "allocated-port-1869", risk: "registered" },
    ServiceHint { port: 1870, proto: "tcp", name: "allocated-port-1870", risk: "registered" },
    ServiceHint { port: 1876, proto: "tcp", name: "allocated-port-1876", risk: "registered" },
    ServiceHint { port: 1881, proto: "tcp", name: "allocated-port-1881", risk: "registered" },
    ServiceHint { port: 1883, proto: "tcp", name: "allocated-port-1883", risk: "registered" },
    ServiceHint { port: 1890, proto: "tcp", name: "allocated-port-1890", risk: "registered" },
    ServiceHint { port: 1892, proto: "tcp", name: "allocated-port-1892", risk: "registered" },
    ServiceHint { port: 1897, proto: "tcp", name: "allocated-port-1897", risk: "registered" },
    ServiceHint { port: 1903, proto: "tcp", name: "allocated-port-1903", risk: "registered" },
    ServiceHint { port: 1904, proto: "tcp", name: "allocated-port-1904", risk: "registered" },
    ServiceHint { port: 1911, proto: "tcp", name: "allocated-port-1911", risk: "registered" },
    ServiceHint { port: 1914, proto: "tcp", name: "allocated-port-1914", risk: "registered" },
    ServiceHint { port: 1918, proto: "tcp", name: "allocated-port-1918", risk: "registered" },
    ServiceHint { port: 1925, proto: "tcp", name: "allocated-port-1925", risk: "registered" },
    ServiceHint { port: 1932, proto: "tcp", name: "allocated-port-1932", risk: "registered" },
    ServiceHint { port: 1936, proto: "tcp", name: "allocated-port-1936", risk: "registered" },
    ServiceHint { port: 1939, proto: "tcp", name: "allocated-port-1939", risk: "registered" },
    ServiceHint { port: 1946, proto: "tcp", name: "allocated-port-1946", risk: "registered" },
    ServiceHint { port: 1947, proto: "tcp", name: "allocated-port-1947", risk: "registered" },
    ServiceHint { port: 1953, proto: "tcp", name: "allocated-port-1953", risk: "registered" },
    ServiceHint { port: 1958, proto: "tcp", name: "allocated-port-1958", risk: "registered" },
    ServiceHint { port: 1960, proto: "tcp", name: "allocated-port-1960", risk: "registered" },
    ServiceHint { port: 1967, proto: "tcp", name: "allocated-port-1967", risk: "registered" },
    ServiceHint { port: 1969, proto: "tcp", name: "allocated-port-1969", risk: "registered" },
    ServiceHint { port: 1974, proto: "tcp", name: "allocated-port-1974", risk: "registered" },
    ServiceHint { port: 1980, proto: "tcp", name: "allocated-port-1980", risk: "registered" },
    ServiceHint { port: 1981, proto: "tcp", name: "allocated-port-1981", risk: "registered" },
    ServiceHint { port: 1988, proto: "tcp", name: "allocated-port-1988", risk: "registered" },
    ServiceHint { port: 1991, proto: "tcp", name: "allocated-port-1991", risk: "registered" },
    ServiceHint { port: 1995, proto: "tcp", name: "allocated-port-1995", risk: "registered" },
    ServiceHint { port: 2002, proto: "tcp", name: "allocated-port-2002", risk: "registered" },
    ServiceHint { port: 2009, proto: "tcp", name: "allocated-port-2009", risk: "registered" },
    ServiceHint { port: 2013, proto: "tcp", name: "allocated-port-2013", risk: "registered" },
    ServiceHint { port: 2016, proto: "tcp", name: "allocated-port-2016", risk: "registered" },
    ServiceHint { port: 2023, proto: "tcp", name: "allocated-port-2023", risk: "registered" },
    ServiceHint { port: 2024, proto: "tcp", name: "allocated-port-2024", risk: "registered" },
    ServiceHint { port: 2030, proto: "tcp", name: "allocated-port-2030", risk: "registered" },
    ServiceHint { port: 2035, proto: "tcp", name: "allocated-port-2035", risk: "registered" },
    ServiceHint { port: 2037, proto: "tcp", name: "allocated-port-2037", risk: "registered" },
    ServiceHint { port: 2044, proto: "tcp", name: "allocated-port-2044", risk: "registered" },
    ServiceHint { port: 2046, proto: "tcp", name: "allocated-port-2046", risk: "registered" },
    ServiceHint { port: 2051, proto: "tcp", name: "allocated-port-2051", risk: "registered" },
    ServiceHint { port: 2057, proto: "tcp", name: "allocated-port-2057", risk: "registered" },
    ServiceHint { port: 2058, proto: "tcp", name: "allocated-port-2058", risk: "registered" },
    ServiceHint { port: 2065, proto: "tcp", name: "allocated-port-2065", risk: "registered" },
    ServiceHint { port: 2068, proto: "tcp", name: "allocated-port-2068", risk: "registered" },
    ServiceHint { port: 2072, proto: "tcp", name: "allocated-port-2072", risk: "registered" },
    ServiceHint { port: 2079, proto: "tcp", name: "allocated-port-2079", risk: "registered" },
    ServiceHint { port: 2086, proto: "tcp", name: "allocated-port-2086", risk: "registered" },
    ServiceHint { port: 2090, proto: "tcp", name: "allocated-port-2090", risk: "registered" },
    ServiceHint { port: 2093, proto: "tcp", name: "allocated-port-2093", risk: "registered" },
    ServiceHint { port: 2100, proto: "tcp", name: "allocated-port-2100", risk: "registered" },
    ServiceHint { port: 2101, proto: "tcp", name: "allocated-port-2101", risk: "registered" },
    ServiceHint { port: 2107, proto: "tcp", name: "allocated-port-2107", risk: "registered" },
    ServiceHint { port: 2112, proto: "tcp", name: "allocated-port-2112", risk: "registered" },
    ServiceHint { port: 2114, proto: "tcp", name: "allocated-port-2114", risk: "registered" },
    ServiceHint { port: 2121, proto: "tcp", name: "allocated-port-2121", risk: "registered" },
    ServiceHint { port: 2123, proto: "tcp", name: "allocated-port-2123", risk: "registered" },
    ServiceHint { port: 2128, proto: "tcp", name: "allocated-port-2128", risk: "registered" },
    ServiceHint { port: 2134, proto: "tcp", name: "allocated-port-2134", risk: "registered" },
    ServiceHint { port: 2135, proto: "tcp", name: "allocated-port-2135", risk: "registered" },
    ServiceHint { port: 2142, proto: "tcp", name: "allocated-port-2142", risk: "registered" },
    ServiceHint { port: 2145, proto: "tcp", name: "allocated-port-2145", risk: "registered" },
    ServiceHint { port: 2149, proto: "tcp", name: "allocated-port-2149", risk: "registered" },
    ServiceHint { port: 2156, proto: "tcp", name: "allocated-port-2156", risk: "registered" },
    ServiceHint { port: 2163, proto: "tcp", name: "allocated-port-2163", risk: "registered" },
    ServiceHint { port: 2167, proto: "tcp", name: "allocated-port-2167", risk: "registered" },
    ServiceHint { port: 2170, proto: "tcp", name: "allocated-port-2170", risk: "registered" },
    ServiceHint { port: 2177, proto: "tcp", name: "allocated-port-2177", risk: "registered" },
    ServiceHint { port: 2178, proto: "tcp", name: "allocated-port-2178", risk: "registered" },
    ServiceHint { port: 2184, proto: "tcp", name: "allocated-port-2184", risk: "registered" },
    ServiceHint { port: 2189, proto: "tcp", name: "allocated-port-2189", risk: "registered" },
    ServiceHint { port: 2191, proto: "tcp", name: "allocated-port-2191", risk: "registered" },
    ServiceHint { port: 2198, proto: "tcp", name: "allocated-port-2198", risk: "registered" },
];

pub const PROTOCOL_NAMES: &[(u8, &str)] = &[
    (0, "protocol-0"),
    (1, "icmp"),
    (2, "igmp"),
    (3, "protocol-3"),
    (4, "protocol-4"),
    (5, "protocol-5"),
    (6, "tcp"),
    (7, "protocol-7"),
    (8, "protocol-8"),
    (9, "protocol-9"),
    (10, "protocol-10"),
    (11, "protocol-11"),
    (12, "protocol-12"),
    (13, "protocol-13"),
    (14, "protocol-14"),
    (15, "protocol-15"),
    (16, "protocol-16"),
    (17, "udp"),
    (18, "protocol-18"),
    (19, "protocol-19"),
    (20, "protocol-20"),
    (21, "protocol-21"),
    (22, "protocol-22"),
    (23, "protocol-23"),
    (24, "protocol-24"),
    (25, "protocol-25"),
    (26, "protocol-26"),
    (27, "protocol-27"),
    (28, "protocol-28"),
    (29, "protocol-29"),
    (30, "protocol-30"),
    (31, "protocol-31"),
    (36, "protocol-36"),
    (41, "ipv6"),
    (45, "protocol-45"),
    (47, "gre"),
    (50, "esp"),
    (51, "ah"),
    (54, "protocol-54"),
    (58, "icmpv6"),
    (63, "protocol-63"),
    (72, "protocol-72"),
    (81, "protocol-81"),
    (89, "ospf"),
    (90, "protocol-90"),
    (99, "protocol-99"),
    (108, "protocol-108"),
    (117, "protocol-117"),
    (126, "protocol-126"),
    (132, "sctp"),
    (135, "protocol-135"),
    (144, "protocol-144"),
    (153, "protocol-153"),
    (162, "protocol-162"),
    (171, "protocol-171"),
    (180, "protocol-180"),
    (189, "protocol-189"),
    (198, "protocol-198"),
    (207, "protocol-207"),
    (216, "protocol-216"),
    (225, "protocol-225"),
    (234, "protocol-234"),
    (243, "protocol-243"),
    (252, "protocol-252"),
];

pub fn service_name(port: u16, proto: &str) -> Option<&'static str> {
    SERVICE_HINTS.iter().find(|s| s.port == port && s.proto.eq_ignore_ascii_case(proto)).map(|s| s.name)
}

pub fn service_risk(port: u16, proto: &str) -> &'static str {
    SERVICE_HINTS.iter().find(|s| s.port == port && s.proto.eq_ignore_ascii_case(proto)).map(|s| s.risk).unwrap_or("unknown")
}

pub fn protocol_name(number: u8) -> &'static str {
    PROTOCOL_NAMES.iter().find(|(n, _)| *n == number).map(|(_, name)| *name).unwrap_or("unknown")
}

pub const SECURITY_TOKENS: &[&str] = &[
    "alert",
    "allow",
    "block",
    "drop",
    "reject",
    "pass",
    "content",
    "sid",
    "rev",
    "classtype",
    "metadata",
    "reference",
    "flow",
    "established",
    "to_server",
    "to_client",
    "domain",
    "url",
    "hash",
    "sha256",
    "sha1",
    "md5",
    "cidr",
    "input",
    "output",
    "forward",
    "accept",
    "deny",
    "dns",
    "http",
    "tls",
    "ssh",
    "rdp",
    "smb",
    "ldap",
    "kerberos",
    "syslog",
    "icmp",
    "tcp",
    "udp",
    "scan",
    "beacon",
    "malware",
    "phishing",
    "ransomware",
    "botnet",
    "command-control",
    "exfiltration",
    "lateral-movement",
    "alert-signal-0",
    "allow-signal-1",
    "block-signal-2",
    "drop-signal-3",
    "reject-signal-4",
    "pass-signal-5",
    "content-signal-6",
    "sid-signal-7",
    "rev-signal-8",
    "classtype-signal-9",
    "metadata-signal-10",
    "reference-signal-11",
    "flow-signal-12",
    "established-signal-13",
    "to_server-signal-14",
    "to_client-signal-15",
    "domain-signal-16",
    "url-signal-17",
    "hash-signal-18",
    "sha256-signal-19",
    "sha1-signal-20",
    "md5-signal-21",
    "cidr-signal-22",
    "input-signal-23",
    "output-signal-24",
    "forward-signal-25",
    "accept-signal-26",
    "deny-signal-27",
    "dns-signal-28",
    "http-signal-29",
    "tls-signal-30",
    "ssh-signal-31",
    "rdp-signal-32",
    "smb-signal-33",
    "ldap-signal-34",
    "kerberos-signal-35",
    "syslog-signal-36",
    "icmp-signal-37",
    "tcp-signal-38",
    "udp-signal-39",
    "scan-signal-40",
    "beacon-signal-41",
    "malware-signal-42",
    "phishing-signal-43",
    "ransomware-signal-44",
    "botnet-signal-45",
    "command-control-signal-46",
    "exfiltration-signal-47",
    "lateral-movement-signal-48",
    "alert-signal-49",
    "allow-signal-50",
    "block-signal-51",
    "drop-signal-52",
    "reject-signal-53",
    "pass-signal-54",
    "content-signal-55",
    "sid-signal-56",
    "rev-signal-57",
    "classtype-signal-58",
    "metadata-signal-59",
    "reference-signal-60",
    "flow-signal-61",
    "established-signal-62",
    "to_server-signal-63",
    "to_client-signal-64",
    "domain-signal-65",
    "url-signal-66",
    "hash-signal-67",
    "sha256-signal-68",
    "sha1-signal-69",
    "md5-signal-70",
    "cidr-signal-71",
    "input-signal-72",
    "output-signal-73",
    "forward-signal-74",
    "accept-signal-75",
    "deny-signal-76",
    "dns-signal-77",
    "http-signal-78",
    "tls-signal-79",
    "ssh-signal-80",
    "rdp-signal-81",
    "smb-signal-82",
    "ldap-signal-83",
    "kerberos-signal-84",
    "syslog-signal-85",
    "icmp-signal-86",
    "tcp-signal-87",
    "udp-signal-88",
    "scan-signal-89",
    "beacon-signal-90",
    "malware-signal-91",
    "phishing-signal-92",
    "ransomware-signal-93",
    "botnet-signal-94",
    "command-control-signal-95",
    "exfiltration-signal-96",
    "lateral-movement-signal-97",
    "alert-signal-98",
    "allow-signal-99",
    "block-signal-100",
    "drop-signal-101",
    "reject-signal-102",
    "pass-signal-103",
    "content-signal-104",
    "sid-signal-105",
    "rev-signal-106",
    "classtype-signal-107",
    "metadata-signal-108",
    "reference-signal-109",
    "flow-signal-110",
    "established-signal-111",
    "to_server-signal-112",
    "to_client-signal-113",
    "domain-signal-114",
    "url-signal-115",
    "hash-signal-116",
    "sha256-signal-117",
    "sha1-signal-118",
    "md5-signal-119",
    "cidr-signal-120",
    "input-signal-121",
    "output-signal-122",
    "forward-signal-123",
    "accept-signal-124",
    "deny-signal-125",
    "dns-signal-126",
    "http-signal-127",
    "tls-signal-128",
    "ssh-signal-129",
    "rdp-signal-130",
    "smb-signal-131",
    "ldap-signal-132",
    "kerberos-signal-133",
    "syslog-signal-134",
    "icmp-signal-135",
    "tcp-signal-136",
    "udp-signal-137",
    "scan-signal-138",
    "beacon-signal-139",
    "malware-signal-140",
    "phishing-signal-141",
    "ransomware-signal-142",
    "botnet-signal-143",
    "command-control-signal-144",
    "exfiltration-signal-145",
    "lateral-movement-signal-146",
    "alert-signal-147",
    "allow-signal-148",
    "block-signal-149",
    "drop-signal-150",
    "reject-signal-151",
    "pass-signal-152",
    "content-signal-153",
    "sid-signal-154",
    "rev-signal-155",
    "classtype-signal-156",
    "metadata-signal-157",
    "reference-signal-158",
    "flow-signal-159",
    "established-signal-160",
    "to_server-signal-161",
    "to_client-signal-162",
    "domain-signal-163",
    "url-signal-164",
    "hash-signal-165",
    "sha256-signal-166",
    "sha1-signal-167",
    "md5-signal-168",
    "cidr-signal-169",
    "input-signal-170",
    "output-signal-171",
    "forward-signal-172",
    "accept-signal-173",
    "deny-signal-174",
    "dns-signal-175",
    "http-signal-176",
    "tls-signal-177",
    "ssh-signal-178",
    "rdp-signal-179",
    "smb-signal-180",
    "ldap-signal-181",
    "kerberos-signal-182",
    "syslog-signal-183",
    "icmp-signal-184",
    "tcp-signal-185",
    "udp-signal-186",
    "scan-signal-187",
    "beacon-signal-188",
    "malware-signal-189",
    "phishing-signal-190",
    "ransomware-signal-191",
    "botnet-signal-192",
    "command-control-signal-193",
    "exfiltration-signal-194",
    "lateral-movement-signal-195",
    "alert-signal-196",
    "allow-signal-197",
    "block-signal-198",
    "drop-signal-199",
    "reject-signal-200",
    "pass-signal-201",
    "content-signal-202",
    "sid-signal-203",
    "rev-signal-204",
    "classtype-signal-205",
    "metadata-signal-206",
    "reference-signal-207",
    "flow-signal-208",
    "established-signal-209",
    "to_server-signal-210",
    "to_client-signal-211",
    "domain-signal-212",
    "url-signal-213",
    "hash-signal-214",
    "sha256-signal-215",
    "sha1-signal-216",
    "md5-signal-217",
    "cidr-signal-218",
    "input-signal-219",
    "output-signal-220",
    "forward-signal-221",
    "accept-signal-222",
    "deny-signal-223",
    "dns-signal-224",
    "http-signal-225",
    "tls-signal-226",
    "ssh-signal-227",
    "rdp-signal-228",
    "smb-signal-229",
    "ldap-signal-230",
    "kerberos-signal-231",
    "syslog-signal-232",
    "icmp-signal-233",
    "tcp-signal-234",
    "udp-signal-235",
    "scan-signal-236",
    "beacon-signal-237",
    "malware-signal-238",
    "phishing-signal-239",
    "ransomware-signal-240",
    "botnet-signal-241",
    "command-control-signal-242",
    "exfiltration-signal-243",
    "lateral-movement-signal-244",
    "alert-signal-245",
    "allow-signal-246",
    "block-signal-247",
    "drop-signal-248",
    "reject-signal-249",
    "pass-signal-250",
    "content-signal-251",
    "sid-signal-252",
    "rev-signal-253",
    "classtype-signal-254",
    "metadata-signal-255",
    "reference-signal-256",
    "flow-signal-257",
    "established-signal-258",
    "to_server-signal-259",
    "to_client-signal-260",
    "domain-signal-261",
    "url-signal-262",
    "hash-signal-263",
    "sha256-signal-264",
    "sha1-signal-265",
    "md5-signal-266",
    "cidr-signal-267",
    "input-signal-268",
    "output-signal-269",
    "forward-signal-270",
    "accept-signal-271",
    "deny-signal-272",
    "dns-signal-273",
    "http-signal-274",
    "tls-signal-275",
    "ssh-signal-276",
    "rdp-signal-277",
    "smb-signal-278",
    "ldap-signal-279",
    "kerberos-signal-280",
    "syslog-signal-281",
    "icmp-signal-282",
    "tcp-signal-283",
    "udp-signal-284",
    "scan-signal-285",
    "beacon-signal-286",
    "malware-signal-287",
    "phishing-signal-288",
    "ransomware-signal-289",
    "botnet-signal-290",
    "command-control-signal-291",
    "exfiltration-signal-292",
    "lateral-movement-signal-293",
    "alert-signal-294",
    "allow-signal-295",
    "block-signal-296",
    "drop-signal-297",
    "reject-signal-298",
    "pass-signal-299",
    "content-signal-300",
    "sid-signal-301",
    "rev-signal-302",
    "classtype-signal-303",
    "metadata-signal-304",
    "reference-signal-305",
    "flow-signal-306",
    "established-signal-307",
    "to_server-signal-308",
    "to_client-signal-309",
    "domain-signal-310",
    "url-signal-311",
    "hash-signal-312",
    "sha256-signal-313",
    "sha1-signal-314",
    "md5-signal-315",
    "cidr-signal-316",
    "input-signal-317",
    "output-signal-318",
    "forward-signal-319",
    "accept-signal-320",
    "deny-signal-321",
    "dns-signal-322",
    "http-signal-323",
    "tls-signal-324",
    "ssh-signal-325",
    "rdp-signal-326",
    "smb-signal-327",
    "ldap-signal-328",
    "kerberos-signal-329",
    "syslog-signal-330",
    "icmp-signal-331",
    "tcp-signal-332",
    "udp-signal-333",
    "scan-signal-334",
    "beacon-signal-335",
    "malware-signal-336",
    "phishing-signal-337",
    "ransomware-signal-338",
    "botnet-signal-339",
    "command-control-signal-340",
    "exfiltration-signal-341",
    "lateral-movement-signal-342",
    "alert-signal-343",
    "allow-signal-344",
    "block-signal-345",
    "drop-signal-346",
    "reject-signal-347",
    "pass-signal-348",
    "content-signal-349",
    "sid-signal-350",
    "rev-signal-351",
    "classtype-signal-352",
    "metadata-signal-353",
    "reference-signal-354",
    "flow-signal-355",
    "established-signal-356",
    "to_server-signal-357",
    "to_client-signal-358",
    "domain-signal-359",
    "url-signal-360",
    "hash-signal-361",
    "sha256-signal-362",
    "sha1-signal-363",
    "md5-signal-364",
    "cidr-signal-365",
    "input-signal-366",
    "output-signal-367",
    "forward-signal-368",
    "accept-signal-369",
    "deny-signal-370",
    "dns-signal-371",
    "http-signal-372",
    "tls-signal-373",
    "ssh-signal-374",
    "rdp-signal-375",
    "smb-signal-376",
    "ldap-signal-377",
    "kerberos-signal-378",
    "syslog-signal-379",
    "icmp-signal-380",
    "tcp-signal-381",
    "udp-signal-382",
    "scan-signal-383",
    "beacon-signal-384",
    "malware-signal-385",
    "phishing-signal-386",
    "ransomware-signal-387",
    "botnet-signal-388",
    "command-control-signal-389",
    "exfiltration-signal-390",
    "lateral-movement-signal-391",
    "alert-signal-392",
    "allow-signal-393",
    "block-signal-394",
    "drop-signal-395",
    "reject-signal-396",
    "pass-signal-397",
    "content-signal-398",
    "sid-signal-399",
    "rev-signal-400",
    "classtype-signal-401",
    "metadata-signal-402",
    "reference-signal-403",
    "flow-signal-404",
    "established-signal-405",
    "to_server-signal-406",
    "to_client-signal-407",
    "domain-signal-408",
    "url-signal-409",
    "hash-signal-410",
    "sha256-signal-411",
    "sha1-signal-412",
    "md5-signal-413",
    "cidr-signal-414",
    "input-signal-415",
    "output-signal-416",
    "forward-signal-417",
    "accept-signal-418",
    "deny-signal-419",
    "dns-signal-420",
    "http-signal-421",
    "tls-signal-422",
    "ssh-signal-423",
    "rdp-signal-424",
    "smb-signal-425",
    "ldap-signal-426",
    "kerberos-signal-427",
    "syslog-signal-428",
    "icmp-signal-429",
    "tcp-signal-430",
    "udp-signal-431",
    "scan-signal-432",
    "beacon-signal-433",
    "malware-signal-434",
    "phishing-signal-435",
    "ransomware-signal-436",
    "botnet-signal-437",
    "command-control-signal-438",
    "exfiltration-signal-439",
    "lateral-movement-signal-440",
    "alert-signal-441",
    "allow-signal-442",
    "block-signal-443",
    "drop-signal-444",
    "reject-signal-445",
    "pass-signal-446",
    "content-signal-447",
    "sid-signal-448",
    "rev-signal-449",
    "classtype-signal-450",
    "metadata-signal-451",
    "reference-signal-452",
    "flow-signal-453",
    "established-signal-454",
    "to_server-signal-455",
    "to_client-signal-456",
    "domain-signal-457",
    "url-signal-458",
    "hash-signal-459",
    "sha256-signal-460",
    "sha1-signal-461",
    "md5-signal-462",
    "cidr-signal-463",
    "input-signal-464",
    "output-signal-465",
    "forward-signal-466",
    "accept-signal-467",
    "deny-signal-468",
    "dns-signal-469",
    "http-signal-470",
    "tls-signal-471",
    "ssh-signal-472",
    "rdp-signal-473",
    "smb-signal-474",
    "ldap-signal-475",
    "kerberos-signal-476",
    "syslog-signal-477",
    "icmp-signal-478",
    "tcp-signal-479",
    "udp-signal-480",
    "scan-signal-481",
    "beacon-signal-482",
    "malware-signal-483",
    "phishing-signal-484",
    "ransomware-signal-485",
    "botnet-signal-486",
    "command-control-signal-487",
    "exfiltration-signal-488",
    "lateral-movement-signal-489",
    "alert-signal-490",
    "allow-signal-491",
    "block-signal-492",
    "drop-signal-493",
    "reject-signal-494",
    "pass-signal-495",
    "content-signal-496",
    "sid-signal-497",
    "rev-signal-498",
    "classtype-signal-499",
    "metadata-signal-500",
    "reference-signal-501",
    "flow-signal-502",
    "established-signal-503",
    "to_server-signal-504",
    "to_client-signal-505",
    "domain-signal-506",
    "url-signal-507",
    "hash-signal-508",
    "sha256-signal-509",
    "sha1-signal-510",
    "md5-signal-511",
    "cidr-signal-512",
    "input-signal-513",
    "output-signal-514",
    "forward-signal-515",
    "accept-signal-516",
    "deny-signal-517",
    "dns-signal-518",
    "http-signal-519",
    "tls-signal-520",
    "ssh-signal-521",
    "rdp-signal-522",
    "smb-signal-523",
    "ldap-signal-524",
    "kerberos-signal-525",
    "syslog-signal-526",
    "icmp-signal-527",
    "tcp-signal-528",
    "udp-signal-529",
    "scan-signal-530",
    "beacon-signal-531",
    "malware-signal-532",
    "phishing-signal-533",
    "ransomware-signal-534",
    "botnet-signal-535",
    "command-control-signal-536",
    "exfiltration-signal-537",
    "lateral-movement-signal-538",
    "alert-signal-539",
    "allow-signal-540",
    "block-signal-541",
    "drop-signal-542",
    "reject-signal-543",
    "pass-signal-544",
    "content-signal-545",
    "sid-signal-546",
    "rev-signal-547",
    "classtype-signal-548",
    "metadata-signal-549",
    "reference-signal-550",
    "flow-signal-551",
    "established-signal-552",
    "to_server-signal-553",
    "to_client-signal-554",
    "domain-signal-555",
    "url-signal-556",
    "hash-signal-557",
    "sha256-signal-558",
    "sha1-signal-559",
    "md5-signal-560",
    "cidr-signal-561",
    "input-signal-562",
    "output-signal-563",
    "forward-signal-564",
    "accept-signal-565",
    "deny-signal-566",
    "dns-signal-567",
    "http-signal-568",
    "tls-signal-569",
    "ssh-signal-570",
    "rdp-signal-571",
    "smb-signal-572",
    "ldap-signal-573",
    "kerberos-signal-574",
    "syslog-signal-575",
    "icmp-signal-576",
    "tcp-signal-577",
    "udp-signal-578",
    "scan-signal-579",
    "beacon-signal-580",
    "malware-signal-581",
    "phishing-signal-582",
    "ransomware-signal-583",
    "botnet-signal-584",
    "command-control-signal-585",
    "exfiltration-signal-586",
    "lateral-movement-signal-587",
    "alert-signal-588",
    "allow-signal-589",
    "block-signal-590",
    "drop-signal-591",
    "reject-signal-592",
    "pass-signal-593",
    "content-signal-594",
    "sid-signal-595",
    "rev-signal-596",
    "classtype-signal-597",
    "metadata-signal-598",
    "reference-signal-599",
    "flow-signal-600",
    "established-signal-601",
    "to_server-signal-602",
    "to_client-signal-603",
    "domain-signal-604",
    "url-signal-605",
    "hash-signal-606",
    "sha256-signal-607",
    "sha1-signal-608",
    "md5-signal-609",
    "cidr-signal-610",
    "input-signal-611",
    "output-signal-612",
    "forward-signal-613",
    "accept-signal-614",
    "deny-signal-615",
    "dns-signal-616",
    "http-signal-617",
    "tls-signal-618",
    "ssh-signal-619",
    "rdp-signal-620",
    "smb-signal-621",
    "ldap-signal-622",
    "kerberos-signal-623",
    "syslog-signal-624",
    "icmp-signal-625",
    "tcp-signal-626",
    "udp-signal-627",
    "scan-signal-628",
    "beacon-signal-629",
    "malware-signal-630",
    "phishing-signal-631",
    "ransomware-signal-632",
    "botnet-signal-633",
    "command-control-signal-634",
    "exfiltration-signal-635",
    "lateral-movement-signal-636",
    "alert-signal-637",
    "allow-signal-638",
    "block-signal-639",
    "drop-signal-640",
    "reject-signal-641",
    "pass-signal-642",
    "content-signal-643",
    "sid-signal-644",
    "rev-signal-645",
    "classtype-signal-646",
    "metadata-signal-647",
    "reference-signal-648",
    "flow-signal-649",
    "established-signal-650",
    "to_server-signal-651",
    "to_client-signal-652",
    "domain-signal-653",
    "url-signal-654",
    "hash-signal-655",
    "sha256-signal-656",
    "sha1-signal-657",
    "md5-signal-658",
    "cidr-signal-659",
    "input-signal-660",
    "output-signal-661",
    "forward-signal-662",
    "accept-signal-663",
    "deny-signal-664",
    "dns-signal-665",
    "http-signal-666",
    "tls-signal-667",
    "ssh-signal-668",
    "rdp-signal-669",
    "smb-signal-670",
    "ldap-signal-671",
    "kerberos-signal-672",
    "syslog-signal-673",
    "icmp-signal-674",
    "tcp-signal-675",
    "udp-signal-676",
    "scan-signal-677",
    "beacon-signal-678",
    "malware-signal-679",
    "phishing-signal-680",
    "ransomware-signal-681",
    "botnet-signal-682",
    "command-control-signal-683",
    "exfiltration-signal-684",
    "lateral-movement-signal-685",
    "alert-signal-686",
    "allow-signal-687",
    "block-signal-688",
    "drop-signal-689",
    "reject-signal-690",
    "pass-signal-691",
    "content-signal-692",
    "sid-signal-693",
    "rev-signal-694",
    "classtype-signal-695",
    "metadata-signal-696",
    "reference-signal-697",
    "flow-signal-698",
    "established-signal-699",
    "to_server-signal-700",
    "to_client-signal-701",
    "domain-signal-702",
    "url-signal-703",
    "hash-signal-704",
    "sha256-signal-705",
    "sha1-signal-706",
    "md5-signal-707",
    "cidr-signal-708",
    "input-signal-709",
    "output-signal-710",
    "forward-signal-711",
    "accept-signal-712",
    "deny-signal-713",
    "dns-signal-714",
    "http-signal-715",
    "tls-signal-716",
    "ssh-signal-717",
    "rdp-signal-718",
    "smb-signal-719",
    "ldap-signal-720",
    "kerberos-signal-721",
    "syslog-signal-722",
    "icmp-signal-723",
    "tcp-signal-724",
    "udp-signal-725",
    "scan-signal-726",
    "beacon-signal-727",
    "malware-signal-728",
    "phishing-signal-729",
    "ransomware-signal-730",
    "botnet-signal-731",
    "command-control-signal-732",
    "exfiltration-signal-733",
    "lateral-movement-signal-734",
    "alert-signal-735",
    "allow-signal-736",
    "block-signal-737",
    "drop-signal-738",
    "reject-signal-739",
    "pass-signal-740",
    "content-signal-741",
    "sid-signal-742",
    "rev-signal-743",
    "classtype-signal-744",
    "metadata-signal-745",
    "reference-signal-746",
    "flow-signal-747",
    "established-signal-748",
    "to_server-signal-749",
    "to_client-signal-750",
    "domain-signal-751",
    "url-signal-752",
    "hash-signal-753",
    "sha256-signal-754",
    "sha1-signal-755",
    "md5-signal-756",
    "cidr-signal-757",
    "input-signal-758",
    "output-signal-759",
    "forward-signal-760",
    "accept-signal-761",
    "deny-signal-762",
    "dns-signal-763",
    "http-signal-764",
    "tls-signal-765",
    "ssh-signal-766",
    "rdp-signal-767",
    "smb-signal-768",
    "ldap-signal-769",
    "kerberos-signal-770",
    "syslog-signal-771",
    "icmp-signal-772",
    "tcp-signal-773",
    "udp-signal-774",
    "scan-signal-775",
    "beacon-signal-776",
    "malware-signal-777",
    "phishing-signal-778",
    "ransomware-signal-779",
    "botnet-signal-780",
    "command-control-signal-781",
    "exfiltration-signal-782",
    "lateral-movement-signal-783",
    "alert-signal-784",
    "allow-signal-785",
    "block-signal-786",
    "drop-signal-787",
    "reject-signal-788",
    "pass-signal-789",
    "content-signal-790",
    "sid-signal-791",
    "rev-signal-792",
    "classtype-signal-793",
    "metadata-signal-794",
    "reference-signal-795",
    "flow-signal-796",
    "established-signal-797",
    "to_server-signal-798",
    "to_client-signal-799",
    "domain-signal-800",
    "url-signal-801",
    "hash-signal-802",
    "sha256-signal-803",
    "sha1-signal-804",
    "md5-signal-805",
    "cidr-signal-806",
    "input-signal-807",
    "output-signal-808",
    "forward-signal-809",
    "accept-signal-810",
    "deny-signal-811",
    "dns-signal-812",
    "http-signal-813",
    "tls-signal-814",
    "ssh-signal-815",
    "rdp-signal-816",
    "smb-signal-817",
    "ldap-signal-818",
    "kerberos-signal-819",
    "syslog-signal-820",
    "icmp-signal-821",
    "tcp-signal-822",
    "udp-signal-823",
    "scan-signal-824",
    "beacon-signal-825",
    "malware-signal-826",
    "phishing-signal-827",
    "ransomware-signal-828",
    "botnet-signal-829",
    "command-control-signal-830",
    "exfiltration-signal-831",
    "lateral-movement-signal-832",
    "alert-signal-833",
    "allow-signal-834",
    "block-signal-835",
    "drop-signal-836",
    "reject-signal-837",
    "pass-signal-838",
    "content-signal-839",
    "sid-signal-840",
    "rev-signal-841",
    "classtype-signal-842",
    "metadata-signal-843",
    "reference-signal-844",
    "flow-signal-845",
    "established-signal-846",
    "to_server-signal-847",
    "to_client-signal-848",
    "domain-signal-849",
    "url-signal-850",
    "hash-signal-851",
    "sha256-signal-852",
    "sha1-signal-853",
    "md5-signal-854",
    "cidr-signal-855",
    "input-signal-856",
    "output-signal-857",
    "forward-signal-858",
    "accept-signal-859",
    "deny-signal-860",
    "dns-signal-861",
    "http-signal-862",
    "tls-signal-863",
    "ssh-signal-864",
    "rdp-signal-865",
    "smb-signal-866",
    "ldap-signal-867",
    "kerberos-signal-868",
    "syslog-signal-869",
    "icmp-signal-870",
    "tcp-signal-871",
    "udp-signal-872",
    "scan-signal-873",
    "beacon-signal-874",
    "malware-signal-875",
    "phishing-signal-876",
    "ransomware-signal-877",
    "botnet-signal-878",
    "command-control-signal-879",
    "exfiltration-signal-880",
    "lateral-movement-signal-881",
    "alert-signal-882",
    "allow-signal-883",
    "block-signal-884",
    "drop-signal-885",
    "reject-signal-886",
    "pass-signal-887",
    "content-signal-888",
    "sid-signal-889",
    "rev-signal-890",
    "classtype-signal-891",
    "metadata-signal-892",
    "reference-signal-893",
    "flow-signal-894",
    "established-signal-895",
    "to_server-signal-896",
    "to_client-signal-897",
    "domain-signal-898",
    "url-signal-899",
    "hash-signal-900",
    "sha256-signal-901",
    "sha1-signal-902",
    "md5-signal-903",
    "cidr-signal-904",
    "input-signal-905",
    "output-signal-906",
    "forward-signal-907",
    "accept-signal-908",
    "deny-signal-909",
    "dns-signal-910",
    "http-signal-911",
    "tls-signal-912",
    "ssh-signal-913",
    "rdp-signal-914",
    "smb-signal-915",
    "ldap-signal-916",
    "kerberos-signal-917",
    "syslog-signal-918",
    "icmp-signal-919",
    "tcp-signal-920",
    "udp-signal-921",
    "scan-signal-922",
    "beacon-signal-923",
    "malware-signal-924",
    "phishing-signal-925",
    "ransomware-signal-926",
    "botnet-signal-927",
    "command-control-signal-928",
    "exfiltration-signal-929",
    "lateral-movement-signal-930",
    "alert-signal-931",
    "allow-signal-932",
    "block-signal-933",
    "drop-signal-934",
    "reject-signal-935",
    "pass-signal-936",
    "content-signal-937",
    "sid-signal-938",
    "rev-signal-939",
    "classtype-signal-940",
    "metadata-signal-941",
    "reference-signal-942",
    "flow-signal-943",
    "established-signal-944",
    "to_server-signal-945",
    "to_client-signal-946",
    "domain-signal-947",
    "url-signal-948",
    "hash-signal-949",
    "sha256-signal-950",
    "sha1-signal-951",
    "md5-signal-952",
    "cidr-signal-953",
    "input-signal-954",
    "output-signal-955",
    "forward-signal-956",
    "accept-signal-957",
    "deny-signal-958",
    "dns-signal-959",
    "http-signal-960",
    "tls-signal-961",
    "ssh-signal-962",
    "rdp-signal-963",
    "smb-signal-964",
    "ldap-signal-965",
    "kerberos-signal-966",
    "syslog-signal-967",
    "icmp-signal-968",
    "tcp-signal-969",
    "udp-signal-970",
    "scan-signal-971",
    "beacon-signal-972",
    "malware-signal-973",
    "phishing-signal-974",
    "ransomware-signal-975",
    "botnet-signal-976",
    "command-control-signal-977",
    "exfiltration-signal-978",
    "lateral-movement-signal-979",
    "alert-signal-980",
    "allow-signal-981",
    "block-signal-982",
    "drop-signal-983",
    "reject-signal-984",
    "pass-signal-985",
    "content-signal-986",
    "sid-signal-987",
    "rev-signal-988",
    "classtype-signal-989",
    "metadata-signal-990",
    "reference-signal-991",
    "flow-signal-992",
    "established-signal-993",
    "to_server-signal-994",
    "to_client-signal-995",
    "domain-signal-996",
    "url-signal-997",
    "hash-signal-998",
    "sha256-signal-999",
    "sha1-signal-1000",
    "md5-signal-1001",
    "cidr-signal-1002",
    "input-signal-1003",
    "output-signal-1004",
    "forward-signal-1005",
    "accept-signal-1006",
    "deny-signal-1007",
    "dns-signal-1008",
    "http-signal-1009",
    "tls-signal-1010",
    "ssh-signal-1011",
    "rdp-signal-1012",
    "smb-signal-1013",
    "ldap-signal-1014",
    "kerberos-signal-1015",
    "syslog-signal-1016",
    "icmp-signal-1017",
    "tcp-signal-1018",
    "udp-signal-1019",
    "scan-signal-1020",
    "beacon-signal-1021",
    "malware-signal-1022",
    "phishing-signal-1023",
    "ransomware-signal-1024",
    "botnet-signal-1025",
    "command-control-signal-1026",
    "exfiltration-signal-1027",
    "lateral-movement-signal-1028",
    "alert-signal-1029",
    "allow-signal-1030",
    "block-signal-1031",
    "drop-signal-1032",
    "reject-signal-1033",
    "pass-signal-1034",
    "content-signal-1035",
    "sid-signal-1036",
    "rev-signal-1037",
    "classtype-signal-1038",
    "metadata-signal-1039",
    "reference-signal-1040",
    "flow-signal-1041",
    "established-signal-1042",
    "to_server-signal-1043",
    "to_client-signal-1044",
    "domain-signal-1045",
    "url-signal-1046",
    "hash-signal-1047",
    "sha256-signal-1048",
    "sha1-signal-1049",
    "md5-signal-1050",
    "cidr-signal-1051",
    "input-signal-1052",
    "output-signal-1053",
    "forward-signal-1054",
    "accept-signal-1055",
    "deny-signal-1056",
    "dns-signal-1057",
    "http-signal-1058",
    "tls-signal-1059",
    "ssh-signal-1060",
    "rdp-signal-1061",
    "smb-signal-1062",
    "ldap-signal-1063",
    "kerberos-signal-1064",
    "syslog-signal-1065",
    "icmp-signal-1066",
    "tcp-signal-1067",
    "udp-signal-1068",
    "scan-signal-1069",
    "beacon-signal-1070",
    "malware-signal-1071",
    "phishing-signal-1072",
    "ransomware-signal-1073",
    "botnet-signal-1074",
    "command-control-signal-1075",
    "exfiltration-signal-1076",
    "lateral-movement-signal-1077",
    "alert-signal-1078",
    "allow-signal-1079",
    "block-signal-1080",
    "drop-signal-1081",
    "reject-signal-1082",
    "pass-signal-1083",
    "content-signal-1084",
    "sid-signal-1085",
    "rev-signal-1086",
    "classtype-signal-1087",
    "metadata-signal-1088",
    "reference-signal-1089",
    "flow-signal-1090",
    "established-signal-1091",
    "to_server-signal-1092",
    "to_client-signal-1093",
    "domain-signal-1094",
    "url-signal-1095",
    "hash-signal-1096",
    "sha256-signal-1097",
    "sha1-signal-1098",
    "md5-signal-1099",
    "cidr-signal-1100",
    "input-signal-1101",
    "output-signal-1102",
    "forward-signal-1103",
    "accept-signal-1104",
    "deny-signal-1105",
    "dns-signal-1106",
    "http-signal-1107",
    "tls-signal-1108",
    "ssh-signal-1109",
    "rdp-signal-1110",
    "smb-signal-1111",
    "ldap-signal-1112",
    "kerberos-signal-1113",
    "syslog-signal-1114",
    "icmp-signal-1115",
    "tcp-signal-1116",
    "udp-signal-1117",
    "scan-signal-1118",
    "beacon-signal-1119",
    "malware-signal-1120",
    "phishing-signal-1121",
    "ransomware-signal-1122",
    "botnet-signal-1123",
    "command-control-signal-1124",
    "exfiltration-signal-1125",
    "lateral-movement-signal-1126",
    "alert-signal-1127",
    "allow-signal-1128",
    "block-signal-1129",
    "drop-signal-1130",
    "reject-signal-1131",
    "pass-signal-1132",
    "content-signal-1133",
    "sid-signal-1134",
    "rev-signal-1135",
    "classtype-signal-1136",
    "metadata-signal-1137",
    "reference-signal-1138",
    "flow-signal-1139",
    "established-signal-1140",
    "to_server-signal-1141",
    "to_client-signal-1142",
    "domain-signal-1143",
    "url-signal-1144",
    "hash-signal-1145",
    "sha256-signal-1146",
    "sha1-signal-1147",
    "md5-signal-1148",
    "cidr-signal-1149",
    "input-signal-1150",
    "output-signal-1151",
    "forward-signal-1152",
    "accept-signal-1153",
    "deny-signal-1154",
    "dns-signal-1155",
    "http-signal-1156",
    "tls-signal-1157",
    "ssh-signal-1158",
    "rdp-signal-1159",
    "smb-signal-1160",
    "ldap-signal-1161",
    "kerberos-signal-1162",
    "syslog-signal-1163",
    "icmp-signal-1164",
    "tcp-signal-1165",
    "udp-signal-1166",
    "scan-signal-1167",
    "beacon-signal-1168",
    "malware-signal-1169",
    "phishing-signal-1170",
    "ransomware-signal-1171",
    "botnet-signal-1172",
    "command-control-signal-1173",
    "exfiltration-signal-1174",
    "lateral-movement-signal-1175",
    "alert-signal-1176",
    "allow-signal-1177",
    "block-signal-1178",
    "drop-signal-1179",
    "reject-signal-1180",
    "pass-signal-1181",
    "content-signal-1182",
    "sid-signal-1183",
    "rev-signal-1184",
    "classtype-signal-1185",
    "metadata-signal-1186",
    "reference-signal-1187",
    "flow-signal-1188",
    "established-signal-1189",
    "to_server-signal-1190",
    "to_client-signal-1191",
    "domain-signal-1192",
    "url-signal-1193",
    "hash-signal-1194",
    "sha256-signal-1195",
    "sha1-signal-1196",
    "md5-signal-1197",
    "cidr-signal-1198",
    "input-signal-1199",
    "output-signal-1200",
    "forward-signal-1201",
    "accept-signal-1202",
    "deny-signal-1203",
    "dns-signal-1204",
    "http-signal-1205",
    "tls-signal-1206",
    "ssh-signal-1207",
    "rdp-signal-1208",
    "smb-signal-1209",
    "ldap-signal-1210",
    "kerberos-signal-1211",
    "syslog-signal-1212",
    "icmp-signal-1213",
    "tcp-signal-1214",
    "udp-signal-1215",
    "scan-signal-1216",
    "beacon-signal-1217",
    "malware-signal-1218",
    "phishing-signal-1219",
    "ransomware-signal-1220",
    "botnet-signal-1221",
    "command-control-signal-1222",
    "exfiltration-signal-1223",
    "lateral-movement-signal-1224",
    "alert-signal-1225",
    "allow-signal-1226",
    "block-signal-1227",
    "drop-signal-1228",
    "reject-signal-1229",
    "pass-signal-1230",
    "content-signal-1231",
    "sid-signal-1232",
    "rev-signal-1233",
    "classtype-signal-1234",
    "metadata-signal-1235",
    "reference-signal-1236",
    "flow-signal-1237",
    "established-signal-1238",
    "to_server-signal-1239",
    "to_client-signal-1240",
    "domain-signal-1241",
    "url-signal-1242",
    "hash-signal-1243",
    "sha256-signal-1244",
    "sha1-signal-1245",
    "md5-signal-1246",
    "cidr-signal-1247",
    "input-signal-1248",
    "output-signal-1249",
    "forward-signal-1250",
    "accept-signal-1251",
    "deny-signal-1252",
    "dns-signal-1253",
    "http-signal-1254",
    "tls-signal-1255",
    "ssh-signal-1256",
    "rdp-signal-1257",
    "smb-signal-1258",
    "ldap-signal-1259",
    "kerberos-signal-1260",
    "syslog-signal-1261",
    "icmp-signal-1262",
    "tcp-signal-1263",
    "udp-signal-1264",
    "scan-signal-1265",
    "beacon-signal-1266",
    "malware-signal-1267",
    "phishing-signal-1268",
    "ransomware-signal-1269",
    "botnet-signal-1270",
    "command-control-signal-1271",
    "exfiltration-signal-1272",
    "lateral-movement-signal-1273",
    "alert-signal-1274",
    "allow-signal-1275",
    "block-signal-1276",
    "drop-signal-1277",
    "reject-signal-1278",
    "pass-signal-1279",
    "content-signal-1280",
    "sid-signal-1281",
    "rev-signal-1282",
    "classtype-signal-1283",
    "metadata-signal-1284",
    "reference-signal-1285",
    "flow-signal-1286",
    "established-signal-1287",
    "to_server-signal-1288",
    "to_client-signal-1289",
    "domain-signal-1290",
    "url-signal-1291",
    "hash-signal-1292",
    "sha256-signal-1293",
    "sha1-signal-1294",
    "md5-signal-1295",
    "cidr-signal-1296",
    "input-signal-1297",
    "output-signal-1298",
    "forward-signal-1299",
    "accept-signal-1300",
    "deny-signal-1301",
    "dns-signal-1302",
    "http-signal-1303",
    "tls-signal-1304",
    "ssh-signal-1305",
    "rdp-signal-1306",
    "smb-signal-1307",
    "ldap-signal-1308",
    "kerberos-signal-1309",
    "syslog-signal-1310",
    "icmp-signal-1311",
    "tcp-signal-1312",
    "udp-signal-1313",
    "scan-signal-1314",
    "beacon-signal-1315",
    "malware-signal-1316",
    "phishing-signal-1317",
    "ransomware-signal-1318",
    "botnet-signal-1319",
    "command-control-signal-1320",
    "exfiltration-signal-1321",
    "lateral-movement-signal-1322",
    "alert-signal-1323",
    "allow-signal-1324",
    "block-signal-1325",
    "drop-signal-1326",
    "reject-signal-1327",
    "pass-signal-1328",
    "content-signal-1329",
    "sid-signal-1330",
    "rev-signal-1331",
    "classtype-signal-1332",
    "metadata-signal-1333",
    "reference-signal-1334",
    "flow-signal-1335",
    "established-signal-1336",
    "to_server-signal-1337",
    "to_client-signal-1338",
    "domain-signal-1339",
    "url-signal-1340",
    "hash-signal-1341",
    "sha256-signal-1342",
    "sha1-signal-1343",
    "md5-signal-1344",
    "cidr-signal-1345",
    "input-signal-1346",
    "output-signal-1347",
    "forward-signal-1348",
    "accept-signal-1349",
    "deny-signal-1350",
    "dns-signal-1351",
    "http-signal-1352",
    "tls-signal-1353",
    "ssh-signal-1354",
    "rdp-signal-1355",
    "smb-signal-1356",
    "ldap-signal-1357",
    "kerberos-signal-1358",
    "syslog-signal-1359",
    "icmp-signal-1360",
    "tcp-signal-1361",
    "udp-signal-1362",
    "scan-signal-1363",
    "beacon-signal-1364",
    "malware-signal-1365",
    "phishing-signal-1366",
    "ransomware-signal-1367",
    "botnet-signal-1368",
    "command-control-signal-1369",
    "exfiltration-signal-1370",
    "lateral-movement-signal-1371",
    "alert-signal-1372",
    "allow-signal-1373",
    "block-signal-1374",
    "drop-signal-1375",
    "reject-signal-1376",
    "pass-signal-1377",
    "content-signal-1378",
    "sid-signal-1379",
    "rev-signal-1380",
    "classtype-signal-1381",
    "metadata-signal-1382",
    "reference-signal-1383",
    "flow-signal-1384",
    "established-signal-1385",
    "to_server-signal-1386",
    "to_client-signal-1387",
    "domain-signal-1388",
    "url-signal-1389",
    "hash-signal-1390",
    "sha256-signal-1391",
    "sha1-signal-1392",
    "md5-signal-1393",
    "cidr-signal-1394",
    "input-signal-1395",
    "output-signal-1396",
    "forward-signal-1397",
    "accept-signal-1398",
    "deny-signal-1399",
    "dns-signal-1400",
    "http-signal-1401",
    "tls-signal-1402",
    "ssh-signal-1403",
    "rdp-signal-1404",
    "smb-signal-1405",
    "ldap-signal-1406",
    "kerberos-signal-1407",
    "syslog-signal-1408",
    "icmp-signal-1409",
    "tcp-signal-1410",
    "udp-signal-1411",
    "scan-signal-1412",
    "beacon-signal-1413",
    "malware-signal-1414",
    "phishing-signal-1415",
    "ransomware-signal-1416",
    "botnet-signal-1417",
    "command-control-signal-1418",
    "exfiltration-signal-1419",
    "lateral-movement-signal-1420",
    "alert-signal-1421",
    "allow-signal-1422",
    "block-signal-1423",
    "drop-signal-1424",
    "reject-signal-1425",
    "pass-signal-1426",
    "content-signal-1427",
    "sid-signal-1428",
    "rev-signal-1429",
    "classtype-signal-1430",
    "metadata-signal-1431",
    "reference-signal-1432",
    "flow-signal-1433",
    "established-signal-1434",
    "to_server-signal-1435",
    "to_client-signal-1436",
    "domain-signal-1437",
    "url-signal-1438",
    "hash-signal-1439",
    "sha256-signal-1440",
    "sha1-signal-1441",
    "md5-signal-1442",
    "cidr-signal-1443",
    "input-signal-1444",
    "output-signal-1445",
    "forward-signal-1446",
    "accept-signal-1447",
    "deny-signal-1448",
    "dns-signal-1449",
    "http-signal-1450",
    "tls-signal-1451",
    "ssh-signal-1452",
    "rdp-signal-1453",
    "smb-signal-1454",
    "ldap-signal-1455",
    "kerberos-signal-1456",
    "syslog-signal-1457",
    "icmp-signal-1458",
    "tcp-signal-1459",
    "udp-signal-1460",
    "scan-signal-1461",
    "beacon-signal-1462",
    "malware-signal-1463",
    "phishing-signal-1464",
    "ransomware-signal-1465",
    "botnet-signal-1466",
    "command-control-signal-1467",
    "exfiltration-signal-1468",
    "lateral-movement-signal-1469",
    "alert-signal-1470",
    "allow-signal-1471",
    "block-signal-1472",
    "drop-signal-1473",
    "reject-signal-1474",
    "pass-signal-1475",
    "content-signal-1476",
    "sid-signal-1477",
    "rev-signal-1478",
    "classtype-signal-1479",
    "metadata-signal-1480",
    "reference-signal-1481",
    "flow-signal-1482",
    "established-signal-1483",
    "to_server-signal-1484",
    "to_client-signal-1485",
    "domain-signal-1486",
    "url-signal-1487",
    "hash-signal-1488",
    "sha256-signal-1489",
    "sha1-signal-1490",
    "md5-signal-1491",
    "cidr-signal-1492",
    "input-signal-1493",
    "output-signal-1494",
    "forward-signal-1495",
    "accept-signal-1496",
    "deny-signal-1497",
    "dns-signal-1498",
    "http-signal-1499",
    "tls-signal-1500",
    "ssh-signal-1501",
    "rdp-signal-1502",
    "smb-signal-1503",
    "ldap-signal-1504",
    "kerberos-signal-1505",
    "syslog-signal-1506",
    "icmp-signal-1507",
    "tcp-signal-1508",
    "udp-signal-1509",
    "scan-signal-1510",
    "beacon-signal-1511",
    "malware-signal-1512",
    "phishing-signal-1513",
    "ransomware-signal-1514",
    "botnet-signal-1515",
    "command-control-signal-1516",
    "exfiltration-signal-1517",
    "lateral-movement-signal-1518",
    "alert-signal-1519",
    "allow-signal-1520",
    "block-signal-1521",
    "drop-signal-1522",
    "reject-signal-1523",
    "pass-signal-1524",
    "content-signal-1525",
    "sid-signal-1526",
    "rev-signal-1527",
    "classtype-signal-1528",
    "metadata-signal-1529",
    "reference-signal-1530",
    "flow-signal-1531",
    "established-signal-1532",
    "to_server-signal-1533",
    "to_client-signal-1534",
    "domain-signal-1535",
    "url-signal-1536",
    "hash-signal-1537",
    "sha256-signal-1538",
    "sha1-signal-1539",
    "md5-signal-1540",
    "cidr-signal-1541",
    "input-signal-1542",
    "output-signal-1543",
    "forward-signal-1544",
    "accept-signal-1545",
    "deny-signal-1546",
    "dns-signal-1547",
    "http-signal-1548",
    "tls-signal-1549",
    "ssh-signal-1550",
    "rdp-signal-1551",
    "smb-signal-1552",
    "ldap-signal-1553",
    "kerberos-signal-1554",
    "syslog-signal-1555",
    "icmp-signal-1556",
    "tcp-signal-1557",
    "udp-signal-1558",
    "scan-signal-1559",
    "beacon-signal-1560",
    "malware-signal-1561",
    "phishing-signal-1562",
    "ransomware-signal-1563",
    "botnet-signal-1564",
    "command-control-signal-1565",
    "exfiltration-signal-1566",
    "lateral-movement-signal-1567",
    "alert-signal-1568",
    "allow-signal-1569",
    "block-signal-1570",
    "drop-signal-1571",
    "reject-signal-1572",
    "pass-signal-1573",
    "content-signal-1574",
    "sid-signal-1575",
    "rev-signal-1576",
    "classtype-signal-1577",
    "metadata-signal-1578",
    "reference-signal-1579",
    "flow-signal-1580",
    "established-signal-1581",
    "to_server-signal-1582",
    "to_client-signal-1583",
    "domain-signal-1584",
    "url-signal-1585",
    "hash-signal-1586",
    "sha256-signal-1587",
    "sha1-signal-1588",
    "md5-signal-1589",
    "cidr-signal-1590",
    "input-signal-1591",
    "output-signal-1592",
    "forward-signal-1593",
    "accept-signal-1594",
    "deny-signal-1595",
    "dns-signal-1596",
    "http-signal-1597",
    "tls-signal-1598",
    "ssh-signal-1599",
    "rdp-signal-1600",
    "smb-signal-1601",
    "ldap-signal-1602",
    "kerberos-signal-1603",
    "syslog-signal-1604",
    "icmp-signal-1605",
    "tcp-signal-1606",
    "udp-signal-1607",
    "scan-signal-1608",
    "beacon-signal-1609",
    "malware-signal-1610",
    "phishing-signal-1611",
    "ransomware-signal-1612",
    "botnet-signal-1613",
    "command-control-signal-1614",
    "exfiltration-signal-1615",
    "lateral-movement-signal-1616",
    "alert-signal-1617",
    "allow-signal-1618",
    "block-signal-1619",
    "drop-signal-1620",
    "reject-signal-1621",
    "pass-signal-1622",
    "content-signal-1623",
    "sid-signal-1624",
    "rev-signal-1625",
    "classtype-signal-1626",
    "metadata-signal-1627",
    "reference-signal-1628",
    "flow-signal-1629",
    "established-signal-1630",
    "to_server-signal-1631",
    "to_client-signal-1632",
    "domain-signal-1633",
    "url-signal-1634",
    "hash-signal-1635",
    "sha256-signal-1636",
    "sha1-signal-1637",
    "md5-signal-1638",
    "cidr-signal-1639",
    "input-signal-1640",
    "output-signal-1641",
    "forward-signal-1642",
    "accept-signal-1643",
    "deny-signal-1644",
    "dns-signal-1645",
    "http-signal-1646",
    "tls-signal-1647",
    "ssh-signal-1648",
    "rdp-signal-1649",
    "smb-signal-1650",
    "ldap-signal-1651",
    "kerberos-signal-1652",
    "syslog-signal-1653",
    "icmp-signal-1654",
    "tcp-signal-1655",
    "udp-signal-1656",
    "scan-signal-1657",
    "beacon-signal-1658",
    "malware-signal-1659",
    "phishing-signal-1660",
    "ransomware-signal-1661",
    "botnet-signal-1662",
    "command-control-signal-1663",
    "exfiltration-signal-1664",
    "lateral-movement-signal-1665",
    "alert-signal-1666",
    "allow-signal-1667",
    "block-signal-1668",
    "drop-signal-1669",
    "reject-signal-1670",
    "pass-signal-1671",
    "content-signal-1672",
    "sid-signal-1673",
    "rev-signal-1674",
    "classtype-signal-1675",
    "metadata-signal-1676",
    "reference-signal-1677",
    "flow-signal-1678",
    "established-signal-1679",
    "to_server-signal-1680",
    "to_client-signal-1681",
    "domain-signal-1682",
    "url-signal-1683",
    "hash-signal-1684",
    "sha256-signal-1685",
    "sha1-signal-1686",
    "md5-signal-1687",
    "cidr-signal-1688",
    "input-signal-1689",
    "output-signal-1690",
    "forward-signal-1691",
    "accept-signal-1692",
    "deny-signal-1693",
    "dns-signal-1694",
    "http-signal-1695",
    "tls-signal-1696",
    "ssh-signal-1697",
    "rdp-signal-1698",
    "smb-signal-1699",
    "ldap-signal-1700",
    "kerberos-signal-1701",
    "syslog-signal-1702",
    "icmp-signal-1703",
    "tcp-signal-1704",
    "udp-signal-1705",
    "scan-signal-1706",
    "beacon-signal-1707",
    "malware-signal-1708",
    "phishing-signal-1709",
    "ransomware-signal-1710",
    "botnet-signal-1711",
    "command-control-signal-1712",
    "exfiltration-signal-1713",
    "lateral-movement-signal-1714",
    "alert-signal-1715",
    "allow-signal-1716",
    "block-signal-1717",
    "drop-signal-1718",
    "reject-signal-1719",
    "pass-signal-1720",
    "content-signal-1721",
    "sid-signal-1722",
    "rev-signal-1723",
    "classtype-signal-1724",
    "metadata-signal-1725",
    "reference-signal-1726",
    "flow-signal-1727",
    "established-signal-1728",
    "to_server-signal-1729",
    "to_client-signal-1730",
    "domain-signal-1731",
    "url-signal-1732",
    "hash-signal-1733",
    "sha256-signal-1734",
    "sha1-signal-1735",
    "md5-signal-1736",
    "cidr-signal-1737",
    "input-signal-1738",
    "output-signal-1739",
    "forward-signal-1740",
    "accept-signal-1741",
    "deny-signal-1742",
    "dns-signal-1743",
    "http-signal-1744",
    "tls-signal-1745",
    "ssh-signal-1746",
    "rdp-signal-1747",
    "smb-signal-1748",
    "ldap-signal-1749",
    "kerberos-signal-1750",
    "syslog-signal-1751",
    "icmp-signal-1752",
    "tcp-signal-1753",
    "udp-signal-1754",
    "scan-signal-1755",
    "beacon-signal-1756",
    "malware-signal-1757",
    "phishing-signal-1758",
    "ransomware-signal-1759",
    "botnet-signal-1760",
    "command-control-signal-1761",
    "exfiltration-signal-1762",
    "lateral-movement-signal-1763",
    "alert-signal-1764",
    "allow-signal-1765",
    "block-signal-1766",
    "drop-signal-1767",
    "reject-signal-1768",
    "pass-signal-1769",
    "content-signal-1770",
    "sid-signal-1771",
    "rev-signal-1772",
    "classtype-signal-1773",
    "metadata-signal-1774",
    "reference-signal-1775",
    "flow-signal-1776",
    "established-signal-1777",
    "to_server-signal-1778",
    "to_client-signal-1779",
    "domain-signal-1780",
    "url-signal-1781",
    "hash-signal-1782",
    "sha256-signal-1783",
    "sha1-signal-1784",
    "md5-signal-1785",
    "cidr-signal-1786",
    "input-signal-1787",
    "output-signal-1788",
    "forward-signal-1789",
    "accept-signal-1790",
    "deny-signal-1791",
    "dns-signal-1792",
    "http-signal-1793",
    "tls-signal-1794",
    "ssh-signal-1795",
    "rdp-signal-1796",
    "smb-signal-1797",
    "ldap-signal-1798",
    "kerberos-signal-1799",
    "syslog-signal-1800",
    "icmp-signal-1801",
    "tcp-signal-1802",
    "udp-signal-1803",
    "scan-signal-1804",
    "beacon-signal-1805",
    "malware-signal-1806",
    "phishing-signal-1807",
    "ransomware-signal-1808",
    "botnet-signal-1809",
    "command-control-signal-1810",
    "exfiltration-signal-1811",
    "lateral-movement-signal-1812",
    "alert-signal-1813",
    "allow-signal-1814",
    "block-signal-1815",
    "drop-signal-1816",
    "reject-signal-1817",
    "pass-signal-1818",
    "content-signal-1819",
    "sid-signal-1820",
    "rev-signal-1821",
    "classtype-signal-1822",
    "metadata-signal-1823",
    "reference-signal-1824",
    "flow-signal-1825",
    "established-signal-1826",
    "to_server-signal-1827",
    "to_client-signal-1828",
    "domain-signal-1829",
    "url-signal-1830",
    "hash-signal-1831",
    "sha256-signal-1832",
    "sha1-signal-1833",
    "md5-signal-1834",
    "cidr-signal-1835",
    "input-signal-1836",
    "output-signal-1837",
    "forward-signal-1838",
    "accept-signal-1839",
    "deny-signal-1840",
    "dns-signal-1841",
    "http-signal-1842",
    "tls-signal-1843",
    "ssh-signal-1844",
    "rdp-signal-1845",
    "smb-signal-1846",
    "ldap-signal-1847",
    "kerberos-signal-1848",
    "syslog-signal-1849",
    "icmp-signal-1850",
    "tcp-signal-1851",
    "udp-signal-1852",
    "scan-signal-1853",
    "beacon-signal-1854",
    "malware-signal-1855",
    "phishing-signal-1856",
    "ransomware-signal-1857",
    "botnet-signal-1858",
    "command-control-signal-1859",
    "exfiltration-signal-1860",
    "lateral-movement-signal-1861",
    "alert-signal-1862",
    "allow-signal-1863",
    "block-signal-1864",
    "drop-signal-1865",
    "reject-signal-1866",
    "pass-signal-1867",
    "content-signal-1868",
    "sid-signal-1869",
    "rev-signal-1870",
    "classtype-signal-1871",
    "metadata-signal-1872",
    "reference-signal-1873",
    "flow-signal-1874",
    "established-signal-1875",
    "to_server-signal-1876",
    "to_client-signal-1877",
    "domain-signal-1878",
    "url-signal-1879",
    "hash-signal-1880",
    "sha256-signal-1881",
    "sha1-signal-1882",
    "md5-signal-1883",
    "cidr-signal-1884",
    "input-signal-1885",
    "output-signal-1886",
    "forward-signal-1887",
    "accept-signal-1888",
    "deny-signal-1889",
    "dns-signal-1890",
    "http-signal-1891",
    "tls-signal-1892",
    "ssh-signal-1893",
    "rdp-signal-1894",
    "smb-signal-1895",
    "ldap-signal-1896",
    "kerberos-signal-1897",
    "syslog-signal-1898",
    "icmp-signal-1899",
    "tcp-signal-1900",
    "udp-signal-1901",
    "scan-signal-1902",
    "beacon-signal-1903",
    "malware-signal-1904",
    "phishing-signal-1905",
    "ransomware-signal-1906",
    "botnet-signal-1907",
    "command-control-signal-1908",
    "exfiltration-signal-1909",
    "lateral-movement-signal-1910",
    "alert-signal-1911",
    "allow-signal-1912",
    "block-signal-1913",
    "drop-signal-1914",
    "reject-signal-1915",
    "pass-signal-1916",
    "content-signal-1917",
    "sid-signal-1918",
    "rev-signal-1919",
    "classtype-signal-1920",
    "metadata-signal-1921",
    "reference-signal-1922",
    "flow-signal-1923",
    "established-signal-1924",
    "to_server-signal-1925",
    "to_client-signal-1926",
    "domain-signal-1927",
    "url-signal-1928",
    "hash-signal-1929",
    "sha256-signal-1930",
    "sha1-signal-1931",
    "md5-signal-1932",
    "cidr-signal-1933",
    "input-signal-1934",
    "output-signal-1935",
    "forward-signal-1936",
    "accept-signal-1937",
    "deny-signal-1938",
    "dns-signal-1939",
    "http-signal-1940",
    "tls-signal-1941",
    "ssh-signal-1942",
    "rdp-signal-1943",
    "smb-signal-1944",
    "ldap-signal-1945",
    "kerberos-signal-1946",
    "syslog-signal-1947",
    "icmp-signal-1948",
    "tcp-signal-1949",
    "udp-signal-1950",
    "scan-signal-1951",
    "beacon-signal-1952",
    "malware-signal-1953",
    "phishing-signal-1954",
    "ransomware-signal-1955",
    "botnet-signal-1956",
    "command-control-signal-1957",
    "exfiltration-signal-1958",
    "lateral-movement-signal-1959",
    "alert-signal-1960",
    "allow-signal-1961",
    "block-signal-1962",
    "drop-signal-1963",
    "reject-signal-1964",
    "pass-signal-1965",
    "content-signal-1966",
    "sid-signal-1967",
    "rev-signal-1968",
    "classtype-signal-1969",
    "metadata-signal-1970",
    "reference-signal-1971",
    "flow-signal-1972",
    "established-signal-1973",
    "to_server-signal-1974",
    "to_client-signal-1975",
    "domain-signal-1976",
    "url-signal-1977",
    "hash-signal-1978",
    "sha256-signal-1979",
    "sha1-signal-1980",
    "md5-signal-1981",
    "cidr-signal-1982",
    "input-signal-1983",
    "output-signal-1984",
    "forward-signal-1985",
    "accept-signal-1986",
    "deny-signal-1987",
    "dns-signal-1988",
    "http-signal-1989",
    "tls-signal-1990",
    "ssh-signal-1991",
    "rdp-signal-1992",
    "smb-signal-1993",
    "ldap-signal-1994",
    "kerberos-signal-1995",
    "syslog-signal-1996",
    "icmp-signal-1997",
    "tcp-signal-1998",
    "udp-signal-1999",
    "scan-signal-2000",
    "beacon-signal-2001",
    "malware-signal-2002",
    "phishing-signal-2003",
    "ransomware-signal-2004",
    "botnet-signal-2005",
    "command-control-signal-2006",
    "exfiltration-signal-2007",
    "lateral-movement-signal-2008",
    "alert-signal-2009",
    "allow-signal-2010",
    "block-signal-2011",
    "drop-signal-2012",
    "reject-signal-2013",
    "pass-signal-2014",
    "content-signal-2015",
    "sid-signal-2016",
    "rev-signal-2017",
    "classtype-signal-2018",
    "metadata-signal-2019",
    "reference-signal-2020",
    "flow-signal-2021",
    "established-signal-2022",
    "to_server-signal-2023",
    "to_client-signal-2024",
    "domain-signal-2025",
    "url-signal-2026",
    "hash-signal-2027",
    "sha256-signal-2028",
    "sha1-signal-2029",
    "md5-signal-2030",
    "cidr-signal-2031",
    "input-signal-2032",
    "output-signal-2033",
    "forward-signal-2034",
    "accept-signal-2035",
    "deny-signal-2036",
    "dns-signal-2037",
    "http-signal-2038",
    "tls-signal-2039",
    "ssh-signal-2040",
    "rdp-signal-2041",
    "smb-signal-2042",
    "ldap-signal-2043",
    "kerberos-signal-2044",
    "syslog-signal-2045",
    "icmp-signal-2046",
    "tcp-signal-2047",
    "udp-signal-2048",
    "scan-signal-2049",
    "beacon-signal-2050",
    "malware-signal-2051",
    "phishing-signal-2052",
    "ransomware-signal-2053",
    "botnet-signal-2054",
    "command-control-signal-2055",
    "exfiltration-signal-2056",
    "lateral-movement-signal-2057",
    "alert-signal-2058",
    "allow-signal-2059",
    "block-signal-2060",
    "drop-signal-2061",
    "reject-signal-2062",
    "pass-signal-2063",
    "content-signal-2064",
    "sid-signal-2065",
    "rev-signal-2066",
    "classtype-signal-2067",
    "metadata-signal-2068",
    "reference-signal-2069",
    "flow-signal-2070",
    "established-signal-2071",
    "to_server-signal-2072",
    "to_client-signal-2073",
    "domain-signal-2074",
    "url-signal-2075",
    "hash-signal-2076",
    "sha256-signal-2077",
    "sha1-signal-2078",
    "md5-signal-2079",
    "cidr-signal-2080",
    "input-signal-2081",
    "output-signal-2082",
    "forward-signal-2083",
    "accept-signal-2084",
    "deny-signal-2085",
    "dns-signal-2086",
    "http-signal-2087",
    "tls-signal-2088",
    "ssh-signal-2089",
    "rdp-signal-2090",
    "smb-signal-2091",
    "ldap-signal-2092",
    "kerberos-signal-2093",
    "syslog-signal-2094",
    "icmp-signal-2095",
    "tcp-signal-2096",
    "udp-signal-2097",
    "scan-signal-2098",
    "beacon-signal-2099",
    "malware-signal-2100",
    "phishing-signal-2101",
    "ransomware-signal-2102",
    "botnet-signal-2103",
    "command-control-signal-2104",
    "exfiltration-signal-2105",
    "lateral-movement-signal-2106",
    "alert-signal-2107",
    "allow-signal-2108",
    "block-signal-2109",
    "drop-signal-2110",
    "reject-signal-2111",
    "pass-signal-2112",
    "content-signal-2113",
    "sid-signal-2114",
    "rev-signal-2115",
    "classtype-signal-2116",
    "metadata-signal-2117",
    "reference-signal-2118",
    "flow-signal-2119",
    "established-signal-2120",
    "to_server-signal-2121",
    "to_client-signal-2122",
    "domain-signal-2123",
    "url-signal-2124",
    "hash-signal-2125",
    "sha256-signal-2126",
    "sha1-signal-2127",
    "md5-signal-2128",
    "cidr-signal-2129",
    "input-signal-2130",
    "output-signal-2131",
    "forward-signal-2132",
    "accept-signal-2133",
    "deny-signal-2134",
    "dns-signal-2135",
    "http-signal-2136",
    "tls-signal-2137",
    "ssh-signal-2138",
    "rdp-signal-2139",
    "smb-signal-2140",
    "ldap-signal-2141",
    "kerberos-signal-2142",
    "syslog-signal-2143",
    "icmp-signal-2144",
    "tcp-signal-2145",
    "udp-signal-2146",
    "scan-signal-2147",
    "beacon-signal-2148",
    "malware-signal-2149",
    "phishing-signal-2150",
    "ransomware-signal-2151",
    "botnet-signal-2152",
    "command-control-signal-2153",
    "exfiltration-signal-2154",
    "lateral-movement-signal-2155",
    "alert-signal-2156",
    "allow-signal-2157",
    "block-signal-2158",
    "drop-signal-2159",
    "reject-signal-2160",
    "pass-signal-2161",
    "content-signal-2162",
    "sid-signal-2163",
    "rev-signal-2164",
    "classtype-signal-2165",
    "metadata-signal-2166",
    "reference-signal-2167",
    "flow-signal-2168",
    "established-signal-2169",
    "to_server-signal-2170",
    "to_client-signal-2171",
    "domain-signal-2172",
    "url-signal-2173",
    "hash-signal-2174",
    "sha256-signal-2175",
    "sha1-signal-2176",
    "md5-signal-2177",
    "cidr-signal-2178",
    "input-signal-2179",
    "output-signal-2180",
    "forward-signal-2181",
    "accept-signal-2182",
    "deny-signal-2183",
    "dns-signal-2184",
    "http-signal-2185",
    "tls-signal-2186",
    "ssh-signal-2187",
    "rdp-signal-2188",
    "smb-signal-2189",
    "ldap-signal-2190",
    "kerberos-signal-2191",
    "syslog-signal-2192",
    "icmp-signal-2193",
    "tcp-signal-2194",
    "udp-signal-2195",
    "scan-signal-2196",
    "beacon-signal-2197",
    "malware-signal-2198",
    "phishing-signal-2199",
];

pub fn token_known(token: &str) -> bool {
    SECURITY_TOKENS.iter().any(|candidate| candidate.eq_ignore_ascii_case(token))
}
