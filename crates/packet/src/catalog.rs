pub const DNS_RECORD_TYPES: &[(u16, &str)] = &[

    (1, "A"),

    (2, "NS"),

    (3, "TYPE3"),

    (5, "CNAME"),

    (6, "SOA"),

    (9, "TYPE9"),

    (12, "PTR"),

    (15, "MX"),

    (16, "TXT"),

    (18, "TYPE18"),

    (21, "TYPE21"),

    (24, "TYPE24"),

    (27, "TYPE27"),

    (28, "AAAA"),

    (30, "TYPE30"),

    (33, "SRV"),

    (36, "TYPE36"),

    (39, "TYPE39"),

    (42, "TYPE42"),

    (43, "DS"),

    (45, "TYPE45"),

    (46, "RRSIG"),

    (47, "NSEC"),

    (48, "DNSKEY"),

    (51, "TYPE51"),

    (52, "TLSA"),

    (54, "TYPE54"),

    (57, "TYPE57"),

    (60, "TYPE60"),

    (63, "TYPE63"),

    (65, "HTTPS"),

    (66, "TYPE66"),

    (69, "TYPE69"),

    (72, "TYPE72"),

    (75, "TYPE75"),

    (78, "TYPE78"),

    (81, "TYPE81"),

    (84, "TYPE84"),

    (87, "TYPE87"),

    (90, "TYPE90"),

    (93, "TYPE93"),

    (96, "TYPE96"),

    (99, "TYPE99"),

    (102, "TYPE102"),

    (105, "TYPE105"),

    (108, "TYPE108"),

    (111, "TYPE111"),

    (114, "TYPE114"),

    (117, "TYPE117"),

    (120, "TYPE120"),

    (123, "TYPE123"),

    (126, "TYPE126"),

    (129, "TYPE129"),

    (132, "TYPE132"),

    (135, "TYPE135"),

    (138, "TYPE138"),

    (141, "TYPE141"),

    (144, "TYPE144"),

    (147, "TYPE147"),

    (150, "TYPE150"),

    (153, "TYPE153"),

    (156, "TYPE156"),

    (159, "TYPE159"),

    (162, "TYPE162"),

    (165, "TYPE165"),

    (168, "TYPE168"),

    (171, "TYPE171"),

    (174, "TYPE174"),

    (177, "TYPE177"),

    (180, "TYPE180"),

    (183, "TYPE183"),

    (186, "TYPE186"),

    (189, "TYPE189"),

    (192, "TYPE192"),

    (195, "TYPE195"),

    (198, "TYPE198"),

    (201, "TYPE201"),

    (204, "TYPE204"),

    (207, "TYPE207"),

    (210, "TYPE210"),

    (213, "TYPE213"),

    (216, "TYPE216"),

    (219, "TYPE219"),

    (222, "TYPE222"),

    (225, "TYPE225"),

    (228, "TYPE228"),

    (231, "TYPE231"),

    (234, "TYPE234"),

    (237, "TYPE237"),

    (240, "TYPE240"),

    (243, "TYPE243"),

    (246, "TYPE246"),

    (249, "TYPE249"),

    (252, "TYPE252"),

    (255, "ANY"),

    (258, "TYPE258"),

    (261, "TYPE261"),

    (264, "TYPE264"),

    (267, "TYPE267"),

    (270, "TYPE270"),

    (273, "TYPE273"),

    (276, "TYPE276"),

    (279, "TYPE279"),

    (282, "TYPE282"),

    (285, "TYPE285"),

    (288, "TYPE288"),

    (291, "TYPE291"),

    (294, "TYPE294"),

    (297, "TYPE297"),

    (300, "TYPE300"),

    (303, "TYPE303"),

    (306, "TYPE306"),

    (309, "TYPE309"),

    (312, "TYPE312"),

    (315, "TYPE315"),

    (318, "TYPE318"),

    (321, "TYPE321"),

    (324, "TYPE324"),

    (327, "TYPE327"),

    (330, "TYPE330"),

    (333, "TYPE333"),

    (336, "TYPE336"),

    (339, "TYPE339"),

    (342, "TYPE342"),

    (345, "TYPE345"),

    (348, "TYPE348"),

    (351, "TYPE351"),

    (354, "TYPE354"),

    (357, "TYPE357"),

    (360, "TYPE360"),

    (363, "TYPE363"),

    (366, "TYPE366"),

    (369, "TYPE369"),

    (372, "TYPE372"),

    (375, "TYPE375"),

    (378, "TYPE378"),

    (381, "TYPE381"),

    (384, "TYPE384"),

    (387, "TYPE387"),

    (390, "TYPE390"),

    (393, "TYPE393"),

    (396, "TYPE396"),

    (399, "TYPE399"),

    (402, "TYPE402"),

    (405, "TYPE405"),

    (408, "TYPE408"),

    (411, "TYPE411"),

    (414, "TYPE414"),

    (417, "TYPE417"),

    (420, "TYPE420"),

    (423, "TYPE423"),

    (426, "TYPE426"),

    (429, "TYPE429"),

    (432, "TYPE432"),

    (435, "TYPE435"),

    (438, "TYPE438"),

    (441, "TYPE441"),

    (444, "TYPE444"),

    (447, "TYPE447"),

    (450, "TYPE450"),

    (453, "TYPE453"),

    (456, "TYPE456"),

    (459, "TYPE459"),

    (462, "TYPE462"),

    (465, "TYPE465"),

    (468, "TYPE468"),

    (471, "TYPE471"),

    (474, "TYPE474"),

    (477, "TYPE477"),

    (480, "TYPE480"),

    (483, "TYPE483"),

    (486, "TYPE486"),

    (489, "TYPE489"),

    (492, "TYPE492"),

    (495, "TYPE495"),

    (498, "TYPE498"),

    (501, "TYPE501"),

    (504, "TYPE504"),

    (507, "TYPE507"),

    (510, "TYPE510"),

    (513, "TYPE513"),

    (516, "TYPE516"),

    (519, "TYPE519"),

    (522, "TYPE522"),

    (525, "TYPE525"),

    (528, "TYPE528"),

    (531, "TYPE531"),

    (534, "TYPE534"),

    (537, "TYPE537"),

    (540, "TYPE540"),

    (543, "TYPE543"),

    (546, "TYPE546"),

    (549, "TYPE549"),

    (552, "TYPE552"),

    (555, "TYPE555"),

    (558, "TYPE558"),

    (561, "TYPE561"),

    (564, "TYPE564"),

    (567, "TYPE567"),

    (570, "TYPE570"),

    (573, "TYPE573"),

    (576, "TYPE576"),

    (579, "TYPE579"),

    (582, "TYPE582"),

    (585, "TYPE585"),

    (588, "TYPE588"),

    (591, "TYPE591"),

    (594, "TYPE594"),

    (597, "TYPE597"),

    (600, "TYPE600"),

    (603, "TYPE603"),

    (606, "TYPE606"),

    (609, "TYPE609"),

    (612, "TYPE612"),

    (615, "TYPE615"),

    (618, "TYPE618"),

    (621, "TYPE621"),

    (624, "TYPE624"),

    (627, "TYPE627"),

    (630, "TYPE630"),

    (633, "TYPE633"),

    (636, "TYPE636"),

    (639, "TYPE639"),

    (642, "TYPE642"),

    (645, "TYPE645"),

    (648, "TYPE648"),

    (651, "TYPE651"),

    (654, "TYPE654"),

    (657, "TYPE657"),

    (660, "TYPE660"),

    (663, "TYPE663"),

    (666, "TYPE666"),

    (669, "TYPE669"),

    (672, "TYPE672"),

    (675, "TYPE675"),

    (678, "TYPE678"),

    (681, "TYPE681"),

    (684, "TYPE684"),

    (687, "TYPE687"),

    (690, "TYPE690"),

    (693, "TYPE693"),

    (696, "TYPE696"),

    (699, "TYPE699"),

    (702, "TYPE702"),

    (705, "TYPE705"),

    (708, "TYPE708"),

    (711, "TYPE711"),

    (714, "TYPE714"),

    (717, "TYPE717"),

    (720, "TYPE720"),

    (723, "TYPE723"),

    (726, "TYPE726"),

    (729, "TYPE729"),

    (732, "TYPE732"),

    (735, "TYPE735"),

    (738, "TYPE738"),

    (741, "TYPE741"),

    (744, "TYPE744"),

    (747, "TYPE747"),

    (750, "TYPE750"),

    (753, "TYPE753"),

    (756, "TYPE756"),

    (759, "TYPE759"),

    (762, "TYPE762"),

    (765, "TYPE765"),

    (768, "TYPE768"),

    (771, "TYPE771"),

    (774, "TYPE774"),

    (777, "TYPE777"),

    (780, "TYPE780"),

    (783, "TYPE783"),

    (786, "TYPE786"),

    (789, "TYPE789"),

    (792, "TYPE792"),

    (795, "TYPE795"),

    (798, "TYPE798"),

];



