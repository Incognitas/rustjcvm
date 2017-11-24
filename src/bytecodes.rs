use jcvmerrors::InterpreterError;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum bytecode {
    nop = 0x00, // 0
    aconst_null, // 1
    sconst_m1, // 2
    sconst_0, // 3
    sconst_1, // 4
    sconst_2, // 5
    sconst_3, // 6
    sconst_4, // 7
    sconst_5, // 8
    iconst_m1, // 9
    iconst_0, // 10 - 0x0A
    iconst_1, // 11
    iconst_2, // 12
    iconst_3, // 13
    iconst_4, // 14
    iconst_5, // 15
    bspush, // 16
    sspush, // 17
    bipush, // 18
    sipush, // 19
    iipush, // 20
    aload, // 21
    sload, // 22
    iload, // 23
    aload_0, // 24
    aload_1, // 25
    aload_2, // 26
    aload_3, // 27
    sload_0, // 28
    sload_1, // 29
    sload_2, // 30
    sload_3, // 31
    iload_0, // 32
    iload_1, // 33
    iload_2, // 34
    iload_3, // 35
    aaload, // 36
    baload, // 37
    saload, // 38
    iaload, // 39
    astore, // 40
    sstore, // 41
    istore, // 42
    astore_0, // 43
    astore_1, // 44
    astore_2, // 45
    astore_3, // 46
    sstore_0, // 47
    sstore_1, // 48
    sstore_2, // 49
    sstore_3, // 50
    istore_0, // 51
    istore_1, // 52
    istore_2, // 53
    istore_3, // 54
    aastore, // 55
    bastore, // 56
    sastore, // 57
    iastore, // 58
    pop, // 59
    pop2, // 60
    dup, // 61
    dup2, // 62
    dup_x, // 63
    swap_x, // 64
    sadd, // 65
    iadd, // 66
    ssub, // 67
    isub, // 68
    smul, // 69
    imul, // 70
    sdiv, // 71
    idiv, // 72
    srem, // 73
    irem, // 74
    sneg, // 75
    ineg, // 76
    sshl, // 77
    ishl, // 78
    sshr, // 79
    ishr, // 80
    sushr, // 81
    iushr, // 82
    sand, // 83
    iand, // 84
    sor, // 85
    ior, // 86
    sxor, // 87
    ixor, // 88
    sinc, // 89
    iinc, // 90
    s2b, // 91
    s2i, // 92
    i2b, // 93
    i2s, // 94
    icmp, // 95
    ifeq, // 96
    ifne, // 97
    iflt, // 98
    ifge, // 99
    ifgt, // 100
    ifle, // 101
    ifnull, // 102
    ifnonnull, // 103
    if_acmpeq, // 104
    if_acmpne, // 105
    if_scmpeq, // 106
    if_scmpne, // 107
    if_scmplt, // 108
    if_scmpge, // 109
    if_scmpgt, // 110
    if_scmple, // 111
    goto, // 112
    jsr, // 113
    ret, // 114
    stableswitch, // 115
    itableswitch, // 116
    slookupswitch, // 117
    ilookupswitch, // 118
    areturn, // 119
    sreturn, // 120
    ireturn, // 121
    return_, // 122
    getstatic_a, // 123
    getstatic_b, // 124
    getstatic_s, // 125
    getstatic_i, // 126
    putstatic_a, // 127
    putstatic_b, // 128
    putstatic_s, // 129
    putstatic_i, // 130
    getfield_a, // 131
    getfield_b, // 132
    getfield_s, // 133
    getfield_i, // 134
    putfield_a, // 135
    putfield_b, // 136
    putfield_s, // 137
    putfield_i, // 138
    invokevirtual, // 139
    invokespecial, // 140
    invokestatic, // 141
    invokeinterface, // 142
    new, // 143
    newarray, // 144
    anewarray, // 145
    arraylength, // 146
    athrow, // 147
    checkcast, // 148
    instanceof, // 149
    sinc_w, // 150
    iinc_w, // 151
    ifeq_w, // 152
    ifne_w, // 153
    iflt_w, // 154
    ifge_w, // 155
    ifgt_w, // 156
    ifle_w, // 157
    ifnull_w, // 158
    ifnonnull_w, // 159
    if_acmpeq_w, // 160
    if_acmpne_w, // 161
    if_scmpeq_w, // 162
    if_scmpne_w, // 163
    if_scmplt_w, // 164
    if_scmpge_w, // 165
    if_scmpgt_w, // 166
    if_scmple_w, // 167
    goto_w, // 168
    getfield_a_w, // 169
    getfield_b_w, // 170
    getfield_s_w, // 171
    getfield_i_w, // 172
    getfield_a_this, // 173
    getfield_b_this, // 174
    getfield_s_this, // 175
    getfield_i_this, // 176
    putfield_a_w, // 177
    putfield_b_w, // 178
    putfield_s_w, // 179
    putfield_i_w, // 180
    putfield_a_this, // 181
    putfield_b_this, // 182
    putfield_s_this, // 183
    putfield_i_this, // 184
    END, // indicator for the end of standard opcodes
    impdep1, // 254
    impdep2, // 255
}

