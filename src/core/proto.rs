pub mod peerboard {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/peerboard.v1.rs"));
    }
}