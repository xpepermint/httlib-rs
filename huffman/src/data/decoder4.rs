// next_id, ascii, leftover
pub const DECODE_TABLE: [[(Option<usize>, Option<usize>, usize); 16]; 54] = [
    [ // 0
        (Some(15), None, 0),
        (Some(16), None, 0),
        (Some(26), None, 0),
        (Some(28), None, 0),
        (Some(30), None, 0),
        (Some(12), None, 0),
        (Some(14), None, 0),
        (Some(17), None, 0),
        (Some(19), None, 0),
        (Some(27), None, 0),
        (Some(29), None, 0),
        (Some(18), None, 0),
        (Some(21), None, 0),
        (Some(22), None, 0),
        (Some(23), None, 0),
        (Some(1), None, 0)
    ],
    [ // 1
        (None, Some(119), 1),
        (None, Some(119), 1),
        (None, Some(120), 1),
        (None, Some(120), 1),
        (None, Some(121), 1),
        (None, Some(121), 1),
        (None, Some(122), 1),
        (None, Some(122), 1),
        (None, Some(38), 0),
        (None, Some(42), 0),
        (None, Some(44), 0),
        (None, Some(59), 0),
        (None, Some(88), 0),
        (None, Some(90), 0),
        (Some(13), None, 0),
        (Some(2), None, 0)
    ],
    [ // 2
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(63), 2),
        (None, Some(39), 1),
        (None, Some(39), 1),
        (None, Some(43), 1),
        (None, Some(43), 1),
        (None, Some(124), 1),
        (None, Some(124), 1),
        (None, Some(35), 0),
        (None, Some(62), 0),
        (Some(3), None, 0),
        (Some(20), None, 0),
        (Some(25), None, 0),
        (Some(4), None, 0)
    ],
    [ // 3
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(0), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3),
        (None, Some(36), 3)
    ],
    [ // 4
        (None, Some(94), 2),
        (None, Some(94), 2),
        (None, Some(94), 2),
        (None, Some(94), 2),
        (None, Some(125), 2),
        (None, Some(125), 2),
        (None, Some(125), 2),
        (None, Some(125), 2),
        (None, Some(60), 1),
        (None, Some(60), 1),
        (None, Some(96), 1),
        (None, Some(96), 1),
        (None, Some(123), 1),
        (None, Some(123), 1),
        (Some(24), None, 0),
        (Some(5), None, 0)
    ],
    [ // 5
        (Some(39), None, 0),
        (Some(41), None, 0),
        (Some(49), None, 0),
        (Some(51), None, 0),
        (Some(31), None, 0),
        (Some(32), None, 0),
        (Some(35), None, 0),
        (Some(36), None, 0),
        (Some(40), None, 0),
        (Some(42), None, 0),
        (Some(44), None, 0),
        (Some(6), None, 0),
        (Some(33), None, 0),
        (Some(37), None, 0),
        (Some(9), None, 0),
        (Some(7), None, 0)
    ],
    [ // 6
        (None, Some(1), 1),
        (None, Some(1), 1),
        (None, Some(135), 1),
        (None, Some(135), 1),
        (None, Some(137), 1),
        (None, Some(137), 1),
        (None, Some(138), 1),
        (None, Some(138), 1),
        (None, Some(139), 1),
        (None, Some(139), 1),
        (None, Some(140), 1),
        (None, Some(140), 1),
        (None, Some(141), 1),
        (None, Some(141), 1),
        (None, Some(143), 1),
        (None, Some(143), 1)
    ],
    [ // 7
        (None, Some(171), 0),
        (None, Some(206), 0),
        (None, Some(215), 0),
        (None, Some(225), 0),
        (None, Some(236), 0),
        (None, Some(237), 0),
        (Some(45), None, 0),
        (Some(52), None, 0),
        (Some(43), None, 0),
        (Some(46), None, 0),
        (Some(50), None, 0),
        (Some(47), None, 0),
        (Some(48), None, 0),
        (Some(53), None, 0),
        (Some(8), None, 0),
        (Some(10), None, 0)
    ],
    [ // 8
        (None, Some(254), 1),
        (None, Some(254), 1),
        (None, Some(2), 0),
        (None, Some(3), 0),
        (None, Some(4), 0),
        (None, Some(5), 0),
        (None, Some(6), 0),
        (None, Some(7), 0),
        (None, Some(8), 0),
        (None, Some(11), 0),
        (None, Some(12), 0),
        (None, Some(14), 0),
        (None, Some(15), 0),
        (None, Some(16), 0),
        (None, Some(17), 0),
        (None, Some(18), 0)
    ],
    [ // 9
        (None, Some(188), 1),
        (None, Some(188), 1),
        (None, Some(191), 1),
        (None, Some(191), 1),
        (None, Some(197), 1),
        (None, Some(197), 1),
        (None, Some(231), 1),
        (None, Some(231), 1),
        (None, Some(239), 1),
        (None, Some(239), 1),
        (None, Some(9), 0),
        (None, Some(142), 0),
        (None, Some(144), 0),
        (None, Some(145), 0),
        (None, Some(148), 0),
        (None, Some(159), 0)
    ],
    [ // 10
        (None, Some(19), 0),
        (None, Some(20), 0),
        (None, Some(21), 0),
        (None, Some(23), 0),
        (None, Some(24), 0),
        (None, Some(25), 0),
        (None, Some(26), 0),
        (None, Some(27), 0),
        (None, Some(28), 0),
        (None, Some(29), 0),
        (None, Some(30), 0),
        (None, Some(31), 0),
        (None, Some(127), 0),
        (None, Some(220), 0),
        (None, Some(249), 0),
        (Some(11), None, 0)
    ],
    [ // 11
        (None, Some(10), 2),
        (None, Some(10), 2),
        (None, Some(10), 2),
        (None, Some(10), 2),
        (None, Some(13), 2),
        (None, Some(13), 2),
        (None, Some(13), 2),
        (None, Some(13), 2),
        (None, Some(22), 2),
        (None, Some(22), 2),
        (None, Some(22), 2),
        (None, Some(22), 2),
        (None, Some(256), 2),
        (None, Some(256), 2),
        (None, Some(256), 2),
        (None, Some(256), 2)
    ],
    [ // 12
        (None, Some(32), 2),
        (None, Some(32), 2),
        (None, Some(32), 2),
        (None, Some(32), 2),
        (None, Some(37), 2),
        (None, Some(37), 2),
        (None, Some(37), 2),
        (None, Some(37), 2),
        (None, Some(45), 2),
        (None, Some(45), 2),
        (None, Some(45), 2),
        (None, Some(45), 2),
        (None, Some(46), 2),
        (None, Some(46), 2),
        (None, Some(46), 2),
        (None, Some(46), 2)
    ],
    [ // 13
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(33), 2),
        (None, Some(34), 2),
        (None, Some(34), 2),
        (None, Some(34), 2),
        (None, Some(34), 2),
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(40), 2),
        (None, Some(41), 2),
        (None, Some(41), 2),
        (None, Some(41), 2),
        (None, Some(41), 2)
    ],
    [ // 14
        (None, Some(47), 2),
        (None, Some(47), 2),
        (None, Some(47), 2),
        (None, Some(47), 2),
        (None, Some(51), 2),
        (None, Some(51), 2),
        (None, Some(51), 2),
        (None, Some(51), 2),
        (None, Some(52), 2),
        (None, Some(52), 2),
        (None, Some(52), 2),
        (None, Some(52), 2),
        (None, Some(53), 2),
        (None, Some(53), 2),
        (None, Some(53), 2),
        (None, Some(53), 2)
    ],
    [ // 15
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(48), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3),
        (None, Some(49), 3)
    ],
    [ // 16
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(50), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3),
        (None, Some(97), 3)
    ],
    [ // 17
        (None, Some(54), 2),
        (None, Some(54), 2),
        (None, Some(54), 2),
        (None, Some(54), 2),
        (None, Some(55), 2),
        (None, Some(55), 2),
        (None, Some(55), 2),
        (None, Some(55), 2),
        (None, Some(56), 2),
        (None, Some(56), 2),
        (None, Some(56), 2),
        (None, Some(56), 2),
        (None, Some(57), 2),
        (None, Some(57), 2),
        (None, Some(57), 2),
        (None, Some(57), 2)
    ],
    [ // 18
        (None, Some(114), 2),
        (None, Some(114), 2),
        (None, Some(114), 2),
        (None, Some(114), 2),
        (None, Some(117), 2),
        (None, Some(117), 2),
        (None, Some(117), 2),
        (None, Some(117), 2),
        (None, Some(58), 1),
        (None, Some(58), 1),
        (None, Some(66), 1),
        (None, Some(66), 1),
        (None, Some(67), 1),
        (None, Some(67), 1),
        (None, Some(68), 1),
        (None, Some(68), 1)
    ],
    [ // 19
        (None, Some(61), 2),
        (None, Some(61), 2),
        (None, Some(61), 2),
        (None, Some(61), 2),
        (None, Some(65), 2),
        (None, Some(65), 2),
        (None, Some(65), 2),
        (None, Some(65), 2),
        (None, Some(95), 2),
        (None, Some(95), 2),
        (None, Some(95), 2),
        (None, Some(95), 2),
        (None, Some(98), 2),
        (None, Some(98), 2),
        (None, Some(98), 2),
        (None, Some(98), 2)
    ],
    [ // 20
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(64), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3),
        (None, Some(91), 3)
    ],
    [ // 21
        (None, Some(69), 1),
        (None, Some(69), 1),
        (None, Some(70), 1),
        (None, Some(70), 1),
        (None, Some(71), 1),
        (None, Some(71), 1),
        (None, Some(72), 1),
        (None, Some(72), 1),
        (None, Some(73), 1),
        (None, Some(73), 1),
        (None, Some(74), 1),
        (None, Some(74), 1),
        (None, Some(75), 1),
        (None, Some(75), 1),
        (None, Some(76), 1),
        (None, Some(76), 1)
    ],
    [ // 22
        (None, Some(77), 1),
        (None, Some(77), 1),
        (None, Some(78), 1),
        (None, Some(78), 1),
        (None, Some(79), 1),
        (None, Some(79), 1),
        (None, Some(80), 1),
        (None, Some(80), 1),
        (None, Some(81), 1),
        (None, Some(81), 1),
        (None, Some(82), 1),
        (None, Some(82), 1),
        (None, Some(83), 1),
        (None, Some(83), 1),
        (None, Some(84), 1),
        (None, Some(84), 1)
    ],
    [ // 23
        (None, Some(85), 1),
        (None, Some(85), 1),
        (None, Some(86), 1),
        (None, Some(86), 1),
        (None, Some(87), 1),
        (None, Some(87), 1),
        (None, Some(89), 1),
        (None, Some(89), 1),
        (None, Some(106), 1),
        (None, Some(106), 1),
        (None, Some(107), 1),
        (None, Some(107), 1),
        (None, Some(113), 1),
        (None, Some(113), 1),
        (None, Some(118), 1),
        (None, Some(118), 1)
    ],
    [ // 24
        (None, Some(92), 1),
        (None, Some(92), 1),
        (None, Some(195), 1),
        (None, Some(195), 1),
        (None, Some(208), 1),
        (None, Some(208), 1),
        (None, Some(128), 0),
        (None, Some(130), 0),
        (None, Some(131), 0),
        (None, Some(162), 0),
        (None, Some(184), 0),
        (None, Some(194), 0),
        (None, Some(224), 0),
        (None, Some(226), 0),
        (Some(34), None, 0),
        (Some(38), None, 0)
    ],
    [ // 25
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(93), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3),
        (None, Some(126), 3)
    ],
    [ // 26
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(99), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3),
        (None, Some(101), 3)
    ],
    [ // 27
        (None, Some(100), 2),
        (None, Some(100), 2),
        (None, Some(100), 2),
        (None, Some(100), 2),
        (None, Some(102), 2),
        (None, Some(102), 2),
        (None, Some(102), 2),
        (None, Some(102), 2),
        (None, Some(103), 2),
        (None, Some(103), 2),
        (None, Some(103), 2),
        (None, Some(103), 2),
        (None, Some(104), 2),
        (None, Some(104), 2),
        (None, Some(104), 2),
        (None, Some(104), 2)
    ],
    [ // 28
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(105), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3),
        (None, Some(111), 3)
    ],
    [ // 29
        (None, Some(108), 2),
        (None, Some(108), 2),
        (None, Some(108), 2),
        (None, Some(108), 2),
        (None, Some(109), 2),
        (None, Some(109), 2),
        (None, Some(109), 2),
        (None, Some(109), 2),
        (None, Some(110), 2),
        (None, Some(110), 2),
        (None, Some(110), 2),
        (None, Some(110), 2),
        (None, Some(112), 2),
        (None, Some(112), 2),
        (None, Some(112), 2),
        (None, Some(112), 2)
    ],
    [ // 30
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(115), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3),
        (None, Some(116), 3)
    ],
    [ // 31
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(230), 3),
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(129), 2),
        (None, Some(132), 2),
        (None, Some(132), 2),
        (None, Some(132), 2),
        (None, Some(132), 2)
    ],
    [ // 32
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(133), 2),
        (None, Some(134), 2),
        (None, Some(134), 2),
        (None, Some(134), 2),
        (None, Some(134), 2),
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(136), 2),
        (None, Some(146), 2),
        (None, Some(146), 2),
        (None, Some(146), 2),
        (None, Some(146), 2)
    ],
    [ // 33
        (None, Some(147), 1),
        (None, Some(147), 1),
        (None, Some(149), 1),
        (None, Some(149), 1),
        (None, Some(150), 1),
        (None, Some(150), 1),
        (None, Some(151), 1),
        (None, Some(151), 1),
        (None, Some(152), 1),
        (None, Some(152), 1),
        (None, Some(155), 1),
        (None, Some(155), 1),
        (None, Some(157), 1),
        (None, Some(157), 1),
        (None, Some(158), 1),
        (None, Some(158), 1)
    ],
    [ // 34
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(153), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3),
        (None, Some(161), 3)
    ],
    [ // 35
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(154), 2),
        (None, Some(156), 2),
        (None, Some(156), 2),
        (None, Some(156), 2),
        (None, Some(156), 2),
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(160), 2),
        (None, Some(163), 2),
        (None, Some(163), 2),
        (None, Some(163), 2),
        (None, Some(163), 2)
    ],
    [ // 36
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(164), 2),
        (None, Some(169), 2),
        (None, Some(169), 2),
        (None, Some(169), 2),
        (None, Some(169), 2),
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(170), 2),
        (None, Some(173), 2),
        (None, Some(173), 2),
        (None, Some(173), 2),
        (None, Some(173), 2)
    ],
    [ // 37
        (None, Some(165), 1),
        (None, Some(165), 1),
        (None, Some(166), 1),
        (None, Some(166), 1),
        (None, Some(168), 1),
        (None, Some(168), 1),
        (None, Some(174), 1),
        (None, Some(174), 1),
        (None, Some(175), 1),
        (None, Some(175), 1),
        (None, Some(180), 1),
        (None, Some(180), 1),
        (None, Some(182), 1),
        (None, Some(182), 1),
        (None, Some(183), 1),
        (None, Some(183), 1)
    ],
    [ // 38
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(167), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3),
        (None, Some(172), 3)
    ],
    [ // 39
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(176), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3),
        (None, Some(177), 3)
    ],
    [ // 40
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(178), 2),
        (None, Some(181), 2),
        (None, Some(181), 2),
        (None, Some(181), 2),
        (None, Some(181), 2),
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(185), 2),
        (None, Some(186), 2),
        (None, Some(186), 2),
        (None, Some(186), 2),
        (None, Some(186), 2)
    ],
    [ // 41
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(179), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3),
        (None, Some(209), 3)
    ],
    [ // 42
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(187), 2),
        (None, Some(189), 2),
        (None, Some(189), 2),
        (None, Some(189), 2),
        (None, Some(189), 2),
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(190), 2),
        (None, Some(196), 2),
        (None, Some(196), 2),
        (None, Some(196), 2),
        (None, Some(196), 2)
    ],
    [ // 43
        (None, Some(192), 2),
        (None, Some(192), 2),
        (None, Some(192), 2),
        (None, Some(192), 2),
        (None, Some(193), 2),
        (None, Some(193), 2),
        (None, Some(193), 2),
        (None, Some(193), 2),
        (None, Some(200), 2),
        (None, Some(200), 2),
        (None, Some(200), 2),
        (None, Some(200), 2),
        (None, Some(201), 2),
        (None, Some(201), 2),
        (None, Some(201), 2),
        (None, Some(201), 2)
    ],
    [ // 44
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(198), 2),
        (None, Some(228), 2),
        (None, Some(228), 2),
        (None, Some(228), 2),
        (None, Some(228), 2),
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(232), 2),
        (None, Some(233), 2),
        (None, Some(233), 2),
        (None, Some(233), 2),
        (None, Some(233), 2)
    ],
    [ // 45
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(199), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3),
        (None, Some(207), 3)
    ],
    [ // 46
        (None, Some(202), 2),
        (None, Some(202), 2),
        (None, Some(202), 2),
        (None, Some(202), 2),
        (None, Some(205), 2),
        (None, Some(205), 2),
        (None, Some(205), 2),
        (None, Some(205), 2),
        (None, Some(210), 2),
        (None, Some(210), 2),
        (None, Some(210), 2),
        (None, Some(210), 2),
        (None, Some(213), 2),
        (None, Some(213), 2),
        (None, Some(213), 2),
        (None, Some(213), 2)
    ],
    [ // 47
        (None, Some(242), 2),
        (None, Some(242), 2),
        (None, Some(242), 2),
        (None, Some(242), 2),
        (None, Some(243), 2),
        (None, Some(243), 2),
        (None, Some(243), 2),
        (None, Some(243), 2),
        (None, Some(255), 2),
        (None, Some(255), 2),
        (None, Some(255), 2),
        (None, Some(255), 2),
        (None, Some(203), 1),
        (None, Some(203), 1),
        (None, Some(204), 1),
        (None, Some(204), 1)
    ],
    [ // 48
        (None, Some(211), 1),
        (None, Some(211), 1),
        (None, Some(212), 1),
        (None, Some(212), 1),
        (None, Some(214), 1),
        (None, Some(214), 1),
        (None, Some(221), 1),
        (None, Some(221), 1),
        (None, Some(222), 1),
        (None, Some(222), 1),
        (None, Some(223), 1),
        (None, Some(223), 1),
        (None, Some(241), 1),
        (None, Some(241), 1),
        (None, Some(244), 1),
        (None, Some(244), 1)
    ],
    [ // 49
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(216), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3),
        (None, Some(217), 3)
    ],
    [ // 50
        (None, Some(218), 2),
        (None, Some(218), 2),
        (None, Some(218), 2),
        (None, Some(218), 2),
        (None, Some(219), 2),
        (None, Some(219), 2),
        (None, Some(219), 2),
        (None, Some(219), 2),
        (None, Some(238), 2),
        (None, Some(238), 2),
        (None, Some(238), 2),
        (None, Some(238), 2),
        (None, Some(240), 2),
        (None, Some(240), 2),
        (None, Some(240), 2),
        (None, Some(240), 2)
    ],
    [ // 51
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(227), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3),
        (None, Some(229), 3)
    ],
    [ // 52
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(234), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3),
        (None, Some(235), 3)
    ],
    [ // 53
        (None, Some(245), 1),
        (None, Some(245), 1),
        (None, Some(246), 1),
        (None, Some(246), 1),
        (None, Some(247), 1),
        (None, Some(247), 1),
        (None, Some(248), 1),
        (None, Some(248), 1),
        (None, Some(250), 1),
        (None, Some(250), 1),
        (None, Some(251), 1),
        (None, Some(251), 1),
        (None, Some(252), 1),
        (None, Some(252), 1),
        (None, Some(253), 1),
        (None, Some(253), 1)
    ]
];
