use crate::{TwoFloat, TwoFloatError};
use core::convert::TryFrom;
use core::fmt;

impl fmt::Display for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert TwoFloat to a i128 pair
        let (num, exp) = self.as_i128_pair().unwrap();
        let p = f.precision().unwrap_or(32).min(32);
        let mut num_str = if f.sign_plus() {
            match f.precision() {
                Some(p) => format!("{:+.*}", p, num,),
                None => format!("{:+.32}", num),
            }
        } else {
            match f.precision() {
                Some(p) => format!("{:.*}", p, num),
                None => format!("{:.32}", num),
            }
        };

        // Define first digit position
        let first_digit_pos = if num < 0 || f.sign_plus() { 2 } else { 1 };

        if exp.abs() < 5 {
            if exp >= 0 {
                num_str.insert(first_digit_pos + exp as usize, '.');
                num_str.truncate(first_digit_pos + 1 + p + exp as usize);
            } else {
                (0..-exp).for_each(|_| num_str.insert(first_digit_pos - 1, '0'));
                num_str.insert(first_digit_pos, '.');
                num_str.truncate(first_digit_pos + 1 + p);
            }
        } else {
            num_str.insert(first_digit_pos, '.');
            num_str.truncate(first_digit_pos + 1 + p);
            num_str.push_str(format!("e{}", exp).as_str());
        }

        if p == 0 {
            num_str = num_str.replace(".", "");
        }
        write!(f, "{num_str}")
    }
}

impl TwoFloat {
    pub fn normalize(&mut self) {
        let u = self.hi + self.lo;
        let mut v = self.hi - u;
        v += self.lo;

        self.hi = u;
        self.lo = v;
    }

    /// Convert TwoFloat into a i128 pair containing the decimal representation
    /// and the associated exponent in base 10
    pub fn as_i128_pair(&self) -> Result<(i128, i128), TwoFloatError> {
        let precision = 32;

        // Normalize representation to have two non-overlapping f64
        let mut ddouble = *self;
        ddouble.normalize();

        // HIGH parto of TwoFloat
        // Obtain precise string of 32 digits from f64 mantissa and convert into i128
        let str_hi = format!("{:.*e}", precision, ddouble.hi());
        let num_exp_hi = str_hi
            .replace(".", "")
            .split("e")
            .map(|n| n.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        // LOW parto of TwoFloat
        // Obtain precise string of 32 digits from f64 mantissa and convert into i128
        let num_exp_lo = if ddouble.lo() == 0.0 {
            vec![0, num_exp_hi[1]]
        } else {
            let str_lo = format!("{:.*e}", precision, ddouble.lo());
            str_lo
                .replace(".", "")
                .splitn(2, "e")
                .map(|n| n.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        };

        let (num_hi, num_lo) = (num_exp_hi[0], num_exp_lo[0]);
        let exp = num_exp_hi[1];

        // Shift digits of `lo` for summation with `hi`
        if exp < num_exp_lo[1] {
            return Err(TwoFloatError::FmtError);
        }
        let mut num = num_hi;
        if exp - num_exp_lo[1] < 46 {
            num += num_lo / 10_i128.pow(u32::try_from(exp - num_exp_lo[1]).unwrap());
        }
        Ok((num, exp))
    }
}

impl fmt::LowerExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert TwoFloat to a i128 pair
        let (num, exp) = self.as_i128_pair().unwrap();
        let p = f.precision().unwrap_or(32).min(32);
        let mut num_str = if f.sign_plus() {
            match f.precision() {
                Some(p) => format!("{:+.*}", p, num,),
                None => format!("{:+.32}", num),
            }
        } else {
            match f.precision() {
                Some(p) => format!("{:.*}", p, num),
                None => format!("{:.32}", num),
            }
        };

        // Define first digit position
        let first_digit_pos = if num < 0 || f.sign_plus() { 2 } else { 1 };

        num_str.insert(first_digit_pos, '.');
        num_str.truncate(first_digit_pos + 1 + p);
        num_str.push_str(format!("e{}", exp).as_str());

        if p == 0 {
            num_str = num_str.replace(".", "");
        }
        write!(f, "{num_str}")
    }
}

impl fmt::UpperExp for TwoFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert TwoFloat to a i128 pair
        let (num, exp) = self.as_i128_pair().unwrap();
        let p = f.precision().unwrap_or(32).min(32);
        let mut num_str = if f.sign_plus() {
            match f.precision() {
                Some(p) => format!("{:+.*}", p, num,),
                None => format!("{:+.32}", num),
            }
        } else {
            match f.precision() {
                Some(p) => format!("{:.*}", p, num),
                None => format!("{:.32}", num),
            }
        };

        // Define first digit position
        let first_digit_pos = if num < 0 || f.sign_plus() { 2 } else { 1 };

        num_str.insert(first_digit_pos, '.');
        num_str.truncate(first_digit_pos + 1 + p);
        num_str.push_str(format!("E{}", exp).as_str());

        if p == 0 {
            num_str = num_str.replace(".", "");
        }
        write!(f, "{num_str}")
    }
}

#[cfg(all(feature = "std", test))]
mod test {
    use crate::TwoFloat;

    #[test]
    fn display_test() {
        let value = TwoFloat { hi: 1.0, lo: 0.4 };
        assert_eq!(format!("{:.29}", value), "1.40000000000000002220446049250");
        assert_eq!(
            format!("{:.29}", -value),
            "-1.40000000000000002220446049250"
        );
        assert_eq!(format!("{:+.10}", value), "+1.4000000000");
        assert_eq!(format!("{:.2}", value), "1.40");
        assert_eq!(format!("{:.2}", -value), "-1.40");
        assert_eq!(format!("{:+.2}", value), "+1.40");
    }

