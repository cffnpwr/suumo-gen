use std::fmt::Display;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum SuumoElement {
    ASuumo,
    Dan,
    Shaan,
    SumoFullMoon,
    SumoNewMoon,
    SuuuumoUp,
    SuuuumoDown,
}
impl SuumoElement {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => Self::ASuumo,
            1 => Self::Dan,
            2 => Self::Shaan,
            3 => Self::SumoFullMoon,
            4 => Self::SumoNewMoon,
            5 => Self::SuuuumoUp,
            6 => Self::SuuuumoDown,
            _ => unreachable!(),
        }
    }
}
impl Display for SuumoElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ASuumo => write!(f, "あ❗️ スーモ❗️🌚"),
            Self::Dan => write!(f, "ダン💥"),
            Self::Shaan => write!(f, "シャーン🎶"),
            Self::SumoFullMoon => write!(f, "スモ🌝"),
            Self::SumoNewMoon => write!(f, "スモ🌚"),
            Self::SuuuumoUp => write!(f, "ス〜〜〜モ⤴🌝"),
            Self::SuuuumoDown => write!(f, "ス〜〜〜モ⤵🌞"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SuumoState {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
}
impl SuumoState {
    pub fn new() -> Self {
        Self::S0
    }

    pub fn next_with_suumo_element(&mut self, next_element: SuumoElement) -> Option<Self> {
        match self {
            Self::S1 if next_element == SuumoElement::Dan => {
                *self = Self::S2;

                Some(Self::S2)
            }
            Self::S2 if next_element == SuumoElement::Dan => {
                *self = Self::S3;

                Some(Self::S3)
            }
            Self::S3 if next_element == SuumoElement::Dan => {
                *self = Self::S4;

                Some(Self::S4)
            }
            Self::S4 if next_element == SuumoElement::Shaan => {
                *self = Self::S5;

                Some(Self::S5)
            }
            Self::S5 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S6;

                Some(Self::S6)
            }
            Self::S6 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S7;

                Some(Self::S7)
            }
            Self::S7 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S8;

                Some(Self::S8)
            }
            Self::S8 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S9;

                Some(Self::S9)
            }
            Self::S9 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S10;

                Some(Self::S10)
            }
            Self::S10 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S11;

                Some(Self::S11)
            }
            Self::S11 if next_element == SuumoElement::SuuuumoUp => {
                *self = Self::S12;

                Some(Self::S12)
            }
            Self::S12 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S13;

                Some(Self::S13)
            }
            Self::S13 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S14;

                Some(Self::S14)
            }
            Self::S14 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S15;

                Some(Self::S15)
            }
            Self::S15 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S16;

                Some(Self::S16)
            }
            Self::S16 if next_element == SuumoElement::SumoNewMoon => {
                *self = Self::S17;

                Some(Self::S17)
            }
            Self::S17 if next_element == SuumoElement::SumoFullMoon => {
                *self = Self::S18;

                Some(Self::S18)
            }
            Self::S18 if next_element == SuumoElement::SuuuumoDown => {
                *self = Self::S19;

                Some(Self::S19)
            }
            Self::S19 => None,
            _ if next_element == SuumoElement::ASuumo => {
                *self = Self::S1;

                Some(Self::S1)
            }
            _ => {
                *self = Self::S0;

                Some(Self::S0)
            }
        }
    }
}
impl Iterator for SuumoState {
    type Item = SuumoState;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_with_suumo_element(SuumoElement::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static ELEMENTS: Lazy<Vec<SuumoElement>> = Lazy::new(|| {
        vec![
            SuumoElement::ASuumo,
            SuumoElement::Dan,
            SuumoElement::Shaan,
            SuumoElement::SumoFullMoon,
            SuumoElement::SumoNewMoon,
            SuumoElement::SuuuumoUp,
            SuumoElement::SuuuumoDown,
        ]
    });

    #[test]
    fn suumo_element_to_string() {
        assert_eq!(SuumoElement::ASuumo.to_string(), "あ❗️ スーモ❗️🌚");
        assert_eq!(SuumoElement::Dan.to_string(), "ダン💥");
        assert_eq!(SuumoElement::Shaan.to_string(), "シャーン🎶");
        assert_eq!(SuumoElement::SumoFullMoon.to_string(), "スモ🌝");
        assert_eq!(SuumoElement::SumoNewMoon.to_string(), "スモ🌚");
        assert_eq!(SuumoElement::SuuuumoUp.to_string(), "ス〜〜〜モ⤴🌝");
        assert_eq!(SuumoElement::SuuuumoDown.to_string(), "ス〜〜〜モ⤵🌞");
    }

    #[test]
    fn new_suumo_state() {
        let suumo_state = SuumoState::new();

        assert_eq!(suumo_state, SuumoState::S0);
    }

    #[test]
    fn suumo_state_s0() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S0;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s1() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S1;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::Dan => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S2)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s2() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S2;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::Dan => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S3)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s3() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S3;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::Dan => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S4)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s4() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S4;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::Shaan => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S5)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s5() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S5;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S6)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s6() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S6;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S7)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s7() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S7;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S8)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s8() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S8;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S9)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s9() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S9;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S10)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s10() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S10;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S11)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s11() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S11;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SuuuumoUp => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S12)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s12() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S12;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S13)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s13() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S13;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S14)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s14() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S14;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S15)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s15() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S15;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S16)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s16() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S16;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoNewMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S17)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s17() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S17;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SumoFullMoon => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S18)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s18() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S18;
            match element {
                SuumoElement::ASuumo => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S1)
                    );
                }
                SuumoElement::SuuuumoDown => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S19)
                    );
                }
                _ => {
                    assert_eq!(
                        suumo_state.next_with_suumo_element(element.clone()),
                        Some(SuumoState::S0)
                    );
                }
            }
        }
    }

    #[test]
    fn suumo_state_s19() {
        for element in ELEMENTS.iter() {
            let mut suumo_state = SuumoState::S19;
            assert_eq!(suumo_state.next_with_suumo_element(element.clone()), None);
        }
    }
}