impl bytecode {
    pub fn from(b: u8) -> Result<bytecode, InterpreterError> {
        match b {
            0 => Ok(bytecode::nop),
            1 => Ok(bytecode::aconst_null),
            2 => Ok(bytecode::sconst_m1),
            3 => Ok(bytecode::sconst_0),
            4 => Ok(bytecode::sconst_1),
            5 => Ok(bytecode::sconst_2),
            6 => Ok(bytecode::sconst_3),
            7 => Ok(bytecode::sconst_4),
            8 => Ok(bytecode::sconst_5),
            9 => Ok(bytecode::iconst_m1),
            10 => Ok(bytecode::iconst_0),
            11 => Ok(bytecode::iconst_1),
            12 => Ok(bytecode::iconst_2),
            13 => Ok(bytecode::iconst_3),
            14 => Ok(bytecode::iconst_4),
            15 => Ok(bytecode::iconst_5),
            16 => Ok(bytecode::bspush),
            17 => Ok(bytecode::sspush),
            18 => Ok(bytecode::bipush),
            19 => Ok(bytecode::sipush),
            20 => Ok(bytecode::iipush),
            21 => Ok(bytecode::aload),
            22 => Ok(bytecode::sload),
            23 => Ok(bytecode::iload),
            24 => Ok(bytecode::aload_0),
            25 => Ok(bytecode::aload_1),
            26 => Ok(bytecode::aload_2),
            27 => Ok(bytecode::aload_3),
            28 => Ok(bytecode::sload_0),
            29 => Ok(bytecode::sload_1),
            30 => Ok(bytecode::sload_2),
            31 => Ok(bytecode::sload_3),
            32 => Ok(bytecode::iload_0),
            33 => Ok(bytecode::iload_1),
            34 => Ok(bytecode::iload_2),
            35 => Ok(bytecode::iload_3),
            36 => Ok(bytecode::aaload),
            37 => Ok(bytecode::baload),
            38 => Ok(bytecode::saload),
            39 => Ok(bytecode::iaload),
            40 => Ok(bytecode::astore),
            41 => Ok(bytecode::sstore),
            42 => Ok(bytecode::istore),
            43 => Ok(bytecode::astore_0),
            44 => Ok(bytecode::astore_1),
            45 => Ok(bytecode::astore_2),
            46 => Ok(bytecode::astore_3),
            47 => Ok(bytecode::sstore_0),
            48 => Ok(bytecode::sstore_1),
            49 => Ok(bytecode::sstore_2),
            50 => Ok(bytecode::sstore_3),
            51 => Ok(bytecode::istore_0),
            52 => Ok(bytecode::istore_1),
            53 => Ok(bytecode::istore_2),
            54 => Ok(bytecode::istore_3),
            55 => Ok(bytecode::aastore),
            56 => Ok(bytecode::bastore),
            57 => Ok(bytecode::sastore),
            58 => Ok(bytecode::iastore),
            59 => Ok(bytecode::pop),
            60 => Ok(bytecode::pop2),
            61 => Ok(bytecode::dup),
            62 => Ok(bytecode::dup2),
            63 => Ok(bytecode::dup_x),
            64 => Ok(bytecode::swap_x),
            65 => Ok(bytecode::sadd),
            66 => Ok(bytecode::iadd),
            67 => Ok(bytecode::ssub),
            68 => Ok(bytecode::isub),
            69 => Ok(bytecode::smul),
            70 => Ok(bytecode::imul),
            71 => Ok(bytecode::sdiv),
            72 => Ok(bytecode::idiv),
            73 => Ok(bytecode::srem),
            74 => Ok(bytecode::irem),
            75 => Ok(bytecode::sneg),
            76 => Ok(bytecode::ineg),
            77 => Ok(bytecode::sshl),
            78 => Ok(bytecode::ishl),
            79 => Ok(bytecode::sshr),
            80 => Ok(bytecode::ishr),
            81 => Ok(bytecode::sushr),
            82 => Ok(bytecode::iushr),
            83 => Ok(bytecode::sand),
            84 => Ok(bytecode::iand),
            85 => Ok(bytecode::sor),
            86 => Ok(bytecode::ior),
            87 => Ok(bytecode::sxor),
            88 => Ok(bytecode::ixor),
            89 => Ok(bytecode::sinc),
            90 => Ok(bytecode::iinc),
            91 => Ok(bytecode::s2b),
            92 => Ok(bytecode::s2i),
            93 => Ok(bytecode::i2b),
            94 => Ok(bytecode::i2s),
            95 => Ok(bytecode::icmp),
            96 => Ok(bytecode::ifeq),
            97 => Ok(bytecode::ifne),
            98 => Ok(bytecode::iflt),
            99 => Ok(bytecode::ifge),
            100 => Ok(bytecode::ifgt),
            101 => Ok(bytecode::ifle),
            102 => Ok(bytecode::ifnull),
            103 => Ok(bytecode::ifnonnull),
            104 => Ok(bytecode::if_acmpeq),
            105 => Ok(bytecode::if_acmpne),
            106 => Ok(bytecode::if_scmpeq),
            107 => Ok(bytecode::if_scmpne),
            108 => Ok(bytecode::if_scmplt),
            109 => Ok(bytecode::if_scmpge),
            110 => Ok(bytecode::if_scmpgt),
            111 => Ok(bytecode::if_scmple),
            112 => Ok(bytecode::goto),
            113 => Ok(bytecode::jsr),
            114 => Ok(bytecode::ret),
            115 => Ok(bytecode::stableswitch),
            116 => Ok(bytecode::itableswitch),
            117 => Ok(bytecode::slookupswitch),
            118 => Ok(bytecode::ilookupswitch),
            119 => Ok(bytecode::areturn),
            120 => Ok(bytecode::sreturn),
            121 => Ok(bytecode::ireturn),
            122 => Ok(bytecode::return_),
            123 => Ok(bytecode::getstatic_a),
            124 => Ok(bytecode::getstatic_b),
            125 => Ok(bytecode::getstatic_s),
            126 => Ok(bytecode::getstatic_i),
            127 => Ok(bytecode::putstatic_a),
            128 => Ok(bytecode::putstatic_b),
            129 => Ok(bytecode::putstatic_s),
            130 => Ok(bytecode::putstatic_i),
            131 => Ok(bytecode::getfield_a),
            132 => Ok(bytecode::getfield_b),
            133 => Ok(bytecode::getfield_s),
            134 => Ok(bytecode::getfield_i),
            135 => Ok(bytecode::putfield_a),
            136 => Ok(bytecode::putfield_b),
            137 => Ok(bytecode::putfield_s),
            138 => Ok(bytecode::putfield_i),
            139 => Ok(bytecode::invokevirtual),
            140 => Ok(bytecode::invokespecial),
            141 => Ok(bytecode::invokestatic),
            142 => Ok(bytecode::invokeinterface),
            143 => Ok(bytecode::new),
            144 => Ok(bytecode::newarray),
            145 => Ok(bytecode::anewarray),
            146 => Ok(bytecode::arraylength),
            147 => Ok(bytecode::athrow),
            148 => Ok(bytecode::checkcast),
            149 => Ok(bytecode::instanceof),
            150 => Ok(bytecode::sinc_w),
            151 => Ok(bytecode::iinc_w),
            152 => Ok(bytecode::ifeq_w),
            153 => Ok(bytecode::ifne_w),
            154 => Ok(bytecode::iflt_w),
            155 => Ok(bytecode::ifge_w),
            156 => Ok(bytecode::ifgt_w),
            157 => Ok(bytecode::ifle_w),
            158 => Ok(bytecode::ifnull_w),
            159 => Ok(bytecode::ifnonnull_w),
            160 => Ok(bytecode::if_acmpeq_w),
            161 => Ok(bytecode::if_acmpne_w),
            162 => Ok(bytecode::if_scmpeq_w),
            163 => Ok(bytecode::if_scmpne_w),
            164 => Ok(bytecode::if_scmplt_w),
            165 => Ok(bytecode::if_scmpge_w),
            166 => Ok(bytecode::if_scmpgt_w),
            167 => Ok(bytecode::if_scmple_w),
            168 => Ok(bytecode::goto_w),
            169 => Ok(bytecode::getfield_a_w),
            170 => Ok(bytecode::getfield_b_w),
            171 => Ok(bytecode::getfield_s_w),
            172 => Ok(bytecode::getfield_i_w),
            173 => Ok(bytecode::getfield_a_this),
            174 => Ok(bytecode::getfield_b_this),
            175 => Ok(bytecode::getfield_s_this),
            176 => Ok(bytecode::getfield_i_this),
            177 => Ok(bytecode::putfield_a_w),
            178 => Ok(bytecode::putfield_b_w),
            179 => Ok(bytecode::putfield_s_w),
            180 => Ok(bytecode::putfield_i_w),
            181 => Ok(bytecode::putfield_a_this),
            182 => Ok(bytecode::putfield_b_this),
            183 => Ok(bytecode::putfield_s_this),
            184 => Ok(bytecode::putfield_i_this),
            185 => Ok(bytecode::END),
            186 => Ok(bytecode::impdep1),
            // 186 => bytecode::impdep1, // 254
            // 187 => bytecode::impdep2,
            _ => Err(InterpreterError::UnrecognizedBytecode),
        }
    }
}