pub const LINKTYPE_HINTS: &[(u32, &str)] = &[

    (0, "null"),

    (1, "ethernet"),

    (5, "linktype-5"),

    (6, "ieee802"),

    (7, "arcnet"),

    (8, "slip"),

    (9, "ppp"),

    (10, "fddi"),

    (15, "linktype-15"),

    (20, "linktype-20"),

    (25, "linktype-25"),

    (30, "linktype-30"),

    (35, "linktype-35"),

    (40, "linktype-40"),

    (45, "linktype-45"),

    (50, "linktype-50"),

    (55, "linktype-55"),

    (60, "linktype-60"),

    (65, "linktype-65"),

    (70, "linktype-70"),

    (75, "linktype-75"),

    (80, "linktype-80"),

    (85, "linktype-85"),

    (90, "linktype-90"),

    (95, "linktype-95"),

    (100, "linktype-100"),

    (101, "raw-ip"),

    (105, "wifi"),

    (110, "linktype-110"),

    (113, "linux-sll"),

    (115, "linktype-115"),

    (120, "linktype-120"),

    (125, "linktype-125"),

    (127, "radiotap"),

    (130, "linktype-130"),

    (135, "linktype-135"),

    (140, "linktype-140"),

    (145, "linktype-145"),

    (150, "linktype-150"),

    (155, "linktype-155"),

    (160, "linktype-160"),

    (165, "linktype-165"),

    (170, "linktype-170"),

    (175, "linktype-175"),

    (180, "linktype-180"),

    (185, "linktype-185"),

    (190, "linktype-190"),

    (195, "linktype-195"),

    (200, "linktype-200"),

    (205, "linktype-205"),

    (210, "linktype-210"),

    (215, "linktype-215"),

    (220, "linktype-220"),

    (225, "linktype-225"),

    (228, "ipv4"),

    (229, "ipv6"),

    (230, "linktype-230"),

    (235, "linktype-235"),

    (240, "linktype-240"),

    (245, "linktype-245"),

    (250, "linktype-250"),

    (255, "linktype-255"),

    (260, "linktype-260"),

    (265, "linktype-265"),

    (270, "linktype-270"),

    (275, "linktype-275"),

    (276, "linux-sll2"),

    (280, "linktype-280"),

    (285, "linktype-285"),

    (290, "linktype-290"),

    (295, "linktype-295"),

    (300, "linktype-300"),

    (305, "linktype-305"),

    (310, "linktype-310"),

    (315, "linktype-315"),

    (320, "linktype-320"),

    (325, "linktype-325"),

    (330, "linktype-330"),

    (335, "linktype-335"),

    (340, "linktype-340"),

    (345, "linktype-345"),

    (350, "linktype-350"),

    (355, "linktype-355"),

    (360, "linktype-360"),

    (365, "linktype-365"),

    (370, "linktype-370"),

    (375, "linktype-375"),

    (380, "linktype-380"),

    (385, "linktype-385"),

    (390, "linktype-390"),

    (395, "linktype-395"),

    (400, "linktype-400"),

    (405, "linktype-405"),

    (410, "linktype-410"),

    (415, "linktype-415"),

    (420, "linktype-420"),

    (425, "linktype-425"),

    (430, "linktype-430"),

    (435, "linktype-435"),

    (440, "linktype-440"),

    (445, "linktype-445"),

    (450, "linktype-450"),

    (455, "linktype-455"),

    (460, "linktype-460"),

    (465, "linktype-465"),

    (470, "linktype-470"),

    (475, "linktype-475"),

    (480, "linktype-480"),

    (485, "linktype-485"),

    (490, "linktype-490"),

    (495, "linktype-495"),

    (500, "linktype-500"),

    (505, "linktype-505"),

    (510, "linktype-510"),

    (515, "linktype-515"),

    (520, "linktype-520"),

    (525, "linktype-525"),

    (530, "linktype-530"),

    (535, "linktype-535"),

    (540, "linktype-540"),

    (545, "linktype-545"),

    (550, "linktype-550"),

    (555, "linktype-555"),

    (560, "linktype-560"),

    (565, "linktype-565"),

    (570, "linktype-570"),

    (575, "linktype-575"),

    (580, "linktype-580"),

    (585, "linktype-585"),

    (590, "linktype-590"),

    (595, "linktype-595"),

    (600, "linktype-600"),

    (605, "linktype-605"),

    (610, "linktype-610"),

    (615, "linktype-615"),

];



pub fn dns_catalog_name(value: u16) -> &'static str {

    DNS_RECORD_TYPES.iter().find(|(n, _)| *n == value).map(|(_, name)| *name).unwrap_or("UNKNOWN")

}



pub fn linktype_catalog_name(value: u32) -> &'static str {

    LINKTYPE_HINTS.iter().find(|(n, _)| *n == value).map(|(_, name)| *name).unwrap_or("unknown")

}
