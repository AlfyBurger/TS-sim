#[cfg(test)]
mod t {
    use crate::data::*;
    use crate::engine::*;
    use crate::state::*;
    use crate::types::*;
    fn s0() -> State { State::new() }
    #[test] fn control_rule() {
        let mut s = s0(); s.inf[ISRAEL] = [4, 0]; assert_eq!(s.ctrl(ISRAEL), US as i8);
        s.inf[ISRAEL] = [4, 1]; assert_eq!(s.ctrl(ISRAEL), -1);
        s.inf[TURKEY] = [2, 0]; assert!(s.controls(US, TURKEY));
        s.inf[TURKEY] = [2, 1]; assert!(!s.controls(US, TURKEY));
    }
    #[test] fn coup_mexico_example() {
        // Rulebook 6.3 example: US 3 Ops Coup in Mexico (USSR 2), roll 4 -> remove 2, add 1.
        let mut s = s0(); s.inf[MEXICO] = [0, 2];
        let r = coup_result(&s, US, MEXICO, 3, false, 4);
        assert_eq!(r.inf[MEXICO], [1, 0]); assert_eq!(r.milops[US], 3); assert_eq!(r.defcon, 4);
    }
    #[test] fn realign_nk_example() {
        let mut s = s0(); s.inf[N_KOREA] = [0, 3];
        assert_eq!(realign_mod(&s, US, N_KOREA), 0); assert_eq!(realign_mod(&s, USSR, N_KOREA), 2);
    }
    #[test] fn central_america_example() {
        let mut s = s0();
        s.inf[CUBA] = [0, 3]; s.inf[HAITI] = [0, 1]; s.inf[DOMINICAN_REP] = [0, 1]; s.inf[GUATEMALA] = [1, 0]; s.inf[PANAMA] = [1, 0];
        assert_eq!(region_score(&s, CENTAM, None), (1, 5));
    }
    #[test] fn example_asia_scoring() {
        let mut s = s0();
        for (c, v) in [(JAPAN, [4, 0]), (S_KOREA, [3, 0]), (THAILAND, [3, 1]), (AUSTRALIA, [4, 0]), (PHILIPPINES, [1, 0]), (MALAYSIA, [1, 1]), (PAKISTAN, [1, 0]), (N_KOREA, [0, 3]), (VIETNAM, [0, 2]), (LAOS_CAMBODIA, [0, 1])] { s.inf[c] = v; }
        assert_eq!(region_score(&s, ASIA, None), (10, 4));
    }
    #[test] fn defcon_geography() {
        let mut s = s0(); s.inf[ITALY] = [2, 0]; s.inf[IRAN] = [1, 0]; s.inf[JAPAN] = [1, 0];
        s.defcon = 4; assert!(!can_coup(&s, USSR, ITALY, false)); assert!(can_coup(&s, USSR, IRAN, false)); assert!(can_coup(&s, USSR, ITALY, true));
        s.defcon = 3; assert!(!can_coup(&s, USSR, JAPAN, false));
        s.defcon = 2; assert!(!can_coup(&s, USSR, IRAN, false));
    }
    #[test] fn bg_coup_degrades_and_phasing_loses() {
        let mut s = s0(); s.inf[IRAN] = [1, 0]; s.defcon = 2; s.phasing = USSR as u8;
        let r = coup_result(&s, USSR, IRAN, 3, false, 1);
        assert_eq!(r.winner, US as i8);
    }
    #[test] fn space_track() {
        let mut s = s0(); space_advance(&mut s, USSR, 1, false); assert_eq!(s.vp, -2);
        space_advance(&mut s, US, 1, false); assert_eq!(s.vp, -1);
        let mut s = s0(); space_advance(&mut s, US, 2, true); assert_eq!(s.space[US], 2); assert_eq!(s.vp, 0); assert!(s.space_ability(US, 2));
    }
    #[test] fn ops_modifiers() {
        let mut s = s0(); s.ev.red_scare[US] = true; assert_eq!(s.card_ops(US, 4), 2); assert_eq!(s.card_ops(US, 19), 1);
        s.ev.containment = true; assert_eq!(s.card_ops(US, 23), 3); assert_eq!(s.card_ops(US, 4), 3); s.ev.red_scare[US] = false; assert_eq!(s.card_ops(US, 23), 4); assert_eq!(s.card_ops(US, 4), 4);
        assert_eq!(ops_as_if(&s, US, 1), 2);
    }
    #[test] fn card_counts() {
        let n = |e| (1..=110usize).filter(|&k| CARDS[k].era == e).count();
        assert_eq!((n(0), n(1), n(2)), (39, 48, 23));
        assert_eq!(COUNTRIES.iter().filter(|c| c.bg).count(), 29);
    }
}
