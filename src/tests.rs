use super::*;

#[test]
fn hangul_start_type_from_char() {
    assert_eq!(HangulStartType::from_char('혀'), Some(HangulStartType::Hyeo));
    assert_eq!(HangulStartType::from_char('하'), Some(HangulStartType::Ha));
    assert_eq!(HangulStartType::from_char('흐'), Some(HangulStartType::Heu));
    assert_eq!(HangulStartType::from_char('형'), Some(HangulStartType::Hyeong));
    assert_eq!(HangulStartType::from_char('항'), Some(HangulStartType::Hang));
    assert_eq!(HangulStartType::from_char('핫'), Some(HangulStartType::Hat));
    assert_eq!(HangulStartType::from_char('흣'), Some(HangulStartType::Heut));
    assert_eq!(HangulStartType::from_char('흡'), Some(HangulStartType::Heup));
    assert_eq!(HangulStartType::from_char('흑'), Some(HangulStartType::Heuk));
    assert_eq!(HangulStartType::from_char('엉'), None);
    assert_eq!(HangulStartType::from_char('앙'), None);
    assert_eq!(HangulStartType::from_char('앗'), None);
    assert_eq!(HangulStartType::from_char('.'), None);
    assert_eq!(HangulStartType::from_char('?'), None);
    assert_eq!(HangulStartType::from_char('♥'), None);
}

#[test]
fn hangul_start_type_self_ending() {
    assert!(!  HangulStartType::Hyeo.is_self_ending());
    assert!(!    HangulStartType::Ha.is_self_ending());
    assert!(!   HangulStartType::Heu.is_self_ending());
    assert!( HangulStartType::Hyeong.is_self_ending());
    assert!(   HangulStartType::Hang.is_self_ending());
    assert!(    HangulStartType::Hat.is_self_ending());
    assert!(   HangulStartType::Heut.is_self_ending());
    assert!(   HangulStartType::Heup.is_self_ending());
    assert!(   HangulStartType::Heuk.is_self_ending());
}

#[test]
fn token_from_char() {
    assert_eq!(Token::from_char('.'), Some(Token::Dot));
    assert_eq!(Token::from_char('!'), Some(Token::ExclamationMark));
    assert_eq!(Token::from_char('?'), Some(Token::QuestionMark));
    assert_eq!(Token::from_char('\u{2026}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{22ee}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{22ef}'), Some(Token::ThreeDots));
    assert_eq!(Token::from_char('\u{2661}'), Some(Token::ReturnHeart));
}

#[test]
fn token_from_char_hearts() {
}
