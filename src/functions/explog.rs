use hexf::hexf64;

use crate::{consts::LN_2, TwoFloat};

// 1/ln(2)
const FRAC_1_LN_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.71547652b82fep0"),
    lo: hexf64!("0x1.777d0ffda0d24p-56"),
};

// ln(10)
const LN_10: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.26bb1bbb55516p1"),
    lo: hexf64!("-0x1.f48ad494ea3e9p-53"),
};

// ln(3/2)
const LN_FRAC_3_2: TwoFloat = TwoFloat {
    hi: hexf64!("0x1.9f323ecbf984cp-2"),
    lo: hexf64!("-0x1.a92e513217f5cp-59"),
};

// limits
const EXP_UPPER_LIMIT: f64 = 709.0;
const EXP_LOWER_LIMIT: f64 = -709.0;

const FRAC_FACT: [TwoFloat; 21] = [
    TwoFloat {
        // 1/0!
        hi: hexf64!("0x1.0000000000000p+0"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/1!
        hi: hexf64!("0x1.0000000000000p+0"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/2!
        hi: hexf64!("0x1.0000000000000p-1"),
        lo: hexf64!("0x0.0p+0"),
    },
    TwoFloat {
        // 1/3!
        hi: hexf64!("0x1.5555555555555p-3"),
        lo: hexf64!("0x1.5555555555555p-57"),
    },
    TwoFloat {
        // 1/4!
        hi: hexf64!("0x1.5555555555555p-5"),
        lo: hexf64!("0x1.5555555555555p-59"),
    },
    TwoFloat {
        // 1/5!
        hi: hexf64!("0x1.1111111111111p-7"),
        lo: hexf64!("0x1.1111111111111p-63"),
    },
    TwoFloat {
        // 1/6!
        hi: hexf64!("0x1.6c16c16c16c17p-10"),
        lo: hexf64!("-0x1.f49f49f49f49fp-65"),
    },
    TwoFloat {
        // 1/7!
        hi: hexf64!("0x1.a01a01a01a01ap-13"),
        lo: hexf64!("0x1.a01a01a01a01ap-73"),
    },
    TwoFloat {
        // 1/8!
        hi: hexf64!("0x1.a01a01a01a01ap-16"),
        lo: hexf64!("0x1.a01a01a01a01ap-76"),
    },
    TwoFloat {
        // 1/9!
        hi: hexf64!("0x1.71de3a556c734p-19"),
        lo: hexf64!("-0x1.c154f8ddc6c00p-73"),
    },
    TwoFloat {
        // 1/10!
        hi: hexf64!("0x1.27e4fb7789f5cp-22"),
        lo: hexf64!("0x1.cbbc05b4fa99ap-76"),
    },
    TwoFloat {
        // 1/11!
        hi: hexf64!("0x1.ae64567f544e4p-26"),
        lo: hexf64!("-0x1.c062e06d1f209p-80"),
    },
    TwoFloat {
        // 1/12!
        hi: hexf64!("0x1.1eed8eff8d898p-29"),
        lo: hexf64!("-0x1.2aec959e14c06p-83"),
    },
    TwoFloat {
        // 1/13!
        hi: hexf64!("0x1.6124613a86d09p-33"),
        lo: hexf64!("0x1.f28e0cc748ebep-87"),
    },
    TwoFloat {
        // 1/14!
        hi: hexf64!("0x1.93974a8c07c9dp-37"),
        lo: hexf64!("0x1.05d6f8a2efd1fp-92"),
    },
    TwoFloat {
        // 1/15!
        hi: hexf64!("0x1.ae7f3e733b81fp-41"),
        lo: hexf64!("0x1.1d8656b0ee8cbp-97"),
    },
    TwoFloat {
        // 1/16!
        hi: hexf64!("0x1.ae7f3e733b81fp-45"),
        lo: hexf64!("0x1.1d8656b0ee8cbp-101"),
    },
    TwoFloat {
        // 1/17!
        hi: hexf64!("0x1.952c77030ad4ap-49"),
        lo: hexf64!("0x1.ac981465ddc6cp-103"),
    },
    TwoFloat {
        // 1/18!
        hi: hexf64!("0x1.6827863b97d97p-53"),
        lo: hexf64!("0x1.eec01221a8b0bp-107"),
    },
    TwoFloat {
        // 1/19!
        hi: hexf64!("0x1.2f49b46814157p-57"),
        lo: hexf64!("0x1.2650f61dbdcb4p-112"),
    },
    TwoFloat {
        // 1/20!
        hi: hexf64!("0x1.e542ba4020225p-62"),
        lo: hexf64!("0x1.ea72b4afe3c2fp-120"),
    },
];

fn mul_pow2(mut x: f64, mut y: i32) -> f64 {
    loop {
        if y < -1074 {
            x *= hexf64!("0x1.0p-1074");
            y += 1074;
        } else if y < -1022 {
            return x * f64::from_bits(1u64 << (y + 1074));
        } else if y < 1024 {
            return x * f64::from_bits(((y + 1023) as u64) << 52);
        } else {
            x *= hexf64!("0x1.0p1023");
            y -= 1023;
        }
    }
}

/// Exact results for the expression `exp(n/128) - 1` for `|n| <= 32`
fn expm1_128th(n: i32) -> TwoFloat {
    assert!(n.abs() <= 32);

    const EXPM1_128TH: [TwoFloat; 65] = [
        TwoFloat {
            // exp(-32/128) - 1
            hi: hexf64!("-0x1.c5041854df7d4p-3"),
            lo: hexf64!("-0x1.797d4686c5393p-57"),
        },
        TwoFloat {
            // exp(-31/128) - 1
            hi: hexf64!("-0x1.b881a23aebb4ap-3"),
            lo: hexf64!("0x1.5e3462e9ccc6ep-59"),
        },
        TwoFloat {
            // exp(-30/128) - 1
            hi: hexf64!("-0x1.abe60e1f21836p-3"),
            lo: hexf64!("-0x1.6f8b82e653e2dp-60"),
        },
        TwoFloat {
            // exp(-29/128) - 1
            hi: hexf64!("-0x1.9f3129931faafp-3"),
            lo: hexf64!("-0x1.00136f85b612cp-59"),
        },
        TwoFloat {
            // exp(-28/128) - 1
            hi: hexf64!("-0x1.9262c1c3430a1p-3"),
            lo: hexf64!("-0x1.46ff6ec4a4251p-57"),
        },
        TwoFloat {
            // exp(-27/128) - 1
            hi: hexf64!("-0x1.857aa375db4e2p-3"),
            lo: hexf64!("-0x1.960d6ed0eefd4p-58"),
        },
        TwoFloat {
            // exp(-26/128) - 1
            hi: hexf64!("-0x1.78789b0a5e0c0p-3"),
            lo: hexf64!("0x1.e3a6bdaece8f9p-58"),
        },
        TwoFloat {
            // exp(-25/128) - 1
            hi: hexf64!("-0x1.6b5c7478983dap-3"),
            lo: hexf64!("0x1.286a8f9e96160p-58"),
        },
        TwoFloat {
            // exp(-24/128) - 1
            hi: hexf64!("-0x1.5e25fb4fde211p-3"),
            lo: hexf64!("0x1.64eec82915df3p-63"),
        },
        TwoFloat {
            // exp(-23/128) - 1
            hi: hexf64!("-0x1.50d4fab639757p-3"),
            lo: hexf64!("-0x1.3bc197e5f2a7ep-59"),
        },
        TwoFloat {
            // exp(-22/128) - 1
            hi: hexf64!("-0x1.43693d679612dp-3"),
            lo: hexf64!("-0x1.9da94a869862ap-57"),
        },
        TwoFloat {
            // exp(-21/128) - 1
            hi: hexf64!("-0x1.35e28db4ecd9bp-3"),
            lo: hexf64!("-0x1.a2252f7d4b5f6p-58"),
        },
        TwoFloat {
            // exp(-20/128) - 1
            hi: hexf64!("-0x1.2840b5836cf67p-3"),
            lo: hexf64!("-0x1.85405051eb425p-57"),
        },
        TwoFloat {
            // exp(-19/128) - 1
            hi: hexf64!("-0x1.1a837e4ba3760p-3"),
            lo: hexf64!("0x1.a94ad2c8fa0bfp-58"),
        },
        TwoFloat {
            // exp(-18/128) - 1
            hi: hexf64!("-0x1.0caab118a1278p-3"),
            lo: hexf64!("0x1.6ad4c353465b0p-61"),
        },
        TwoFloat {
            // exp(-17/128) - 1
            hi: hexf64!("-0x1.fd6c2d0e3d912p-4"),
            lo: hexf64!("0x1.d117a3c69926cp-58"),
        },
        TwoFloat {
            // exp(-16/128) - 1
            hi: hexf64!("-0x1.e14aed893eef4p-4"),
            lo: hexf64!("0x1.e1f58934f97afp-59"),
        },
        TwoFloat {
            // exp(-15/128) - 1
            hi: hexf64!("-0x1.c4f1331d22d3cp-4"),
            lo: hexf64!("-0x1.ece0aa18a07e5p-63"),
        },
        TwoFloat {
            // exp(-14/128) - 1
            hi: hexf64!("-0x1.a85e8c62d9c13p-4"),
            lo: hexf64!("-0x1.adf7745e77188p-58"),
        },
        TwoFloat {
            // exp(-13/128) - 1
            hi: hexf64!("-0x1.8b92870fa2b59p-4"),
            lo: hexf64!("-0x1.ffa6c0b097a6bp-58"),
        },
        TwoFloat {
            // exp(-12/128) - 1
            hi: hexf64!("-0x1.6e8caff341feap-4"),
            lo: hexf64!("-0x1.9573ded7888b2p-58"),
        },
        TwoFloat {
            // exp(-11/128) - 1
            hi: hexf64!("-0x1.514c92f634786p-4"),
            lo: hexf64!("-0x1.64c069cd0a314p-58"),
        },
        TwoFloat {
            // exp(-10/128) - 1
            hi: hexf64!("-0x1.33d1bb17df2e7p-4"),
            lo: hexf64!("-0x1.e19c873b1d6a8p-59"),
        },
        TwoFloat {
            // exp(-9/128) - 1
            hi: hexf64!("-0x1.161bb26cbb590p-4"),
            lo: hexf64!("-0x1.589321a7ef10bp-60"),
        },
        TwoFloat {
            // exp(-8/128) - 1
            hi: hexf64!("-0x1.f0540438fd5c3p-5"),
            lo: hexf64!("-0x1.a1ce01f9f6ca7p-61"),
        },
        TwoFloat {
            // exp(-7/128) - 1
            hi: hexf64!("-0x1.b3f864c07fffbp-5"),
            lo: hexf64!("0x1.cfbc1f5774ea7p-61"),
        },
        TwoFloat {
            // exp(-6/128) - 1
            hi: hexf64!("-0x1.7723950130405p-5"),
            lo: hexf64!("0x1.c677ad8fa478dp-61"),
        },
        TwoFloat {
            // exp(-5/128) - 1
            hi: hexf64!("-0x1.39d4a1a77e051p-5"),
            lo: hexf64!("0x1.ee8939ec858d8p-59"),
        },
        TwoFloat {
            // exp(-4/128) - 1
            hi: hexf64!("-0x1.f8152aee9450ep-6"),
            lo: hexf64!("0x1.4b00abf977627p-61"),
        },
        TwoFloat {
            // exp(-3/128) - 1
            hi: hexf64!("-0x1.7b88f290230dep-6"),
            lo: hexf64!("0x1.e93d61cf69296p-60"),
        },
        TwoFloat {
            // exp(-2/128) - 1
            hi: hexf64!("-0x1.fc055004416dbp-7"),
            lo: hexf64!("-0x1.82ef422ab152ap-61"),
        },
        TwoFloat {
            // exp(-1/128) - 1
            hi: hexf64!("-0x1.fe0154aaeed83p-8"),
            lo: hexf64!("-0x1.00681d99aceefp-62"),
        },
        TwoFloat {
            // exp(0/128) - 1
            hi: hexf64!("0x0.0p+0"),
            lo: hexf64!("0x0.0p+0"),
        },
        TwoFloat {
            // exp(1/128) - 1
            hi: hexf64!("0x1.0100ab00222d8p-7"),
            lo: hexf64!("0x1.864c70578e6d1p-61"),
        },
        TwoFloat {
            // exp(2/128) - 1
            hi: hexf64!("0x1.0202ad5778e46p-6"),
            lo: hexf64!("-0x1.51e6d305beec6p-62"),
        },
        TwoFloat {
            // exp(3/128) - 1
            hi: hexf64!("0x1.84890d9043745p-6"),
            lo: hexf64!("0x1.cacb3aebd2b6fp-61"),
        },
        TwoFloat {
            // exp(4/128) - 1
            hi: hexf64!("0x1.040ac0224fd93p-5"),
            lo: hexf64!("0x1.c17a107575019p-61"),
        },
        TwoFloat {
            // exp(5/128) - 1
            hi: hexf64!("0x1.465509d383eb0p-5"),
            lo: hexf64!("0x1.45cc1cf959b1bp-60"),
        },
        TwoFloat {
            // exp(6/128) - 1
            hi: hexf64!("0x1.89246d053d178p-5"),
            lo: hexf64!("0x1.4967f31eb2595p-59"),
        },
        TwoFloat {
            // exp(7/128) - 1
            hi: hexf64!("0x1.cc79f4f5613a3p-5"),
            lo: hexf64!("-0x1.9b7d9052797c8p-61"),
        },
        TwoFloat {
            // exp(8/128) - 1
            hi: hexf64!("0x1.082b577d34ed8p-4"),
            lo: hexf64!("-0x1.5272ff30eed1bp-59"),
        },
        TwoFloat {
            // exp(9/128) - 1
            hi: hexf64!("0x1.2a5dd543ccc4ep-4"),
            lo: hexf64!("-0x1.280f19dace1bep-59"),
        },
        TwoFloat {
            // exp(10/128) - 1
            hi: hexf64!("0x1.4cd4fc989cd64p-4"),
            lo: hexf64!("0x1.557a8671b89e7p-58"),
        },
        TwoFloat {
            // exp(11/128) - 1
            hi: hexf64!("0x1.6f91575870693p-4"),
            lo: hexf64!("-0x1.b71235569f4d4p-61"),
        },
        TwoFloat {
            // exp(12/128) - 1
            hi: hexf64!("0x1.92937074e0cd7p-4"),
            lo: hexf64!("-0x1.db0b9cc915fc5p-58"),
        },
        TwoFloat {
            // exp(13/128) - 1
            hi: hexf64!("0x1.b5dbd3f681223p-4"),
            lo: hexf64!("0x1.f5c92a5200eeep-63"),
        },
        TwoFloat {
            // exp(14/128) - 1
            hi: hexf64!("0x1.d96b0eff0e794p-4"),
            lo: hexf64!("-0x1.75385b2cdf93dp-59"),
        },
        TwoFloat {
            // exp(15/128) - 1
            hi: hexf64!("0x1.fd41afcba45e7p-4"),
            lo: hexf64!("-0x1.2db6f4bbe33b4p-60"),
        },
        TwoFloat {
            // exp(16/128) - 1
            hi: hexf64!("0x1.10b022db7ae68p-3"),
            lo: hexf64!("-0x1.8c4a5df1ec7e5p-58"),
        },
        TwoFloat {
            // exp(17/128) - 1
            hi: hexf64!("0x1.22e3b09dc54d8p-3"),
            lo: hexf64!("-0x1.bd4b1c37ea8a2p-57"),
        },
        TwoFloat {
            // exp(18/128) - 1
            hi: hexf64!("0x1.353bc9fb00b21p-3"),
            lo: hexf64!("0x1.6bae618011342p-57"),
        },
        TwoFloat {
            // exp(19/128) - 1
            hi: hexf64!("0x1.47b8b853aafecp-3"),
            lo: hexf64!("-0x1.4c26602c63fdap-57"),
        },
        TwoFloat {
            // exp(20/128) - 1
            hi: hexf64!("0x1.5a5ac59b963cbp-3"),
            lo: hexf64!("-0x1.fd91307e74c50p-57"),
        },
        TwoFloat {
            // exp(21/128) - 1
            hi: hexf64!("0x1.6d223c5b1063ap-3"),
            lo: hexf64!("-0x1.4aae273c07a5ep-60"),
        },
        TwoFloat {
            // exp(22/128) - 1
            hi: hexf64!("0x1.800f67b00d7b8p-3"),
            lo: hexf64!("0x1.7ab912c69ffebp-61"),
        },
        TwoFloat {
            // exp(23/128) - 1
            hi: hexf64!("0x1.9322934f54148p-3"),
            lo: hexf64!("-0x1.b3564bc0ec9cdp-58"),
        },
        TwoFloat {
            // exp(24/128) - 1
            hi: hexf64!("0x1.a65c0b85ac1a9p-3"),
            lo: hexf64!("0x1.a9c189196f8cdp-57"),
        },
        TwoFloat {
            // exp(25/128) - 1
            hi: hexf64!("0x1.b9bc1d3910092p-3"),
            lo: hexf64!("0x1.ea39cb4039031p-57"),
        },
        TwoFloat {
            // exp(26/128) - 1
            hi: hexf64!("0x1.cd4315e9e0833p-3"),
            lo: hexf64!("-0x1.172c31a1781f1p-61"),
        },
        TwoFloat {
            // exp(27/128) - 1
            hi: hexf64!("0x1.e0f143b41a554p-3"),
            lo: hexf64!("-0x1.6e7fb859d5055p-62"),
        },
        TwoFloat {
            // exp(28/128) - 1
            hi: hexf64!("0x1.f4c6f5508ee5dp-3"),
            lo: hexf64!("0x1.46ef7b808180ap-57"),
        },
        TwoFloat {
            // exp(29/128) - 1
            hi: hexf64!("0x1.04623d0b0f8c8p-2"),
            lo: hexf64!("0x1.e17611afc42c5p-57"),
        },
        TwoFloat {
            // exp(30/128) - 1
            hi: hexf64!("0x1.0e7510fd7c564p-2"),
            lo: hexf64!("-0x1.1c5b2e8735a43p-56"),
        },
        TwoFloat {
            // exp(31/128) - 1
            hi: hexf64!("0x1.189c1ecaeb083p-2"),
            lo: hexf64!("0x1.b403d8c766006p-56"),
        },
        TwoFloat {
            // exp(32/128) - 1
            hi: hexf64!("0x1.22d78f0fa061ap-2"),
            lo: hexf64!("-0x1.89843c4964554p-56"),
        },
    ];

    EXPM1_128TH[(n + 32) as usize]
}

/// Compute exp(1/2)^n with n = 32*a + b
///
/// exp(1/2)^n = exp(32/2)^a * exp(1/2)^b
///
/// In order to have the exact result we can store the coefficeints
/// for different values of `a` and `b`
///
/// with :
///  - `|a| < 45`
///  - `|b| < 32`
///
/// We obtain valid expression for `|n| < 32 * 45 = 1440`
fn exp_half(n: i32) -> TwoFloat {
    assert!(n < 1440, "exp_half max exponent is 1439: {}", n);

    const EXP_HALF_N: [TwoFloat; 31] = [
        TwoFloat {
            // exp(1/2)^1
            hi: hexf64!("0x1.a61298e1e069cp+0"),
            lo: hexf64!("-0x1.b4690082a4906p-55"),
        },
        TwoFloat {
            // exp(1/2)^2
            hi: hexf64!("0x1.5bf0a8b145769p+1"),
            lo: hexf64!("0x1.4d57ee2b1013ap-53"),
        },
        TwoFloat {
            // exp(1/2)^3
            hi: hexf64!("0x1.1ed3fe64fc541p+2"),
            lo: hexf64!("0x1.5f6e4658d43eap-52"),
        },
        TwoFloat {
            // exp(1/2)^4
            hi: hexf64!("0x1.d8e64b8d4ddaep+2"),
            lo: hexf64!("-0x1.9e62e22efca4cp-53"),
        },
        TwoFloat {
            // exp(1/2)^5
            hi: hexf64!("0x1.85d6fd931e0bbp+3"),
            lo: hexf64!("0x1.d4dec34de84a0p-53"),
        },
        TwoFloat {
            // exp(1/2)^6
            hi: hexf64!("0x1.415e5bf6fb106p+4"),
            lo: hexf64!("-0x1.a568407591768p-53"),
        },
        TwoFloat {
            // exp(1/2)^7
            hi: hexf64!("0x1.08ec721396bdbp+5"),
            lo: hexf64!("0x1.4354c26b2875ep-49"),
        },
        TwoFloat {
            // exp(1/2)^8
            hi: hexf64!("0x1.b4c902e273a58p+5"),
            lo: hexf64!("0x1.9e35b4eff6e4fp-49"),
        },
        TwoFloat {
            // exp(1/2)^9
            hi: hexf64!("0x1.68118ade1deaap+6"),
            lo: hexf64!("0x1.6f9d8bafaba87p-49"),
        },
        TwoFloat {
            // exp(1/2)^10
            hi: hexf64!("0x1.28d389970338fp+7"),
            lo: hexf64!("0x1.f66faad9235acp-49"),
        },
        TwoFloat {
            // exp(1/2)^11
            hi: hexf64!("0x1.e96244f21bbf6p+7"),
            lo: hexf64!("0x1.298c834010b39p-48"),
        },
        TwoFloat {
            // exp(1/2)^12
            hi: hexf64!("0x1.936dc5690c08fp+8"),
            lo: hexf64!("0x1.bd4d728fcb999p-47"),
        },
        TwoFloat {
            // exp(1/2)^13
            hi: hexf64!("0x1.4c92210816c89p+9"),
            lo: hexf64!("0x1.0d5b86bb2485cp-45"),
        },
        TwoFloat {
            // exp(1/2)^14
            hi: hexf64!("0x1.122885aaeddaap+10"),
            lo: hexf64!("0x1.bc7e802a24decp-44"),
        },
        TwoFloat {
            // exp(1/2)^15
            hi: hexf64!("0x1.c402b6eb1f6adp+10"),
            lo: hexf64!("0x1.49c5fd40401f0p-45"),
        },
        TwoFloat {
            // exp(1/2)^16
            hi: hexf64!("0x1.749ea7d470c6ep+11"),
            lo: hexf64!("-0x1.e83fe3ef6afd4p-46"),
        },
        TwoFloat {
            // exp(1/2)^17
            hi: hexf64!("0x1.332c4d2b7c4a1p+12"),
            lo: hexf64!("0x1.877bfc89a825ep-46"),
        },
        TwoFloat {
            // exp(1/2)^18
            hi: hexf64!("0x1.fa7157c470f82p+12"),
            lo: hexf64!("-0x1.e4d50f21f5ac5p-43"),
        },
        TwoFloat {
            // exp(1/2)^19
            hi: hexf64!("0x1.a17dd08c11dc1p+13"),
            lo: hexf64!("-0x1.de54a23f86061p-41"),
        },
        TwoFloat {
            // exp(1/2)^20
            hi: hexf64!("0x1.5829dcf950560p+14"),
            lo: hexf64!("-0x1.83e055cfea4bbp-40"),
        },
        TwoFloat {
            // exp(1/2)^21
            hi: hexf64!("0x1.1bb7015e84d3bp+15"),
            lo: hexf64!("0x1.bc1c4193bcdb9p-40"),
        },
        TwoFloat {
            // exp(1/2)^22
            hi: hexf64!("0x1.d3c4488ee4f7fp+15"),
            lo: hexf64!("0x1.f7b8937dac77dp-40"),
        },
        TwoFloat {
            // exp(1/2)^23
            hi: hexf64!("0x1.819bc560f6113p+16"),
            lo: hexf64!("0x1.ab5fcbf2b9216p-39"),
        },
        TwoFloat {
            // exp(1/2)^24
            hi: hexf64!("0x1.3de1654d37c9ap+17"),
            lo: hexf64!("0x1.75002e232b908p-38"),
        },
        TwoFloat {
            // exp(1/2)^25
            hi: hexf64!("0x1.060c52565ba66p+18"),
            lo: hexf64!("-0x1.607621f784efep-36"),
        },
        TwoFloat {
            // exp(1/2)^26
            hi: hexf64!("0x1.b00b5916ac955p+18"),
            lo: hexf64!("0x1.aa63a6c655d68p-37"),
        },
        TwoFloat {
            // exp(1/2)^27
            hi: hexf64!("0x1.64290bd5cad8bp+19"),
            lo: hexf64!("0x1.c4da08cc70404p-35"),
        },
        TwoFloat {
            // exp(1/2)^28
            hi: hexf64!("0x1.259ac48bf05d7p+20"),
            lo: hexf64!("-0x1.07e45cbbee1cfp-36"),
        },
        TwoFloat {
            // exp(1/2)^29
            hi: hexf64!("0x1.e4127437732b7p+20"),
            lo: hexf64!("0x1.f4a21b9a4afd1p-36"),
        },
        TwoFloat {
            // exp(1/2)^30
            hi: hexf64!("0x1.8f0ccafad2a87p+21"),
            lo: hexf64!("-0x1.0e8d00e46995ap-35"),
        },
        TwoFloat {
            // exp(1/2)^31
            hi: hexf64!("0x1.48f609e7b6bbep+22"),
            lo: hexf64!("0x1.c297debcbca60p-32"),
        },
    ];
    const EXP_16_N: [TwoFloat; 44] = [
        TwoFloat {
            // exp(16)^1
            hi: hexf64!("0x1.0f2ebd0a80020p+23"),
            lo: hexf64!("0x1.2488fc5c220adp-31"),
        },
        TwoFloat {
            // exp(16)^2
            hi: hexf64!("0x1.1f43fcc4b662cp+46"),
            lo: hexf64!("0x1.f611e21006108p-8"),
        },
        TwoFloat {
            // exp(16)^3
            hi: hexf64!("0x1.304d6aeca254bp+69"),
            lo: hexf64!("0x1.d7a5e2eb149ebp+14"),
        },
        TwoFloat {
            // exp(16)^4
            hi: hexf64!("0x1.425982cf597cdp+92"),
            lo: hexf64!("0x1.02e71eada76d8p+37"),
        },
        TwoFloat {
            // exp(16)^5
            hi: hexf64!("0x1.55779b984f3ebp+115"),
            lo: hexf64!("0x1.e45281da54712p+60"),
        },
        TwoFloat {
            // exp(16)^6
            hi: hexf64!("0x1.69b7f55b808bap+138"),
            lo: hexf64!("0x1.6f21a89b844aep+83"),
        },
        TwoFloat {
            // exp(16)^7
            hi: hexf64!("0x1.7f2bc6e599b7ep+161"),
            lo: hexf64!("0x1.46d9358056eb4p+106"),
        },
        TwoFloat {
            // exp(16)^8
            hi: hexf64!("0x1.95e54c5dd4217p+184"),
            lo: hexf64!("0x1.fd4fd3548677cp+130"),
        },
        TwoFloat {
            // exp(16)^9
            hi: hexf64!("0x1.adf7d6c5fbb7ap+207"),
            lo: hexf64!("0x1.9ffe2ce2ba6cdp+153"),
        },
        TwoFloat {
            // exp(16)^10
            hi: hexf64!("0x1.c777dc65c9488p+230"),
            lo: hexf64!("0x1.d3ccd78123ba5p+174"),
        },
        TwoFloat {
            // exp(16)^11
            hi: hexf64!("0x1.e27b0a2f86833p+253"),
            lo: hexf64!("0x1.a7b6e2b8658a4p+198"),
        },
        TwoFloat {
            // exp(16)^12
            hi: hexf64!("0x1.ff18562cc483ep+276"),
            lo: hexf64!("-0x1.233168c763a20p+221"),
        },
        TwoFloat {
            // exp(16)^13
            hi: hexf64!("0x1.0eb40981671acp+300"),
            lo: hexf64!("0x1.29640bb156506p+245"),
        },
        TwoFloat {
            // exp(16)^14
            hi: hexf64!("0x1.1ec2024fb6cefp+323"),
            lo: hexf64!("-0x1.9422d9c2347aep+269"),
        },
        TwoFloat {
            // exp(16)^15
            hi: hexf64!("0x1.2fc3bb0fcb841p+346"),
            lo: hexf64!("0x1.db4ee23db7cd6p+292"),
        },
        TwoFloat {
            // exp(16)^16
            hi: hexf64!("0x1.41c7a8814bebap+369"),
            lo: hexf64!("0x1.c646601eeefb5p+312"),
        },
        TwoFloat {
            // exp(16)^17
            hi: hexf64!("0x1.54dd1adec0b48p+392"),
            lo: hexf64!("-0x1.2bfc53615f0fep+338"),
        },
        TwoFloat {
            // exp(16)^18
            hi: hexf64!("0x1.69144ae1d9f07p+415"),
            lo: hexf64!("-0x1.67d8bf9f3dd53p+360"),
        },
        TwoFloat {
            // exp(16)^19
            hi: hexf64!("0x1.7e7e678d54eb5p+438"),
            lo: hexf64!("0x1.63ae7d736556cp+384"),
        },
        TwoFloat {
            // exp(16)^20
            hi: hexf64!("0x1.952da4c83af00p+461"),
            lo: hexf64!("-0x1.aad695063fc20p+406"),
        },
        TwoFloat {
            // exp(16)^21
            hi: hexf64!("0x1.ad354ad6e368fp+484"),
            lo: hexf64!("-0x1.45d9910503235p+430"),
        },
        TwoFloat {
            // exp(16)^22
            hi: hexf64!("0x1.c6a9c6bee04c8p+507"),
            lo: hexf64!("0x1.d3ea517425318p+453"),
        },
        TwoFloat {
            // exp(16)^23
            hi: hexf64!("0x1.e1a0bba3c3728p+530"),
            lo: hexf64!("-0x1.c8f9ccce05e3ap+476"),
        },
        TwoFloat {
            // exp(16)^24
            hi: hexf64!("0x1.fe31152b7ef6bp+553"),
            lo: hexf64!("0x1.e0a8b9fec7ecep+497"),
        },
        TwoFloat {
            // exp(16)^25
            hi: hexf64!("0x1.0e398d7d01704p+577"),
            lo: hexf64!("-0x1.1b8649000ffedp+523"),
        },
        TwoFloat {
            // exp(16)^26
            hi: hexf64!("0x1.1e4042aa53cfdp+600"),
            lo: hexf64!("-0x1.73ca574593c91p+546"),
        },
        TwoFloat {
            // exp(16)^27
            hi: hexf64!("0x1.2f3a497f7830cp+623"),
            lo: hexf64!("0x1.ece7bf7bd12b2p+568"),
        },
        TwoFloat {
            // exp(16)^28
            hi: hexf64!("0x1.413610319d4ccp+646"),
            lo: hexf64!("-0x1.250de9eb0fea2p+592"),
        },
        TwoFloat {
            // exp(16)^29
            hi: hexf64!("0x1.5442e00d851d5p+669"),
            lo: hexf64!("0x1.83b70a56057f9p+613"),
        },
        TwoFloat {
            // exp(16)^30
            hi: hexf64!("0x1.6870ea75e682dp+692"),
            lo: hexf64!("-0x1.034729ffdd0d6p+636"),
        },
        TwoFloat {
            // exp(16)^31
            hi: hexf64!("0x1.7dd156a715f16p+715"),
            lo: hexf64!("0x1.3d26f623c56bcp+661"),
        },
        TwoFloat {
            // exp(16)^32
            hi: hexf64!("0x1.9476504ba852ep+738"),
            lo: hexf64!("0x1.b0272159f0071p+684"),
        },
        TwoFloat {
            // exp(16)^33
            hi: hexf64!("0x1.ac7316ee74ed5p+761"),
            lo: hexf64!("0x1.ec158afbf5767p+707"),
        },
        TwoFloat {
            // exp(16)^34
            hi: hexf64!("0x1.c5dc0e57174a2p+784"),
            lo: hexf64!("0x1.0813267d07b90p+725"),
        },
        TwoFloat {
            // exp(16)^35
            hi: hexf64!("0x1.e0c6cfded96e2p+807"),
            lo: hexf64!("-0x1.0a3b12a7dd3dep+752"),
        },
        TwoFloat {
            // exp(16)^36
            hi: hexf64!("0x1.fd4a3cccc1d98p+830"),
            lo: hexf64!("-0x1.4f3870d0b9535p+776"),
        },
        TwoFloat {
            // exp(16)^37
            hi: hexf64!("0x1.0dbf48e430396p+854"),
            lo: hexf64!("-0x1.31e09ef47fe9fp+799"),
        },
        TwoFloat {
            // exp(16)^38
            hi: hexf64!("0x1.1dbebdb9f1388p+877"),
            lo: hexf64!("-0x1.94b9c8dfe4ecdp+823"),
        },
        TwoFloat {
            // exp(16)^39
            hi: hexf64!("0x1.2eb1161f782b7p+900"),
            lo: hexf64!("0x1.858cc679c7d16p+843"),
        },
        TwoFloat {
            // exp(16)^40
            hi: hexf64!("0x1.40a4b9c27178ap+923"),
            lo: hexf64!("-0x1.41d437e9132c2p+869"),
        },
        TwoFloat {
            // exp(16)^41
            hi: hexf64!("0x1.53a8eb04faf7cp+946"),
            lo: hexf64!("0x1.7deea8bc3420dp+891"),
        },
        TwoFloat {
            // exp(16)^42
            hi: hexf64!("0x1.67cdd3f624846p+969"),
            lo: hexf64!("-0x1.b2c19b09043f9p+913"),
        },
        TwoFloat {
            // exp(16)^43
            hi: hexf64!("0x1.7d24940f5e537p+992"),
            lo: hexf64!("0x1.99a8f3ddc5edcp+933"),
        },
        TwoFloat {
            // exp(16)^44
            hi: hexf64!("0x1.93bf4ec282efbp+1015"),
            lo: hexf64!("0x1.9052bfcd70170p+960"),
        },
    ];

    // TODO: Check that the inversion doesn't lose precision
    if n.is_negative() {
        return 1.0 / exp_half(-n);
    }

    let (a, b) = ((n / 32) as usize, (n % 32) as usize);

    match (a > 0, b > 0) {
        (true, true) => EXP_16_N[a - 1] * EXP_HALF_N[b - 1],
        (true, false) => EXP_16_N[a - 1],
        (false, true) => EXP_HALF_N[b - 1],
        (false, false) => 1.into(),
    }
}

impl TwoFloat {
    fn expm1_quarter(self) -> TwoFloat {
        // We need to make sure that (1 + x) does not lose possible significant
        // digits, so no matter what strategy we choose here, the convergence
        // needs to go out to x = log(1.5) = 0.22. We have it work for until a
        // quarter, because that's a nice round power of two.
        assert!(self.hi().abs() <= 0.25);

        // The idea is to use the identity
        //
        //   expm1(x) = expm1(x0) + exp(x0) * expm1(x - x0)
        //
        // to reduce the expansion order.
        let n = libm::round(128.0 * self.hi());
        let x0 = n / 128.0;
        let y = self - x0;

        let expm1_x0 = expm1_128th(libm::trunc(n) as i32);
        let exp_x0 = expm1_x0 + 1.0;
        let expm1_y = y * polynomial!(y, 1.0, FRAC_FACT[2..15]);
        //return expm1_x0.add_small(exp_x0 * exp_y);
        expm1_x0 + exp_x0 * expm1_y
    }

    /// Returns `e^(self)`, (the exponential function).
    ///
    /// Computed by rewriting `self = y/2 +z` with `y` the rounded value of self.
    /// From this we can rewrite the exponential function into `exp(sel) = exp(1/2)^y * exp(z)`.
    /// The two exponential functions can now be computed by means of lookup table
    /// and fast converging taylor series.
    ///
    /// (Shout-out to the author of  [libxprec](https://github.com/tuwien-cms/libxprec) for
    /// pointing it out )
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(2.0);
    /// let b = a.exp();
    /// let e2 = twofloat::consts::E * twofloat::consts::E;
    ///
    /// assert!((b - e2).abs() / e2 < 1e-31);
    /// ```
    pub fn exp(self) -> Self {
        if self.hi <= EXP_LOWER_LIMIT {
            Self::from(0.0)
        } else if self.hi >= EXP_UPPER_LIMIT {
            Self {
                hi: f64::INFINITY,
                lo: 0.0,
            }
        } else if self.hi == 0.0 {
            Self::from(1.0)
        } else {
            // Compute the exponential of x = y/2 + z
            // Where y = round(2*x) giving z <= 0.25
            //
            // exp( y/2 + z ) = exp(1/2)^y * exp(z)
            //
            // exp(1/2)^y : can be computed with lookup table for integer y
            // exp(z) : can be computed using the value of exp_m1(z)
            //          with another lookup table

            // x = y/2 + z
            let y = libm::round(2.0 * self.hi());
            let z = self - y / 2.0;

            // exp(z + y/2) = (1 + expm1(z)) exp(1/2)^y
            let exp_z = z.expm1_quarter() + 1.0;
            let exp_y = exp_half(y as i32);
            return exp_z * exp_y;
        }
    }

    /// Returns `e^(self) - 1` in a way that provides additional accuracy
    /// when the value is close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// # use core::{convert::TryFrom};
    /// let a = TwoFloat::from(2f64.powi(-20));
    ///
    /// let b = a.exp_m1();
    /// let c = a.exp() - 1.0;
    ///
    /// // Exact Result
    /// // res = 9.5367477115374544678824955687428e-7;
    /// let res = TwoFloat::try_from((9.5367477115374552e-07, -7.0551613072428143e-23)).unwrap();
    ///
    /// assert!(((b-res)/res) == 0.0);
    /// assert!(((c-res)/res).abs() < 1e-29);
    /// ```
    pub fn exp_m1(self) -> Self {
        if self < -LN_2 || self > LN_FRAC_3_2 {
            self.exp() - 1.0
        } else {
            let x = self.abs();
            let r = polynomial!(x, 1.0, FRAC_FACT[2..15]);
            if self < 0.0 {
                self * r * self.exp()
            } else {
                self * r
            }
        }
    }

    /// Returns `2^(self)`.
    ///
    /// where self = k + r * n,  k > 0 and n = 2^9 = 512
    /// The taylor series for the small value of r converges very fast
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(0.5).exp2();
    /// let b = TwoFloat::from(2).sqrt();
    /// let c = (TwoFloat::from(0.5)*twofloat::consts::LN_2).exp();
    /// let res = twofloat::consts::SQRT_2;
    ///
    /// assert!((a - res).abs() < 1e-29);
    /// assert!((b - res).abs() < 1e-31);
    /// assert!((c - res).abs() < 1e-31);
    /// ```
    pub fn exp2(self) -> Self {
        if self < -1074.0 {
            Self::from(0.0)
        } else if self >= 1023.0 {
            Self {
                hi: f64::INFINITY,
                lo: f64::INFINITY,
            }
        } else {
            let k = libm::round(self.hi);
            let r = (self - k) * LN_2 / 512.0;
            //let x = self * LN_2;
            let mut r1 = polynomial!(r, FRAC_FACT[..12]);

            // Recover rescaling of r
            r1 = r1 * r1; // 2^(r * 2)
            r1 = r1 * r1; // 2^(r * 4)
            r1 = r1 * r1; // 2^(r * 8)
            r1 = r1 * r1; // 2^(r * 16)
            r1 = r1 * r1; // 2^(r * 32)
            r1 = r1 * r1; // 2^(r * 64)
            r1 = r1 * r1; // 2^(r * 128)
            r1 = r1 * r1; // 2^(r * 256)
            r1 = r1 * r1; // 2^(r * 512)

            //let r1 = polynomial!(r, 1.0, EXP2_COEFFS);
            if k == 0.0 {
                r1
            } else {
                Self {
                    hi: mul_pow2(r1.hi, k as i32),
                    lo: mul_pow2(r1.lo, k as i32),
                }
            }
        }
    }

    /// Returns the natural logarithm of the value.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp` function, so
    /// may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Example
    ///
    /// ```
    /// let a = twofloat::consts::E.ln();
    /// assert!((a - 1.0).abs() < 1e-31);
    /// ```
    pub fn ln(self) -> Self {
        if self == 1.0 {
            Self::from(0.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log(self.hi));
            x += self * (-x).exp() - 1.0;
            x += self * (-x).exp() - 1.0;
            x + self * (-x).exp() - 1.0
        }
    }

    /// Returns the natural logarithm of `1 + self`.
    ///
    /// Uses Newton–Raphson iteration which depends on the `expm1` function
    ///
    /// # Example
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(-0.5);
    /// let b = a.ln_1p();
    /// let c = -twofloat::consts::LN_2;//0.1f64.ln_1p();
    /// assert!((b - c).abs() < 1e-29);
    /// ```
    pub fn ln_1p(self) -> Self {
        if self == 0.0 {
            Self::from(0.0)
        } else if self <= -1.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log1p(self.hi));
            let mut e = x.exp_m1();
            x -= (e - self) / (e + 1.0);
            e = x.exp_m1();
            x - (e - self) / (e + 1.0)
        }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// This is a convenience method that computes `self.ln() / base.ln()`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(81.0);
    /// let b = TwoFloat::from(3.0);
    /// let c = TwoFloat::log(a, b);
    ///
    /// assert!((c - 4.0).abs()/4.0 < 1e-31);
    /// ```
    pub fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }

    /// Returns the base 2 logarithm of the number.
    ///
    /// Uses Newton–Raphson iteration which depends on the `exp2` function,
    /// so may not be fully accurate to the full precision of a `TwoFloat`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(64.0).log2();
    ///
    /// assert!(a - 6.0 == 0.0, "{}", a);
    /// ```
    pub fn log2(self) -> Self {
        if self == 1.0 {
            Self::from(1.0)
        } else if self <= 0.0 {
            Self::NAN
        } else {
            let mut x = Self::from(libm::log2(self.hi));
            x += (self * (-x).exp2() - 1.0) * FRAC_1_LN_2;
            x + (self * (-x).exp2() - 1.0) * FRAC_1_LN_2
        }
    }

    /// Returns the base 10 logarithm of the number.
    ///
    /// This is a convenience method that computes `self.ln() / LN_10`, no
    /// additional accuracy is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use twofloat::TwoFloat;
    /// let a = TwoFloat::from(100.0).log10();
    /// assert!((a - 2.0).abs() < 1e-30, "{}", a);
    /// ```
    pub fn log10(self) -> Self {
        self.ln() / LN_10
    }
}

#[cfg(test)]
mod tests {
    use crate::TwoFloat;

    #[test]
    fn exp_test() {
        assert_eq!(
            TwoFloat::from(-1000.0).exp(),
            0.0,
            "Large negative exponent produced non-zero value"
        );
        assert!(
            !TwoFloat::from(1000.0).exp().is_valid(),
            "Large positive exponent produced valid value"
        );
        assert_eq!(
            TwoFloat::from(0.0).exp(),
            TwoFloat::from(1.0),
            "exp(0) did not return 1"
        );
    }

    #[test]
    fn ln_test() {
        assert!(
            !TwoFloat::from(0.0).ln().is_valid(),
            "ln(0) produced valid result"
        );
        assert!(
            !TwoFloat::from(-5.0).ln().is_valid(),
            "ln(negative) produced valid result"
        );
        assert_eq!(
            TwoFloat::from(1.0).ln(),
            TwoFloat::from(0.0),
            "ln(1) did not return 0"
        );
    }
}
