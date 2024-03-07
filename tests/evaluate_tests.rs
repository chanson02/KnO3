use kn_o3::board::Chessboard;

#[test]
fn test_get_score() {
    let new = Chessboard::new();
    assert!(new.get_score() == 0);
}