    #[test]
    fn lowerexp_test() {
        let value = TwoFloat { hi: 1.0, lo: -0.3 };
        assert_eq!(
            format!("{:e}", value),
            "7.00000000000000011102230246251565e-1"
        );
        assert_eq!(
            format!("{:e}", -value),
            "-7.00000000000000011102230246251565e-1"
        );
        assert_eq!(
            format!("{:+e}", value),
            "+7.00000000000000011102230246251565e-1"
        );
        assert_eq!(format!("{:.2e}", value), "7.00e-1");
        assert_eq!(format!("{:.2e}", -value), "-7.00e-1");
        assert_eq!(format!("{:+.2e}", value), "+7.00e-1");
        assert_eq!(format!("{:.0e}", value), "7e-1");
        assert_eq!(format!("{:.0e}", -value), "-7e-1");
        assert_eq!(format!("{:+.0e}", value), "+7e-1");
    }

    #[test]
    fn upperexp_test() {
        let value = TwoFloat { hi: 1.0, lo: 0.3 };
        assert_eq!(
            format!("{:E}", value),
            "1.29999999999999998889776975374844E0"
        );
        assert_eq!(
            format!("{:E}", -value),
            "-1.29999999999999998889776975374844E0"
        );
        assert_eq!(
            format!("{:+E}", value),
            "+1.29999999999999998889776975374844E0"
        );
        assert_eq!(format!("{:.2E}", value), "1.29E0");
        assert_eq!(format!("{:.2E}", -value), "-1.29E0");
        assert_eq!(format!("{:+.2E}", value), "+1.29E0");
    }

    #[test]
    fn representation_test() {
        // Check representation of TwoFloat with a mantissa representing 2^-200
        let a = TwoFloat::new_add(2f64.powi(-200), 0.0);
        assert_eq!(a.hi(), 6.223015277861142e-61);
        assert_eq!(a.lo(), 0.0);
        assert_eq!(format!("{}", a), "6.22301527786114170714406405378012e-61");
        assert_eq!(
            format!("{:+}", a),
            "+6.22301527786114170714406405378012e-61"
        );

        let a = TwoFloat::new_add(-7.002331194145285e261, -3.0691966300953055e-292);
        assert_eq!(format!("{}", a), "-7.00233119414528470514145334361671e261");

        let a = TwoFloat::new_add(4.9504043073917445e187, -7.783010164746793e-193);
        assert_eq!(format!("{}", a), "4.95040430739174446644728402159648e187");

        let a = TwoFloat::new_add(4.7285140888327324e-154, -6.947626393992663e-171);
        assert_eq!(format!("{}", a), "4.72851408883273234179034984593673e-154");

        let a = TwoFloat::new_add(-1.914530883798549e-175, 2.7001070760038845e25);
        assert_eq!(format!("{}", a), "2.70010707600388445521838080000000e25");

        let a = TwoFloat::new_add(-1.651835746059791e275, 1.8892141968775649e112);
        assert_eq!(format!("{}", a), "-1.65183574605979111550616178666366e275");

        let a = TwoFloat::new_add(3.311911036609425e-166, -0.8253639786890961);
        assert_eq!(format!("{}", a), "-0.82536397868909605612941504659829");

        let a = TwoFloat::new_add(-8.382204862545721e-209, -1.7129470347388192e-228);
        assert_eq!(format!("{}", a), "-8.38220486254572110606670878720825e-209");

        let a = TwoFloat::new_add(4.2499011457145386e272, 8.642923960287653e-218);
        assert_eq!(format!("{}", a), "4.24990114571453864311787472476268e272");

        let a = TwoFloat::new_add(4.534141350609096e-44, 2.7700855291511005e253);
        assert_eq!(format!("{}", a), "2.77008552915110048476418949222398e253");

        let a = TwoFloat::new_add(63.55157396097761, 1.920985695640798e100);
        assert_eq!(format!("{}", a), "1.92098569564079784073860799157444e100");

        let a = TwoFloat::new_add(4.30428772920657e-51, 6.322345521521574e119);
        assert_eq!(format!("{}", a), "6.32234552152157424311468967698072e119");

        let a = TwoFloat::new_add(-5.9930006057943446e-148, 1.707103094941765e240);
        assert_eq!(format!("{}", a), "1.70710309494176494268007970591919e240");

        let a = TwoFloat::new_add(3.2619025217862523e240, 4.083685700133997e44);
        assert_eq!(format!("{}", a), "3.26190252178625228774189801771006e240");

        let a = TwoFloat::new_add(2.537621718979616e246, 3.363893025742543e-82);
        assert_eq!(format!("{}", a), "2.53762171897961584683232501858424e246");

        let a = TwoFloat::new_add(-4.854901303333293e25, 4.29254101196476e135);
        assert_eq!(format!("{}", a), "4.29254101196475991033573405990271e135");

        let a = TwoFloat::new_add(-1.3088114389892744e-140, -2.0457184865512558e112);
        assert_eq!(format!("{}", a), "-2.04571848655125575683428046259204e112");

        let a = TwoFloat::new_add(2.550135165244688e-155, -3.714866916147064e-66);
        assert_eq!(format!("{}", a), "-3.71486691614706400643217621476249e-66");

        let a = TwoFloat::new_add(7.64012400356314e176, 1.6153532745222202e-134);
        assert_eq!(format!("{}", a), "7.64012400356313944494626929186346e176");

        let a = TwoFloat::new_add(1.8124585610631264e137, 5.43909443175123e-193);
        assert_eq!(format!("{}", a), "1.81245856106312642199139094722723e137");

        let a = TwoFloat::new_add(3.305454918057549e-42, 1.336639270599439e38);
        assert_eq!(format!("{}", a), "1.33663927059943903253450114419537e38");
    }
}
