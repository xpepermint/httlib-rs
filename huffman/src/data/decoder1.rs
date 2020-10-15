/// This is a static translation matrix for decoding Huffman sequence by reading
/// 1-bit at at time.
pub const DECODE_TABLE: [[(Option<usize>, Option<usize>, usize); 2]; 256] = [ // (next_id, ascii, leftover)
    [ // 0
        (Some(66), None, 0),
        (Some(1), None, 0)
    ],
    [ // 1
        (Some(93), None, 0),
        (Some(2), None, 0)
    ],
    [ // 2
        (Some(104), None, 0),
        (Some(3), None, 0)
    ],
    [ // 3
        (Some(119), None, 0),
        (Some(4), None, 0)
    ],
    [ // 4
        (Some(144), None, 0),
        (Some(5), None, 0)
    ],
    [ // 5
        (Some(75), None, 0),
        (Some(6), None, 0)
    ],
    [ // 6
        (Some(123), None, 0),
        (Some(7), None, 0)
    ],
    [ // 7
        (Some(71), None, 0),
        (Some(8), None, 0)
    ],
    [ // 8
        (Some(77), None, 0),
        (Some(9), None, 0)
    ],
    [ // 9
        (Some(73), None, 0),
        (Some(10), None, 0)
    ],
    [ // 10
        (Some(11), None, 0),
        (Some(13), None, 0)
    ],
    [ // 11
        (Some(12), None, 0),
        (Some(102), None, 0)
    ],
    [ // 12
        (None, Some(0), 0),
        (None, Some(36), 0)
    ],
    [ // 13
        (Some(127), None, 0),
        (Some(14), None, 0)
    ],
    [ // 14
        (Some(128), None, 0),
        (Some(15), None, 0)
    ],
    [ // 15
        (Some(98), None, 0),
        (Some(16), None, 0)
    ],
    [ // 16
        (None, Some(123), 0),
        (Some(17), None, 0)
    ],
    [ // 17
        (Some(124), None, 0),
        (Some(18), None, 0)
    ],
    [ // 18
        (Some(150), None, 0),
        (Some(19), None, 0)
    ],
    [ // 19
        (Some(20), None, 0),
        (Some(25), None, 0)
    ],
    [ // 20
        (Some(199), None, 0),
        (Some(21), None, 0)
    ],
    [ // 21
        (Some(216), None, 0),
        (Some(22), None, 0)
    ],
    [ // 22
        (Some(23), None, 0),
        (Some(162), None, 0)
    ],
    [ // 23
        (Some(24), None, 0),
        (Some(161), None, 0)
    ],
    [ // 24
        (None, Some(1), 0),
        (None, Some(135), 0)
    ],
    [ // 25
        (Some(167), None, 0),
        (Some(26), None, 0)
    ],
    [ // 26
        (Some(41), None, 0),
        (Some(27), None, 0)
    ],
    [ // 27
        (Some(191), None, 0),
        (Some(28), None, 0)
    ],
    [ // 28
        (Some(211), None, 0),
        (Some(29), None, 0)
    ],
    [ // 29
        (Some(229), None, 0),
        (Some(30), None, 0)
    ],
    [ // 30
        (Some(31), None, 0),
        (Some(45), None, 0)
    ],
    [ // 31
        (Some(32), None, 0),
        (Some(38), None, 0)
    ],
    [ // 32
        (Some(33), None, 0),
        (Some(35), None, 0)
    ],
    [ // 33
        (None, Some(254), 0),
        (Some(34), None, 0)
    ],
    [ // 34
        (None, Some(2), 0),
        (None, Some(3), 0)
    ],
    [ // 35
        (Some(36), None, 0),
        (Some(37), None, 0)
    ],
    [ // 36
        (None, Some(4), 0),
        (None, Some(5), 0)
    ],
    [ // 37
        (None, Some(6), 0),
        (None, Some(7), 0)
    ],
    [ // 38
        (Some(39), None, 0),
        (Some(52), None, 0)
    ],
    [ // 39
        (Some(40), None, 0),
        (Some(51), None, 0)
    ],
    [ // 40
        (None, Some(8), 0),
        (None, Some(11), 0)
    ],
    [ // 41
        (Some(208), None, 0),
        (Some(42), None, 0)
    ],
    [ // 42
        (Some(43), None, 0),
        (Some(165), None, 0)
    ],
    [ // 43
        (None, Some(239), 0),
        (Some(44), None, 0)
    ],
    [ // 44
        (None, Some(9), 0),
        (None, Some(142), 0)
    ],
    [ // 45
        (Some(55), None, 0),
        (Some(46), None, 0)
    ],
    [ // 46
        (Some(63), None, 0),
        (Some(47), None, 0)
    ],
    [ // 47
        (Some(147), None, 0),
        (Some(48), None, 0)
    ],
    [ // 48
        (None, Some(249), 0),
        (Some(49), None, 0)
    ],
    [ // 49
        (Some(50), None, 0),
        (Some(59), None, 0)
    ],
    [ // 50
        (None, Some(10), 0),
        (None, Some(13), 0)
    ],
    [ // 51
        (None, Some(12), 0),
        (None, Some(14), 0)
    ],
    [ // 52
        (Some(53), None, 0),
        (Some(54), None, 0)
    ],
    [ // 53
        (None, Some(15), 0),
        (None, Some(16), 0)
    ],
    [ // 54
        (None, Some(17), 0),
        (None, Some(18), 0)
    ],
    [ // 55
        (Some(56), None, 0),
        (Some(60), None, 0)
    ],
    [ // 56
        (Some(57), None, 0),
        (Some(58), None, 0)
    ],
    [ // 57
        (None, Some(19), 0),
        (None, Some(20), 0)
    ],
    [ // 58
        (None, Some(21), 0),
        (None, Some(23), 0)
    ],
    [ // 59
        (None, Some(22), 0),
        (None, Some(256), 0)
    ],
    [ // 60
        (Some(61), None, 0),
        (Some(62), None, 0)
    ],
    [ // 61
        (None, Some(24), 0),
        (None, Some(25), 0)
    ],
    [ // 62
        (None, Some(26), 0),
        (None, Some(27), 0)
    ],
    [ // 63
        (Some(64), None, 0),
        (Some(65), None, 0)
    ],
    [ // 64
        (None, Some(28), 0),
        (None, Some(29), 0)
    ],
    [ // 65
        (None, Some(30), 0),
        (None, Some(31), 0)
    ],
    [ // 66
        (Some(85), None, 0),
        (Some(67), None, 0)
    ],
    [ // 67
        (Some(68), None, 0),
        (Some(82), None, 0)
    ],
    [ // 68
        (Some(143), None, 0),
        (Some(69), None, 0)
    ],
    [ // 69
        (Some(70), None, 0),
        (Some(81), None, 0)
    ],
    [ // 70
        (None, Some(32), 0),
        (None, Some(37), 0)
    ],
    [ // 71
        (Some(72), None, 0),
        (Some(79), None, 0)
    ],
    [ // 72
        (None, Some(33), 0),
        (None, Some(34), 0)
    ],
    [ // 73
        (None, Some(124), 0),
        (Some(74), None, 0)
    ],
    [ // 74
        (None, Some(35), 0),
        (None, Some(62), 0)
    ],
    [ // 75
        (Some(76), None, 0),
        (Some(80), None, 0)
    ],
    [ // 76
        (None, Some(38), 0),
        (None, Some(42), 0)
    ],
    [ // 77
        (None, Some(63), 0),
        (Some(78), None, 0)
    ],
    [ // 78
        (None, Some(39), 0),
        (None, Some(43), 0)
    ],
    [ // 79
        (None, Some(40), 0),
        (None, Some(41), 0)
    ],
    [ // 80
        (None, Some(44), 0),
        (None, Some(59), 0)
    ],
    [ // 81
        (None, Some(45), 0),
        (None, Some(46), 0)
    ],
    [ // 82
        (Some(83), None, 0),
        (Some(90), None, 0)
    ],
    [ // 83
        (Some(84), None, 0),
        (Some(89), None, 0)
    ],
    [ // 84
        (None, Some(47), 0),
        (None, Some(51), 0)
    ],
    [ // 85
        (Some(86), None, 0),
        (Some(130), None, 0)
    ],
    [ // 86
        (Some(87), None, 0),
        (Some(88), None, 0)
    ],
    [ // 87
        (None, Some(48), 0),
        (None, Some(49), 0)
    ],
    [ // 88
        (None, Some(50), 0),
        (None, Some(97), 0)
    ],
    [ // 89
        (None, Some(52), 0),
        (None, Some(53), 0)
    ],
    [ // 90
        (Some(91), None, 0),
        (Some(92), None, 0)
    ],
    [ // 91
        (None, Some(54), 0),
        (None, Some(55), 0)
    ],
    [ // 92
        (None, Some(56), 0),
        (None, Some(57), 0)
    ],
    [ // 93
        (Some(99), None, 0),
        (Some(94), None, 0)
    ],
    [ // 94
        (Some(138), None, 0),
        (Some(95), None, 0)
    ],
    [ // 95
        (Some(142), None, 0),
        (Some(96), None, 0)
    ],
    [ // 96
        (Some(97), None, 0),
        (Some(103), None, 0)
    ],
    [ // 97
        (None, Some(58), 0),
        (None, Some(66), 0)
    ],
    [ // 98
        (None, Some(60), 0),
        (None, Some(96), 0)
    ],
    [ // 99
        (Some(100), None, 0),
        (Some(132), None, 0)
    ],
    [ // 100
        (Some(101), None, 0),
        (Some(129), None, 0)
    ],
    [ // 101
        (None, Some(61), 0),
        (None, Some(65), 0)
    ],
    [ // 102
        (None, Some(64), 0),
        (None, Some(91), 0)
    ],
    [ // 103
        (None, Some(67), 0),
        (None, Some(68), 0)
    ],
    [ // 104
        (Some(105), None, 0),
        (Some(112), None, 0)
    ],
    [ // 105
        (Some(106), None, 0),
        (Some(109), None, 0)
    ],
    [ // 106
        (Some(107), None, 0),
        (Some(108), None, 0)
    ],
    [ // 107
        (None, Some(69), 0),
        (None, Some(70), 0)
    ],
    [ // 108
        (None, Some(71), 0),
        (None, Some(72), 0)
    ],
    [ // 109
        (Some(110), None, 0),
        (Some(111), None, 0)
    ],
    [ // 110
        (None, Some(73), 0),
        (None, Some(74), 0)
    ],
    [ // 111
        (None, Some(75), 0),
        (None, Some(76), 0)
    ],
    [ // 112
        (Some(113), None, 0),
        (Some(116), None, 0)
    ],
    [ // 113
        (Some(114), None, 0),
        (Some(115), None, 0)
    ],
    [ // 114
        (None, Some(77), 0),
        (None, Some(78), 0)
    ],
    [ // 115
        (None, Some(79), 0),
        (None, Some(80), 0)
    ],
    [ // 116
        (Some(117), None, 0),
        (Some(118), None, 0)
    ],
    [ // 117
        (None, Some(81), 0),
        (None, Some(82), 0)
    ],
    [ // 118
        (None, Some(83), 0),
        (None, Some(84), 0)
    ],
    [ // 119
        (Some(120), None, 0),
        (Some(136), None, 0)
    ],
    [ // 120
        (Some(121), None, 0),
        (Some(122), None, 0)
    ],
    [ // 121
        (None, Some(85), 0),
        (None, Some(86), 0)
    ],
    [ // 122
        (None, Some(87), 0),
        (None, Some(89), 0)
    ],
    [ // 123
        (None, Some(88), 0),
        (None, Some(90), 0)
    ],
    [ // 124
        (Some(125), None, 0),
        (Some(155), None, 0)
    ],
    [ // 125
        (Some(126), None, 0),
        (Some(148), None, 0)
    ],
    [ // 126
        (None, Some(92), 0),
        (None, Some(195), 0)
    ],
    [ // 127
        (None, Some(93), 0),
        (None, Some(126), 0)
    ],
    [ // 128
        (None, Some(94), 0),
        (None, Some(125), 0)
    ],
    [ // 129
        (None, Some(95), 0),
        (None, Some(98), 0)
    ],
    [ // 130
        (Some(131), None, 0),
        (Some(135), None, 0)
    ],
    [ // 131
        (None, Some(99), 0),
        (None, Some(101), 0)
    ],
    [ // 132
        (Some(133), None, 0),
        (Some(134), None, 0)
    ],
    [ // 133
        (None, Some(100), 0),
        (None, Some(102), 0)
    ],
    [ // 134
        (None, Some(103), 0),
        (None, Some(104), 0)
    ],
    [ // 135
        (None, Some(105), 0),
        (None, Some(111), 0)
    ],
    [ // 136
        (Some(137), None, 0),
        (Some(141), None, 0)
    ],
    [ // 137
        (None, Some(106), 0),
        (None, Some(107), 0)
    ],
    [ // 138
        (Some(139), None, 0),
        (Some(140), None, 0)
    ],
    [ // 139
        (None, Some(108), 0),
        (None, Some(109), 0)
    ],
    [ // 140
        (None, Some(110), 0),
        (None, Some(112), 0)
    ],
    [ // 141
        (None, Some(113), 0),
        (None, Some(118), 0)
    ],
    [ // 142
        (None, Some(114), 0),
        (None, Some(117), 0)
    ],
    [ // 143
        (None, Some(115), 0),
        (None, Some(116), 0)
    ],
    [ // 144
        (Some(145), None, 0),
        (Some(146), None, 0)
    ],
    [ // 145
        (None, Some(119), 0),
        (None, Some(120), 0)
    ],
    [ // 146
        (None, Some(121), 0),
        (None, Some(122), 0)
    ],
    [ // 147
        (None, Some(127), 0),
        (None, Some(220), 0)
    ],
    [ // 148
        (None, Some(208), 0),
        (Some(149), None, 0)
    ],
    [ // 149
        (None, Some(128), 0),
        (None, Some(130), 0)
    ],
    [ // 150
        (Some(196), None, 0),
        (Some(151), None, 0)
    ],
    [ // 151
        (Some(152), None, 0),
        (Some(178), None, 0)
    ],
    [ // 152
        (Some(153), None, 0),
        (Some(158), None, 0)
    ],
    [ // 153
        (None, Some(230), 0),
        (Some(154), None, 0)
    ],
    [ // 154
        (None, Some(129), 0),
        (None, Some(132), 0)
    ],
    [ // 155
        (Some(156), None, 0),
        (Some(175), None, 0)
    ],
    [ // 156
        (Some(157), None, 0),
        (Some(204), None, 0)
    ],
    [ // 157
        (None, Some(131), 0),
        (None, Some(162), 0)
    ],
    [ // 158
        (Some(159), None, 0),
        (Some(160), None, 0)
    ],
    [ // 159
        (None, Some(133), 0),
        (None, Some(134), 0)
    ],
    [ // 160
        (None, Some(136), 0),
        (None, Some(146), 0)
    ],
    [ // 161
        (None, Some(137), 0),
        (None, Some(138), 0)
    ],
    [ // 162
        (Some(163), None, 0),
        (Some(164), None, 0)
    ],
    [ // 163
        (None, Some(139), 0),
        (None, Some(140), 0)
    ],
    [ // 164
        (None, Some(141), 0),
        (None, Some(143), 0)
    ],
    [ // 165
        (Some(166), None, 0),
        (Some(171), None, 0)
    ],
    [ // 166
        (None, Some(144), 0),
        (None, Some(145), 0)
    ],
    [ // 167
        (Some(168), None, 0),
        (Some(185), None, 0)
    ],
    [ // 168
        (Some(169), None, 0),
        (Some(173), None, 0)
    ],
    [ // 169
        (Some(170), None, 0),
        (Some(172), None, 0)
    ],
    [ // 170
        (None, Some(147), 0),
        (None, Some(149), 0)
    ],
    [ // 171
        (None, Some(148), 0),
        (None, Some(159), 0)
    ],
    [ // 172
        (None, Some(150), 0),
        (None, Some(151), 0)
    ],
    [ // 173
        (Some(174), None, 0),
        (Some(181), None, 0)
    ],
    [ // 174
        (None, Some(152), 0),
        (None, Some(155), 0)
    ],
    [ // 175
        (Some(241), None, 0),
        (Some(176), None, 0)
    ],
    [ // 176
        (Some(177), None, 0),
        (Some(188), None, 0)
    ],
    [ // 177
        (None, Some(153), 0),
        (None, Some(161), 0)
    ],
    [ // 178
        (Some(179), None, 0),
        (Some(183), None, 0)
    ],
    [ // 179
        (Some(180), None, 0),
        (Some(182), None, 0)
    ],
    [ // 180
        (None, Some(154), 0),
        (None, Some(156), 0)
    ],
    [ // 181
        (None, Some(157), 0),
        (None, Some(158), 0)
    ],
    [ // 182
        (None, Some(160), 0),
        (None, Some(163), 0)
    ],
    [ // 183
        (Some(184), None, 0),
        (Some(190), None, 0)
    ],
    [ // 184
        (None, Some(164), 0),
        (None, Some(169), 0)
    ],
    [ // 185
        (Some(186), None, 0),
        (Some(194), None, 0)
    ],
    [ // 186
        (Some(187), None, 0),
        (Some(189), None, 0)
    ],
    [ // 187
        (None, Some(165), 0),
        (None, Some(166), 0)
    ],
    [ // 188
        (None, Some(167), 0),
        (None, Some(172), 0)
    ],
    [ // 189
        (None, Some(168), 0),
        (None, Some(174), 0)
    ],
    [ // 190
        (None, Some(170), 0),
        (None, Some(173), 0)
    ],
    [ // 191
        (Some(192), None, 0),
        (Some(218), None, 0)
    ],
    [ // 192
        (Some(193), None, 0),
        (Some(234), None, 0)
    ],
    [ // 193
        (None, Some(171), 0),
        (None, Some(206), 0)
    ],
    [ // 194
        (Some(195), None, 0),
        (Some(203), None, 0)
    ],
    [ // 195
        (None, Some(175), 0),
        (None, Some(180), 0)
    ],
    [ // 196
        (Some(197), None, 0),
        (Some(235), None, 0)
    ],
    [ // 197
        (Some(198), None, 0),
        (Some(202), None, 0)
    ],
    [ // 198
        (None, Some(176), 0),
        (None, Some(177), 0)
    ],
    [ // 199
        (Some(200), None, 0),
        (Some(206), None, 0)
    ],
    [ // 200
        (Some(201), None, 0),
        (Some(205), None, 0)
    ],
    [ // 201
        (None, Some(178), 0),
        (None, Some(181), 0)
    ],
    [ // 202
        (None, Some(179), 0),
        (None, Some(209), 0)
    ],
    [ // 203
        (None, Some(182), 0),
        (None, Some(183), 0)
    ],
    [ // 204
        (None, Some(184), 0),
        (None, Some(194), 0)
    ],
    [ // 205
        (None, Some(185), 0),
        (None, Some(186), 0)
    ],
    [ // 206
        (Some(207), None, 0),
        (Some(210), None, 0)
    ],
    [ // 207
        (None, Some(187), 0),
        (None, Some(189), 0)
    ],
    [ // 208
        (Some(209), None, 0),
        (Some(215), None, 0)
    ],
    [ // 209
        (None, Some(188), 0),
        (None, Some(191), 0)
    ],
    [ // 210
        (None, Some(190), 0),
        (None, Some(196), 0)
    ],
    [ // 211
        (Some(212), None, 0),
        (Some(224), None, 0)
    ],
    [ // 212
        (Some(213), None, 0),
        (Some(222), None, 0)
    ],
    [ // 213
        (Some(214), None, 0),
        (Some(221), None, 0)
    ],
    [ // 214
        (None, Some(192), 0),
        (None, Some(193), 0)
    ],
    [ // 215
        (None, Some(197), 0),
        (None, Some(231), 0)
    ],
    [ // 216
        (Some(217), None, 0),
        (Some(243), None, 0)
    ],
    [ // 217
        (None, Some(198), 0),
        (None, Some(228), 0)
    ],
    [ // 218
        (Some(245), None, 0),
        (Some(219), None, 0)
    ],
    [ // 219
        (Some(220), None, 0),
        (Some(244), None, 0)
    ],
    [ // 220
        (None, Some(199), 0),
        (None, Some(207), 0)
    ],
    [ // 221
        (None, Some(200), 0),
        (None, Some(201), 0)
    ],
    [ // 222
        (Some(223), None, 0),
        (Some(228), None, 0)
    ],
    [ // 223
        (None, Some(202), 0),
        (None, Some(205), 0)
    ],
    [ // 224
        (Some(237), None, 0),
        (Some(225), None, 0)
    ],
    [ // 225
        (Some(248), None, 0),
        (Some(226), None, 0)
    ],
    [ // 226
        (None, Some(255), 0),
        (Some(227), None, 0)
    ],
    [ // 227
        (None, Some(203), 0),
        (None, Some(204), 0)
    ],
    [ // 228
        (None, Some(210), 0),
        (None, Some(213), 0)
    ],
    [ // 229
        (Some(230), None, 0),
        (Some(249), None, 0)
    ],
    [ // 230
        (Some(231), None, 0),
        (Some(239), None, 0)
    ],
    [ // 231
        (Some(232), None, 0),
        (Some(233), None, 0)
    ],
    [ // 232
        (None, Some(211), 0),
        (None, Some(212), 0)
    ],
    [ // 233
        (None, Some(214), 0),
        (None, Some(221), 0)
    ],
    [ // 234
        (None, Some(215), 0),
        (None, Some(225), 0)
    ],
    [ // 235
        (Some(236), None, 0),
        (Some(242), None, 0)
    ],
    [ // 236
        (None, Some(216), 0),
        (None, Some(217), 0)
    ],
    [ // 237
        (Some(238), None, 0),
        (Some(246), None, 0)
    ],
    [ // 238
        (None, Some(218), 0),
        (None, Some(219), 0)
    ],
    [ // 239
        (Some(240), None, 0),
        (Some(247), None, 0)
    ],
    [ // 240
        (None, Some(222), 0),
        (None, Some(223), 0)
    ],
    [ // 241
        (None, Some(224), 0),
        (None, Some(226), 0)
    ],
    [ // 242
        (None, Some(227), 0),
        (None, Some(229), 0)
    ],
    [ // 243
        (None, Some(232), 0),
        (None, Some(233), 0)
    ],
    [ // 244
        (None, Some(234), 0),
        (None, Some(235), 0)
    ],
    [ // 245
        (None, Some(236), 0),
        (None, Some(237), 0)
    ],
    [ // 246
        (None, Some(238), 0),
        (None, Some(240), 0)
    ],
    [ // 247
        (None, Some(241), 0),
        (None, Some(244), 0)
    ],
    [ // 248
        (None, Some(242), 0),
        (None, Some(243), 0)
    ],
    [ // 249
        (Some(250), None, 0),
        (Some(253), None, 0)
    ],
    [ // 250
        (Some(251), None, 0),
        (Some(252), None, 0)
    ],
    [ // 251
        (None, Some(245), 0),
        (None, Some(246), 0)
    ],
    [ // 252
        (None, Some(247), 0),
        (None, Some(248), 0)
    ],
    [ // 253
        (Some(254), None, 0),
        (Some(255), None, 0)
    ],
    [ // 254
        (None, Some(250), 0),
        (None, Some(251), 0)
    ],
    [ // 255
        (None, Some(252), 0),
        (None, Some(253), 0)
    ]
];
