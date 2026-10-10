#[cfg(test)]
mod tests {
    use crate::earcons::{EarconPlayer, EarconType};
    use crate::ptt::PttEvent;
    use crossbeam_channel::unbounded;

    #[test]
    fn test_ptt_event_channel() {
        let (tx, rx) = unbounded();
        tx.send(PttEvent::Pressed).unwrap();
        tx.send(PttEvent::Released).unwrap();

        assert_eq!(rx.recv().unwrap(), PttEvent::Pressed);
        assert_eq!(rx.recv().unwrap(), PttEvent::Released);
    }

    #[test]
    fn test_earcon_player_instantiation() {
        let player = EarconPlayer::new();
        assert!(player.is_ok());
        if let Ok(p) = player {
            p.play(EarconType::PttStart);
            p.play(EarconType::PttStop);
        }
    }
}
