/// Represents a static translation table for decoding Huffman sequence by
/// reading 2-bits at at time.
pub const DECODE_TABLE: [[(Option<u8>, Option<u16>, u8); 4]; 126] = [ // (next_id, ascii, leftover)
    [ // 0
        (Some(36), None, 0),
        (Some(29), None, 0),
        (Some(40), None, 0),
        (Some(1), None, 0)
    ],
    [ // 1
        (Some(47), None, 0),
        (Some(52), None, 0),
        (Some(57), None, 0),
        (Some(2), None, 0)
    ],
    [ // 2
        (Some(70), None, 0),
        (Some(71), None, 0),
        (Some(33), None, 0),
        (Some(3), None, 0)
    ],
    [ // 3
        (None, Some(88), 0),
        (None, Some(90), 0),
        (Some(31), None, 0),
        (Some(4), None, 0)
    ],
    [ // 4
        (None, Some(63), 0),
        (Some(34), None, 0),
        (Some(32), None, 0),
        (Some(5), None, 0)
    ],
    [ // 5
        (Some(6), None, 0),
        (Some(45), None, 0),
        (Some(62), None, 0),
        (Some(7), None, 0)
    ],
    [ // 6
        (None, Some(0), 1),
        (None, Some(0), 1),
        (None, Some(36), 1),
        (None, Some(36), 1)
    ],
    [ // 7
        (None, Some(94), 0),
        (None, Some(125), 0),
        (Some(43), None, 0),
        (Some(8), None, 0)
    ],
    [ // 8
        (None, Some(123), 1),
        (None, Some(123), 1),
        (Some(60), None, 0),
        (Some(9), None, 0)
    ],
    [ // 9
        (Some(96), None, 0),
        (Some(73), None, 0),
        (Some(10), None, 0),
        (Some(13), None, 0)
    ],
    [ // 10
        (Some(98), None, 0),
        (Some(101), None, 0),
        (Some(106), None, 0),
        (Some(11), None, 0)
    ],
    [ // 11
        (Some(12), None, 0),
        (Some(77), None, 0),
        (Some(78), None, 0),
        (Some(79), None, 0)
    ],
    [ // 12
        (None, Some(1), 1),
        (None, Some(1), 1),
        (None, Some(135), 1),
        (None, Some(135), 1)
    ],
    [ // 13
        (Some(81), None, 0),
        (Some(90), None, 0),
        (Some(20), None, 0),
        (Some(14), None, 0)
    ],
    [ // 14
        (Some(94), None, 0),
        (Some(107), None, 0),
        (Some(103), None, 0),
        (Some(15), None, 0)
    ],
    [ // 15
        (Some(112), None, 0),
        (Some(121), None, 0),
        (Some(16), None, 0),
        (Some(22), None, 0)
    ],
    [ // 16
        (Some(17), None, 0),
        (Some(18), None, 0),
        (Some(19), None, 0),
        (Some(25), None, 0)
    ],
    [ // 17
        (None, Some(254), 1),
        (None, Some(254), 1),
        (None, Some(2), 0),
        (None, Some(3), 0)
    ],
    [ // 18
        (None, Some(4), 0),
        (None, Some(5), 0),
        (None, Some(6), 0),
        (None, Some(7), 0)
    ],
    [ // 19
        (None, Some(8), 0),
        (None, Some(11), 0),
        (None, Some(12), 0),
        (None, Some(14), 0)
    ],
    [ // 20
        (Some(102), None, 0),
        (Some(105), None, 0),
        (Some(21), None, 0),
        (Some(80), None, 0)
    ],
    [ // 21
        (None, Some(239), 1),
        (None, Some(239), 1),
        (None, Some(9), 0),
        (None, Some(142), 0)
    ],
    [ // 22
        (Some(26), None, 0),
        (Some(27), None, 0),
        (Some(28), None, 0),
        (Some(23), None, 0)
    ],
    [ // 23
        (None, Some(127), 0),
        (None, Some(220), 0),
        (None, Some(249), 0),
        (Some(24), None, 0)
    ],
    [ // 24
        (None, Some(10), 0),
        (None, Some(13), 0),
        (None, Some(22), 0),
        (None, Some(256), 0)
    ],
    [ // 25
        (None, Some(15), 0),
        (None, Some(16), 0),
        (None, Some(17), 0),
        (None, Some(18), 0)
    ],
    [ // 26
        (None, Some(19), 0),
        (None, Some(20), 0),
        (None, Some(21), 0),
        (None, Some(23), 0)
    ],
    [ // 27
        (None, Some(24), 0),
        (None, Some(25), 0),
        (None, Some(26), 0),
        (None, Some(27), 0)
    ],
    [ // 28
        (None, Some(28), 0),
        (None, Some(29), 0),
        (None, Some(30), 0),
        (None, Some(31), 0)
    ],
    [ // 29
        (Some(69), None, 0),
        (Some(30), None, 0),
        (Some(35), None, 0),
        (Some(39), None, 0)
    ],
    [ // 30
        (None, Some(32), 0),
        (None, Some(37), 0),
        (None, Some(45), 0),
        (None, Some(46), 0)
    ],
    [ // 31
        (None, Some(33), 0),
        (None, Some(34), 0),
        (None, Some(40), 0),
        (None, Some(41), 0)
    ],
    [ // 32
        (None, Some(124), 1),
        (None, Some(124), 1),
        (None, Some(35), 0),
        (None, Some(62), 0)
    ],
    [ // 33
        (None, Some(38), 0),
        (None, Some(42), 0),
        (None, Some(44), 0),
        (None, Some(59), 0)
    ],
    [ // 34
        (None, Some(39), 1),
        (None, Some(39), 1),
        (None, Some(43), 1),
        (None, Some(43), 1)
    ],
    [ // 35
        (None, Some(47), 0),
        (None, Some(51), 0),
        (None, Some(52), 0),
        (None, Some(53), 0)
    ],
    [ // 36
        (Some(37), None, 0),
        (Some(38), None, 0),
        (Some(63), None, 0),
        (Some(65), None, 0)
    ],
    [ // 37
        (None, Some(48), 1),
        (None, Some(48), 1),
        (None, Some(49), 1),
        (None, Some(49), 1)
    ],
    [ // 38
        (None, Some(50), 1),
        (None, Some(50), 1),
        (None, Some(97), 1),
        (None, Some(97), 1)
    ],
    [ // 39
        (None, Some(54), 0),
        (None, Some(55), 0),
        (None, Some(56), 0),
        (None, Some(57), 0)
    ],
    [ // 40
        (Some(44), None, 0),
        (Some(64), None, 0),
        (Some(67), None, 0),
        (Some(41), None, 0)
    ],
    [ // 41
        (None, Some(114), 0),
        (None, Some(117), 0),
        (Some(42), None, 0),
        (Some(46), None, 0)
    ],
    [ // 42
        (None, Some(58), 1),
        (None, Some(58), 1),
        (None, Some(66), 1),
        (None, Some(66), 1)
    ],
    [ // 43
        (None, Some(60), 1),
        (None, Some(60), 1),
        (None, Some(96), 1),
        (None, Some(96), 1)
    ],
    [ // 44
        (None, Some(61), 0),
        (None, Some(65), 0),
        (None, Some(95), 0),
        (None, Some(98), 0)
    ],
    [ // 45
        (None, Some(64), 1),
        (None, Some(64), 1),
        (None, Some(91), 1),
        (None, Some(91), 1)
    ],
    [ // 46
        (None, Some(67), 1),
        (None, Some(67), 1),
        (None, Some(68), 1),
        (None, Some(68), 1)
    ],
    [ // 47
        (Some(48), None, 0),
        (Some(49), None, 0),
        (Some(50), None, 0),
        (Some(51), None, 0)
    ],
    [ // 48
        (None, Some(69), 1),
        (None, Some(69), 1),
        (None, Some(70), 1),
        (None, Some(70), 1)
    ],
    [ // 49
        (None, Some(71), 1),
        (None, Some(71), 1),
        (None, Some(72), 1),
        (None, Some(72), 1)
    ],
    [ // 50
        (None, Some(73), 1),
        (None, Some(73), 1),
        (None, Some(74), 1),
        (None, Some(74), 1)
    ],
    [ // 51
        (None, Some(75), 1),
        (None, Some(75), 1),
        (None, Some(76), 1),
        (None, Some(76), 1)
    ],
    [ // 52
        (Some(53), None, 0),
        (Some(54), None, 0),
        (Some(55), None, 0),
        (Some(56), None, 0)
    ],
    [ // 53
        (None, Some(77), 1),
        (None, Some(77), 1),
        (None, Some(78), 1),
        (None, Some(78), 1)
    ],
    [ // 54
        (None, Some(79), 1),
        (None, Some(79), 1),
        (None, Some(80), 1),
        (None, Some(80), 1)
    ],
    [ // 55
        (None, Some(81), 1),
        (None, Some(81), 1),
        (None, Some(82), 1),
        (None, Some(82), 1)
    ],
    [ // 56
        (None, Some(83), 1),
        (None, Some(83), 1),
        (None, Some(84), 1),
        (None, Some(84), 1)
    ],
    [ // 57
        (Some(58), None, 0),
        (Some(59), None, 0),
        (Some(66), None, 0),
        (Some(68), None, 0)
    ],
    [ // 58
        (None, Some(85), 1),
        (None, Some(85), 1),
        (None, Some(86), 1),
        (None, Some(86), 1)
    ],
    [ // 59
        (None, Some(87), 1),
        (None, Some(87), 1),
        (None, Some(89), 1),
        (None, Some(89), 1)
    ],
    [ // 60
        (Some(61), None, 0),
        (Some(72), None, 0),
        (Some(75), None, 0),
        (Some(85), None, 0)
    ],
    [ // 61
        (None, Some(92), 1),
        (None, Some(92), 1),
        (None, Some(195), 1),
        (None, Some(195), 1)
    ],
    [ // 62
        (None, Some(93), 1),
        (None, Some(93), 1),
        (None, Some(126), 1),
        (None, Some(126), 1)
    ],
    [ // 63
        (None, Some(99), 1),
        (None, Some(99), 1),
        (None, Some(101), 1),
        (None, Some(101), 1)
    ],
    [ // 64
        (None, Some(100), 0),
        (None, Some(102), 0),
        (None, Some(103), 0),
        (None, Some(104), 0)
    ],
    [ // 65
        (None, Some(105), 1),
        (None, Some(105), 1),
        (None, Some(111), 1),
        (None, Some(111), 1)
    ],
    [ // 66
        (None, Some(106), 1),
        (None, Some(106), 1),
        (None, Some(107), 1),
        (None, Some(107), 1)
    ],
    [ // 67
        (None, Some(108), 0),
        (None, Some(109), 0),
        (None, Some(110), 0),
        (None, Some(112), 0)
    ],
    [ // 68
        (None, Some(113), 1),
        (None, Some(113), 1),
        (None, Some(118), 1),
        (None, Some(118), 1)
    ],
    [ // 69
        (None, Some(115), 1),
        (None, Some(115), 1),
        (None, Some(116), 1),
        (None, Some(116), 1)
    ],
    [ // 70
        (None, Some(119), 1),
        (None, Some(119), 1),
        (None, Some(120), 1),
        (None, Some(120), 1)
    ],
    [ // 71
        (None, Some(121), 1),
        (None, Some(121), 1),
        (None, Some(122), 1),
        (None, Some(122), 1)
    ],
    [ // 72
        (None, Some(208), 1),
        (None, Some(208), 1),
        (None, Some(128), 0),
        (None, Some(130), 0)
    ],
    [ // 73
        (Some(74), None, 0),
        (Some(76), None, 0),
        (Some(87), None, 0),
        (Some(89), None, 0)
    ],
    [ // 74
        (None, Some(230), 1),
        (None, Some(230), 1),
        (None, Some(129), 0),
        (None, Some(132), 0)
    ],
    [ // 75
        (None, Some(131), 0),
        (None, Some(162), 0),
        (None, Some(184), 0),
        (None, Some(194), 0)
    ],
    [ // 76
        (None, Some(133), 0),
        (None, Some(134), 0),
        (None, Some(136), 0),
        (None, Some(146), 0)
    ],
    [ // 77
        (None, Some(137), 1),
        (None, Some(137), 1),
        (None, Some(138), 1),
        (None, Some(138), 1)
    ],
    [ // 78
        (None, Some(139), 1),
        (None, Some(139), 1),
        (None, Some(140), 1),
        (None, Some(140), 1)
    ],
    [ // 79
        (None, Some(141), 1),
        (None, Some(141), 1),
        (None, Some(143), 1),
        (None, Some(143), 1)
    ],
    [ // 80
        (None, Some(144), 0),
        (None, Some(145), 0),
        (None, Some(148), 0),
        (None, Some(159), 0)
    ],
    [ // 81
        (Some(82), None, 0),
        (Some(83), None, 0),
        (Some(84), None, 0),
        (Some(88), None, 0)
    ],
    [ // 82
        (None, Some(147), 1),
        (None, Some(147), 1),
        (None, Some(149), 1),
        (None, Some(149), 1)
    ],
    [ // 83
        (None, Some(150), 1),
        (None, Some(150), 1),
        (None, Some(151), 1),
        (None, Some(151), 1)
    ],
    [ // 84
        (None, Some(152), 1),
        (None, Some(152), 1),
        (None, Some(155), 1),
        (None, Some(155), 1)
    ],
    [ // 85
        (None, Some(224), 0),
        (None, Some(226), 0),
        (Some(86), None, 0),
        (Some(92), None, 0)
    ],
    [ // 86
        (None, Some(153), 1),
        (None, Some(153), 1),
        (None, Some(161), 1),
        (None, Some(161), 1)
    ],
    [ // 87
        (None, Some(154), 0),
        (None, Some(156), 0),
        (None, Some(160), 0),
        (None, Some(163), 0)
    ],
    [ // 88
        (None, Some(157), 1),
        (None, Some(157), 1),
        (None, Some(158), 1),
        (None, Some(158), 1)
    ],
    [ // 89
        (None, Some(164), 0),
        (None, Some(169), 0),
        (None, Some(170), 0),
        (None, Some(173), 0)
    ],
    [ // 90
        (Some(91), None, 0),
        (Some(93), None, 0),
        (Some(95), None, 0),
        (Some(100), None, 0)
    ],
    [ // 91
        (None, Some(165), 1),
        (None, Some(165), 1),
        (None, Some(166), 1),
        (None, Some(166), 1)
    ],
    [ // 92
        (None, Some(167), 1),
        (None, Some(167), 1),
        (None, Some(172), 1),
        (None, Some(172), 1)
    ],
    [ // 93
        (None, Some(168), 1),
        (None, Some(168), 1),
        (None, Some(174), 1),
        (None, Some(174), 1)
    ],
    [ // 94
        (None, Some(171), 0),
        (None, Some(206), 0),
        (None, Some(215), 0),
        (None, Some(225), 0)
    ],
    [ // 95
        (None, Some(175), 1),
        (None, Some(175), 1),
        (None, Some(180), 1),
        (None, Some(180), 1)
    ],
    [ // 96
        (Some(97), None, 0),
        (Some(99), None, 0),
        (Some(115), None, 0),
        (Some(118), None, 0)
    ],
    [ // 97
        (None, Some(176), 1),
        (None, Some(176), 1),
        (None, Some(177), 1),
        (None, Some(177), 1)
    ],
    [ // 98
        (None, Some(178), 0),
        (None, Some(181), 0),
        (None, Some(185), 0),
        (None, Some(186), 0)
    ],
    [ // 99
        (None, Some(179), 1),
        (None, Some(179), 1),
        (None, Some(209), 1),
        (None, Some(209), 1)
    ],
    [ // 100
        (None, Some(182), 1),
        (None, Some(182), 1),
        (None, Some(183), 1),
        (None, Some(183), 1)
    ],
    [ // 101
        (None, Some(187), 0),
        (None, Some(189), 0),
        (None, Some(190), 0),
        (None, Some(196), 0)
    ],
    [ // 102
        (None, Some(188), 1),
        (None, Some(188), 1),
        (None, Some(191), 1),
        (None, Some(191), 1)
    ],
    [ // 103
        (Some(104), None, 0),
        (Some(109), None, 0),
        (Some(116), None, 0),
        (Some(110), None, 0)
    ],
    [ // 104
        (None, Some(192), 0),
        (None, Some(193), 0),
        (None, Some(200), 0),
        (None, Some(201), 0)
    ],
    [ // 105
        (None, Some(197), 1),
        (None, Some(197), 1),
        (None, Some(231), 1),
        (None, Some(231), 1)
    ],
    [ // 106
        (None, Some(198), 0),
        (None, Some(228), 0),
        (None, Some(232), 0),
        (None, Some(233), 0)
    ],
    [ // 107
        (None, Some(236), 0),
        (None, Some(237), 0),
        (Some(108), None, 0),
        (Some(119), None, 0)
    ],
    [ // 108
        (None, Some(199), 1),
        (None, Some(199), 1),
        (None, Some(207), 1),
        (None, Some(207), 1)
    ],
    [ // 109
        (None, Some(202), 0),
        (None, Some(205), 0),
        (None, Some(210), 0),
        (None, Some(213), 0)
    ],
    [ // 110
        (None, Some(242), 0),
        (None, Some(243), 0),
        (None, Some(255), 0),
        (Some(111), None, 0)
    ],
    [ // 111
        (None, Some(203), 1),
        (None, Some(203), 1),
        (None, Some(204), 1),
        (None, Some(204), 1)
    ],
    [ // 112
        (Some(113), None, 0),
        (Some(114), None, 0),
        (Some(117), None, 0),
        (Some(120), None, 0)
    ],
    [ // 113
        (None, Some(211), 1),
        (None, Some(211), 1),
        (None, Some(212), 1),
        (None, Some(212), 1)
    ],
    [ // 114
        (None, Some(214), 1),
        (None, Some(214), 1),
        (None, Some(221), 1),
        (None, Some(221), 1)
    ],
    [ // 115
        (None, Some(216), 1),
        (None, Some(216), 1),
        (None, Some(217), 1),
        (None, Some(217), 1)
    ],
    [ // 116
        (None, Some(218), 0),
        (None, Some(219), 0),
        (None, Some(238), 0),
        (None, Some(240), 0)
    ],
    [ // 117
        (None, Some(222), 1),
        (None, Some(222), 1),
        (None, Some(223), 1),
        (None, Some(223), 1)
    ],
    [ // 118
        (None, Some(227), 1),
        (None, Some(227), 1),
        (None, Some(229), 1),
        (None, Some(229), 1)
    ],
    [ // 119
        (None, Some(234), 1),
        (None, Some(234), 1),
        (None, Some(235), 1),
        (None, Some(235), 1)
    ],
    [ // 120
        (None, Some(241), 1),
        (None, Some(241), 1),
        (None, Some(244), 1),
        (None, Some(244), 1)
    ],
    [ // 121
        (Some(122), None, 0),
        (Some(123), None, 0),
        (Some(124), None, 0),
        (Some(125), None, 0)
    ],
    [ // 122
        (None, Some(245), 1),
        (None, Some(245), 1),
        (None, Some(246), 1),
        (None, Some(246), 1)
    ],
    [ // 123
        (None, Some(247), 1),
        (None, Some(247), 1),
        (None, Some(248), 1),
        (None, Some(248), 1)
    ],
    [ // 124
        (None, Some(250), 1),
        (None, Some(250), 1),
        (None, Some(251), 1),
        (None, Some(251), 1)
    ],
    [ // 125
        (None, Some(252), 1),
        (None, Some(252), 1),
        (None, Some(253), 1),
        (None, Some(253), 1)
    ]
];
