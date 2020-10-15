/// This is a static translation matrix for decoding Huffman sequence by reading
/// 3-bit(s) at at time.
pub const DECODE_TABLE: [[(Option<usize>, Option<usize>, usize); 8]; 92] = [ // (next_id, ascii, leftover)
    [ // 0
        (Some(34), None, 0),
        (Some(53), None, 0),
        (Some(28), None, 0),
        (Some(33), None, 0),
        (Some(37), None, 0),
        (Some(35), None, 0),
        (Some(40), None, 0),
        (Some(1), None, 0)
    ],
    [ // 1
        (Some(49), None, 0),
        (Some(50), None, 0),
        (Some(54), None, 0),
        (Some(55), None, 0),
        (Some(56), None, 0),
        (Some(57), None, 0),
        (Some(30), None, 0),
        (Some(2), None, 0)
    ],
    [ // 2
        (None, Some(88), 1),
        (None, Some(88), 1),
        (None, Some(90), 1),
        (None, Some(90), 1),
        (Some(29), None, 0),
        (Some(32), None, 0),
        (Some(31), None, 0),
        (Some(3), None, 0)
    ],
    [ // 3
        (None, Some(124), 1),
        (None, Some(124), 1),
        (None, Some(35), 0),
        (None, Some(62), 0),
        (Some(4), None, 0),
        (Some(38), None, 0),
        (Some(52), None, 0),
        (Some(5), None, 0)
    ],
    [ // 4
        (None, Some(0), 2),
        (None, Some(0), 2),
        (None, Some(0), 2),
        (None, Some(0), 2),
        (None, Some(36), 2),
        (None, Some(36), 2),
        (None, Some(36), 2),
        (None, Some(36), 2)
    ],
    [ // 5
        (None, Some(94), 1),
        (None, Some(94), 1),
        (None, Some(125), 1),
        (None, Some(125), 1),
        (None, Some(60), 0),
        (None, Some(96), 0),
        (None, Some(123), 0),
        (Some(6), None, 0)
    ],
    [ // 6
        (Some(51), None, 0),
        (Some(59), None, 0),
        (Some(62), None, 0),
        (Some(68), None, 0),
        (Some(76), None, 0),
        (Some(60), None, 0),
        (Some(7), None, 0),
        (Some(9), None, 0)
    ],
    [ // 7
        (Some(77), None, 0),
        (Some(78), None, 0),
        (Some(79), None, 0),
        (Some(81), None, 0),
        (Some(83), None, 0),
        (Some(89), None, 0),
        (Some(8), None, 0),
        (Some(65), None, 0)
    ],
    [ // 8
        (None, Some(1), 1),
        (None, Some(1), 1),
        (None, Some(135), 1),
        (None, Some(135), 1),
        (None, Some(137), 1),
        (None, Some(137), 1),
        (None, Some(138), 1),
        (None, Some(138), 1)
    ],
    [ // 9
        (Some(66), None, 0),
        (Some(67), None, 0),
        (Some(72), None, 0),
        (Some(75), None, 0),
        (Some(80), None, 0),
        (Some(16), None, 0),
        (Some(74), None, 0),
        (Some(10), None, 0)
    ],
    [ // 10
        (Some(82), None, 0),
        (Some(85), None, 0),
        (Some(88), None, 0),
        (Some(86), None, 0),
        (Some(87), None, 0),
        (Some(91), None, 0),
        (Some(11), None, 0),
        (Some(17), None, 0)
    ],
    [ // 11
        (None, Some(254), 0),
        (Some(12), None, 0),
        (Some(13), None, 0),
        (Some(14), None, 0),
        (Some(15), None, 0),
        (Some(19), None, 0),
        (Some(20), None, 0),
        (Some(21), None, 0)
    ],
    [ // 12
        (None, Some(2), 2),
        (None, Some(2), 2),
        (None, Some(2), 2),
        (None, Some(2), 2),
        (None, Some(3), 2),
        (None, Some(3), 2),
        (None, Some(3), 2),
        (None, Some(3), 2)
    ],
    [ // 13
        (None, Some(4), 2),
        (None, Some(4), 2),
        (None, Some(4), 2),
        (None, Some(4), 2),
        (None, Some(5), 2),
        (None, Some(5), 2),
        (None, Some(5), 2),
        (None, Some(5), 2)
    ],
    [ // 14
        (None, Some(6), 2),
        (None, Some(6), 2),
        (None, Some(6), 2),
        (None, Some(6), 2),
        (None, Some(7), 2),
        (None, Some(7), 2),
        (None, Some(7), 2),
        (None, Some(7), 2)
    ],
    [ // 15
        (None, Some(8), 2),
        (None, Some(8), 2),
        (None, Some(8), 2),
        (None, Some(8), 2),
        (None, Some(11), 2),
        (None, Some(11), 2),
        (None, Some(11), 2),
        (None, Some(11), 2)
    ],
    [ // 16
        (None, Some(239), 1),
        (None, Some(239), 1),
        (None, Some(9), 0),
        (None, Some(142), 0),
        (None, Some(144), 0),
        (None, Some(145), 0),
        (None, Some(148), 0),
        (None, Some(159), 0)
    ],
    [ // 17
        (Some(22), None, 0),
        (Some(23), None, 0),
        (Some(24), None, 0),
        (Some(25), None, 0),
        (Some(26), None, 0),
        (Some(27), None, 0),
        (Some(58), None, 0),
        (Some(18), None, 0)
    ],
    [ // 18
        (None, Some(249), 2),
        (None, Some(249), 2),
        (None, Some(249), 2),
        (None, Some(249), 2),
        (None, Some(10), 0),
        (None, Some(13), 0),
        (None, Some(22), 0),
        (None, Some(256), 0)
    ],
    [ // 19
        (None, Some(12), 2),
        (None, Some(12), 2),
        (None, Some(12), 2),
        (None, Some(12), 2),
        (None, Some(14), 2),
        (None, Some(14), 2),
        (None, Some(14), 2),
        (None, Some(14), 2)
    ],
    [ // 20
        (None, Some(15), 2),
        (None, Some(15), 2),
        (None, Some(15), 2),
        (None, Some(15), 2),
        (None, Some(16), 2),
        (None, Some(16), 2),
        (None, Some(16), 2),
        (None, Some(16), 2)
    ],
    [ // 21
        (None, Some(17), 2),
        (None, Some(17), 2),
        (None, Some(17), 2),
        (None, Some(17), 2),
        (None, Some(18), 2),
        (None, Some(18), 2),
        (None, Some(18), 2),
        (None, Some(18), 2)
    ],
    [ // 22
        (None, Some(19), 2),
        (None, Some(19), 2),
        (None, Some(19), 2),
        (None, Some(19), 2),
        (None, Some(20), 2),
        (None, Some(20), 2),
        (None, Some(20), 2),
        (None, Some(20), 2)
    ],
    [ // 23
        (None, Some(21), 2),
        (None, Some(21), 2),
        (None, Some(21), 2),
        (None, Some(21), 2),
        (None, Some(23), 2),
        (None, Some(23), 2),
        (None, Some(23), 2),
        (None, Some(23), 2)
    ],
    [ // 24
        (None, Some(24), 2),
        (None, Some(24), 2),
        (None, Some(24), 2),
        (None, Some(24), 2),
        (None, Some(25), 2),
        (None, Some(25), 2),
        (None, Some(25), 2),
        (None, Some(25), 2)
    ],
    [ // 25
        (None, Some(26), 2),
        (None, Some(26), 2),
        (None, Some(26), 2),
        (None, Some(26), 2),
        (None, Some(27), 2),
        (None, Some(27), 2),
        (None, Some(27), 2),
        (None, Some(27), 2)
    ],
    [ // 26
        (None, Some(28), 2),
        (None, Some(28), 2),
        (None, Some(28), 2),
        (None, Some(28), 2),
        (None, Some(29), 2),
        (None, Some(29), 2),
        (None, Some(29), 2),
        (None, Some(29), 2)
    ],
    [ // 27
        (None, Some(30), 2),
        (None, Some(30), 2),
        (None, Some(30), 2),
        (None, Some(30), 2),
        (None, Some(31), 2),
        (None, Some(31), 2),
        (None, Some(31), 2),
        (None, Some(31), 2)
    ],
    [ // 28
        (None, Some(115), 1),
        (None, Some(115), 1),
        (None, Some(116), 1),
        (None, Some(116), 1),
        (None, Some(32), 0),
        (None, Some(37), 0),
        (None, Some(45), 0),
        (None, Some(46), 0)
    ],
    [ // 29
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(34), 2),
        (None, Some(34), 2),
        (None, Some(34), 2),
        (None, Some(34), 2)
    ],
    [ // 30
        (None, Some(38), 1),
        (None, Some(38), 1),
        (None, Some(42), 1),
        (None, Some(42), 1),
        (None, Some(44), 1),
        (None, Some(44), 1),
        (None, Some(59), 1),
        (None, Some(59), 1)
    ],
    [ // 31
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(39), 1),
        (None, Some(39), 1),
        (None, Some(43), 1),
        (None, Some(43), 1)
    ],
    [ // 32
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(41), 2),
        (None, Some(41), 2),
        (None, Some(41), 2),
        (None, Some(41), 2)
    ],
    [ // 33
        (None, Some(47), 0),
        (None, Some(51), 0),
        (None, Some(52), 0),
        (None, Some(53), 0),
        (None, Some(54), 0),
        (None, Some(55), 0),
        (None, Some(56), 0),
        (None, Some(57), 0)
    ],
    [ // 34
        (None, Some(48), 1),
        (None, Some(48), 1),
        (None, Some(49), 1),
        (None, Some(49), 1),
        (None, Some(50), 1),
        (None, Some(50), 1),
        (None, Some(97), 1),
        (None, Some(97), 1)
    ],
    [ // 35
        (None, Some(108), 0),
        (None, Some(109), 0),
        (None, Some(110), 0),
        (None, Some(112), 0),
        (None, Some(114), 0),
        (None, Some(117), 0),
        (Some(36), None, 0),
        (Some(39), None, 0)
    ],
    [ // 36
        (None, Some(58), 2),
        (None, Some(58), 2),
        (None, Some(58), 2),
        (None, Some(58), 2),
        (None, Some(66), 2),
        (None, Some(66), 2),
        (None, Some(66), 2),
        (None, Some(66), 2)
    ],
    [ // 37
        (None, Some(61), 0),
        (None, Some(65), 0),
        (None, Some(95), 0),
        (None, Some(98), 0),
        (None, Some(100), 0),
        (None, Some(102), 0),
        (None, Some(103), 0),
        (None, Some(104), 0)
    ],
    [ // 38
        (None, Some(64), 2),
        (None, Some(64), 2),
        (None, Some(64), 2),
        (None, Some(64), 2),
        (None, Some(91), 2),
        (None, Some(91), 2),
        (None, Some(91), 2),
        (None, Some(91), 2)
    ],
    [ // 39
        (None, Some(67), 2),
        (None, Some(67), 2),
        (None, Some(67), 2),
        (None, Some(67), 2),
        (None, Some(68), 2),
        (None, Some(68), 2),
        (None, Some(68), 2),
        (None, Some(68), 2)
    ],
    [ // 40
        (Some(41), None, 0),
        (Some(42), None, 0),
        (Some(43), None, 0),
        (Some(44), None, 0),
        (Some(45), None, 0),
        (Some(46), None, 0),
        (Some(47), None, 0),
        (Some(48), None, 0)
    ],
    [ // 41
        (None, Some(69), 2),
        (None, Some(69), 2),
        (None, Some(69), 2),
        (None, Some(69), 2),
        (None, Some(70), 2),
        (None, Some(70), 2),
        (None, Some(70), 2),
        (None, Some(70), 2)
    ],
    [ // 42
        (None, Some(71), 2),
        (None, Some(71), 2),
        (None, Some(71), 2),
        (None, Some(71), 2),
        (None, Some(72), 2),
        (None, Some(72), 2),
        (None, Some(72), 2),
        (None, Some(72), 2)
    ],
    [ // 43
        (None, Some(73), 2),
        (None, Some(73), 2),
        (None, Some(73), 2),
        (None, Some(73), 2),
        (None, Some(74), 2),
        (None, Some(74), 2),
        (None, Some(74), 2),
        (None, Some(74), 2)
    ],
    [ // 44
        (None, Some(75), 2),
        (None, Some(75), 2),
        (None, Some(75), 2),
        (None, Some(75), 2),
        (None, Some(76), 2),
        (None, Some(76), 2),
        (None, Some(76), 2),
        (None, Some(76), 2)
    ],
    [ // 45
        (None, Some(77), 2),
        (None, Some(77), 2),
        (None, Some(77), 2),
        (None, Some(77), 2),
        (None, Some(78), 2),
        (None, Some(78), 2),
        (None, Some(78), 2),
        (None, Some(78), 2)
    ],
    [ // 46
        (None, Some(79), 2),
        (None, Some(79), 2),
        (None, Some(79), 2),
        (None, Some(79), 2),
        (None, Some(80), 2),
        (None, Some(80), 2),
        (None, Some(80), 2),
        (None, Some(80), 2)
    ],
    [ // 47
        (None, Some(81), 2),
        (None, Some(81), 2),
        (None, Some(81), 2),
        (None, Some(81), 2),
        (None, Some(82), 2),
        (None, Some(82), 2),
        (None, Some(82), 2),
        (None, Some(82), 2)
    ],
    [ // 48
        (None, Some(83), 2),
        (None, Some(83), 2),
        (None, Some(83), 2),
        (None, Some(83), 2),
        (None, Some(84), 2),
        (None, Some(84), 2),
        (None, Some(84), 2),
        (None, Some(84), 2)
    ],
    [ // 49
        (None, Some(85), 2),
        (None, Some(85), 2),
        (None, Some(85), 2),
        (None, Some(85), 2),
        (None, Some(86), 2),
        (None, Some(86), 2),
        (None, Some(86), 2),
        (None, Some(86), 2)
    ],
    [ // 50
        (None, Some(87), 2),
        (None, Some(87), 2),
        (None, Some(87), 2),
        (None, Some(87), 2),
        (None, Some(89), 2),
        (None, Some(89), 2),
        (None, Some(89), 2),
        (None, Some(89), 2)
    ],
    [ // 51
        (None, Some(92), 2),
        (None, Some(92), 2),
        (None, Some(92), 2),
        (None, Some(92), 2),
        (None, Some(195), 2),
        (None, Some(195), 2),
        (None, Some(195), 2),
        (None, Some(195), 2)
    ],
    [ // 52
        (None, Some(93), 2),
        (None, Some(93), 2),
        (None, Some(93), 2),
        (None, Some(93), 2),
        (None, Some(126), 2),
        (None, Some(126), 2),
        (None, Some(126), 2),
        (None, Some(126), 2)
    ],
    [ // 53
        (None, Some(99), 1),
        (None, Some(99), 1),
        (None, Some(101), 1),
        (None, Some(101), 1),
        (None, Some(105), 1),
        (None, Some(105), 1),
        (None, Some(111), 1),
        (None, Some(111), 1)
    ],
    [ // 54
        (None, Some(106), 2),
        (None, Some(106), 2),
        (None, Some(106), 2),
        (None, Some(106), 2),
        (None, Some(107), 2),
        (None, Some(107), 2),
        (None, Some(107), 2),
        (None, Some(107), 2)
    ],
    [ // 55
        (None, Some(113), 2),
        (None, Some(113), 2),
        (None, Some(113), 2),
        (None, Some(113), 2),
        (None, Some(118), 2),
        (None, Some(118), 2),
        (None, Some(118), 2),
        (None, Some(118), 2)
    ],
    [ // 56
        (None, Some(119), 2),
        (None, Some(119), 2),
        (None, Some(119), 2),
        (None, Some(119), 2),
        (None, Some(120), 2),
        (None, Some(120), 2),
        (None, Some(120), 2),
        (None, Some(120), 2)
    ],
    [ // 57
        (None, Some(121), 2),
        (None, Some(121), 2),
        (None, Some(121), 2),
        (None, Some(121), 2),
        (None, Some(122), 2),
        (None, Some(122), 2),
        (None, Some(122), 2),
        (None, Some(122), 2)
    ],
    [ // 58
        (None, Some(127), 2),
        (None, Some(127), 2),
        (None, Some(127), 2),
        (None, Some(127), 2),
        (None, Some(220), 2),
        (None, Some(220), 2),
        (None, Some(220), 2),
        (None, Some(220), 2)
    ],
    [ // 59
        (None, Some(208), 2),
        (None, Some(208), 2),
        (None, Some(208), 2),
        (None, Some(208), 2),
        (None, Some(128), 1),
        (None, Some(128), 1),
        (None, Some(130), 1),
        (None, Some(130), 1)
    ],
    [ // 60
        (None, Some(230), 0),
        (Some(61), None, 0),
        (Some(63), None, 0),
        (Some(64), None, 0),
        (Some(69), None, 0),
        (Some(70), None, 0),
        (Some(71), None, 0),
        (Some(73), None, 0)
    ],
    [ // 61
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(132), 2),
        (None, Some(132), 2),
        (None, Some(132), 2),
        (None, Some(132), 2)
    ],
    [ // 62
        (None, Some(131), 1),
        (None, Some(131), 1),
        (None, Some(162), 1),
        (None, Some(162), 1),
        (None, Some(184), 1),
        (None, Some(184), 1),
        (None, Some(194), 1),
        (None, Some(194), 1)
    ],
    [ // 63
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(134), 2),
        (None, Some(134), 2),
        (None, Some(134), 2),
        (None, Some(134), 2)
    ],
    [ // 64
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(146), 2),
        (None, Some(146), 2),
        (None, Some(146), 2),
        (None, Some(146), 2)
    ],
    [ // 65
        (None, Some(139), 1),
        (None, Some(139), 1),
        (None, Some(140), 1),
        (None, Some(140), 1),
        (None, Some(141), 1),
        (None, Some(141), 1),
        (None, Some(143), 1),
        (None, Some(143), 1)
    ],
    [ // 66
        (None, Some(147), 1),
        (None, Some(147), 1),
        (None, Some(149), 1),
        (None, Some(149), 1),
        (None, Some(150), 1),
        (None, Some(150), 1),
        (None, Some(151), 1),
        (None, Some(151), 1)
    ],
    [ // 67
        (None, Some(152), 1),
        (None, Some(152), 1),
        (None, Some(155), 1),
        (None, Some(155), 1),
        (None, Some(157), 1),
        (None, Some(157), 1),
        (None, Some(158), 1),
        (None, Some(158), 1)
    ],
    [ // 68
        (None, Some(224), 1),
        (None, Some(224), 1),
        (None, Some(226), 1),
        (None, Some(226), 1),
        (None, Some(153), 0),
        (None, Some(161), 0),
        (None, Some(167), 0),
        (None, Some(172), 0)
    ],
    [ // 69
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(156), 2),
        (None, Some(156), 2),
        (None, Some(156), 2),
        (None, Some(156), 2)
    ],
    [ // 70
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(163), 2),
        (None, Some(163), 2),
        (None, Some(163), 2),
        (None, Some(163), 2)
    ],
    [ // 71
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(169), 2),
        (None, Some(169), 2),
        (None, Some(169), 2),
        (None, Some(169), 2)
    ],
    [ // 72
        (None, Some(165), 1),
        (None, Some(165), 1),
        (None, Some(166), 1),
        (None, Some(166), 1),
        (None, Some(168), 1),
        (None, Some(168), 1),
        (None, Some(174), 1),
        (None, Some(174), 1)
    ],
    [ // 73
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(173), 2),
        (None, Some(173), 2),
        (None, Some(173), 2),
        (None, Some(173), 2)
    ],
    [ // 74
        (None, Some(171), 0),
        (None, Some(206), 0),
        (None, Some(215), 0),
        (None, Some(225), 0),
        (None, Some(236), 0),
        (None, Some(237), 0),
        (Some(84), None, 0),
        (Some(90), None, 0)
    ],
    [ // 75
        (None, Some(175), 1),
        (None, Some(175), 1),
        (None, Some(180), 1),
        (None, Some(180), 1),
        (None, Some(182), 1),
        (None, Some(182), 1),
        (None, Some(183), 1),
        (None, Some(183), 1)
    ],
    [ // 76
        (None, Some(176), 0),
        (None, Some(177), 0),
        (None, Some(179), 0),
        (None, Some(209), 0),
        (None, Some(216), 0),
        (None, Some(217), 0),
        (None, Some(227), 0),
        (None, Some(229), 0)
    ],
    [ // 77
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(181), 2),
        (None, Some(181), 2),
        (None, Some(181), 2),
        (None, Some(181), 2)
    ],
    [ // 78
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(186), 2),
        (None, Some(186), 2),
        (None, Some(186), 2),
        (None, Some(186), 2)
    ],
    [ // 79
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(189), 2),
        (None, Some(189), 2),
        (None, Some(189), 2),
        (None, Some(189), 2)
    ],
    [ // 80
        (None, Some(188), 1),
        (None, Some(188), 1),
        (None, Some(191), 1),
        (None, Some(191), 1),
        (None, Some(197), 1),
        (None, Some(197), 1),
        (None, Some(231), 1),
        (None, Some(231), 1)
    ],
    [ // 81
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(196), 2),
        (None, Some(196), 2),
        (None, Some(196), 2),
        (None, Some(196), 2)
    ],
    [ // 82
        (None, Some(192), 1),
        (None, Some(192), 1),
        (None, Some(193), 1),
        (None, Some(193), 1),
        (None, Some(200), 1),
        (None, Some(200), 1),
        (None, Some(201), 1),
        (None, Some(201), 1)
    ],
    [ // 83
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(228), 2),
        (None, Some(228), 2),
        (None, Some(228), 2),
        (None, Some(228), 2)
    ],
    [ // 84
        (None, Some(199), 2),
        (None, Some(199), 2),
        (None, Some(199), 2),
        (None, Some(199), 2),
        (None, Some(207), 2),
        (None, Some(207), 2),
        (None, Some(207), 2),
        (None, Some(207), 2)
    ],
    [ // 85
        (None, Some(202), 1),
        (None, Some(202), 1),
        (None, Some(205), 1),
        (None, Some(205), 1),
        (None, Some(210), 1),
        (None, Some(210), 1),
        (None, Some(213), 1),
        (None, Some(213), 1)
    ],
    [ // 86
        (None, Some(242), 1),
        (None, Some(242), 1),
        (None, Some(243), 1),
        (None, Some(243), 1),
        (None, Some(255), 1),
        (None, Some(255), 1),
        (None, Some(203), 0),
        (None, Some(204), 0)
    ],
    [ // 87
        (None, Some(211), 0),
        (None, Some(212), 0),
        (None, Some(214), 0),
        (None, Some(221), 0),
        (None, Some(222), 0),
        (None, Some(223), 0),
        (None, Some(241), 0),
        (None, Some(244), 0)
    ],
    [ // 88
        (None, Some(218), 1),
        (None, Some(218), 1),
        (None, Some(219), 1),
        (None, Some(219), 1),
        (None, Some(238), 1),
        (None, Some(238), 1),
        (None, Some(240), 1),
        (None, Some(240), 1)
    ],
    [ // 89
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(233), 2),
        (None, Some(233), 2),
        (None, Some(233), 2),
        (None, Some(233), 2)
    ],
    [ // 90
        (None, Some(234), 2),
        (None, Some(234), 2),
        (None, Some(234), 2),
        (None, Some(234), 2),
        (None, Some(235), 2),
        (None, Some(235), 2),
        (None, Some(235), 2),
        (None, Some(235), 2)
    ],
    [ // 91
        (None, Some(245), 0),
        (None, Some(246), 0),
        (None, Some(247), 0),
        (None, Some(248), 0),
        (None, Some(250), 0),
        (None, Some(251), 0),
        (None, Some(252), 0),
        (None, Some(253), 0)
    ]
];
