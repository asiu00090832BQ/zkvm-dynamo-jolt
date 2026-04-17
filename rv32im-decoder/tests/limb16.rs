use rv32im_decoder::{Limb16, Word};

#[test]
fn splits_and_rejoins_a_word() {
    let word = 0xdead_beefu32;
    let (low, high) = Limb16::split(word);

    assert_eq!(low.get(), 0xbeef);
    assert_eq!(high.get(), 0xdead);
    assert_eq!(Limb16::join(low, high), word);
}

#[test]
fn word_round_trips_through_limbs() {
    let word = Word::new(0x1234_5678);
    let (low, high) = word.limbs();

    assert_eq!(Word::from_limbs(low, high), word);
    assert_eq!(word.low_limb(), Limb16::new(0x5678));
    assert_eq!(word.high_limb(), Limb16::new(0x1234));
}
