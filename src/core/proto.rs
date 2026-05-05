pub mod peerboard {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/peerboard.v1.rs"));
    }

    pub mod challenge {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/peerboard.challenge.v1.rs"));
        }
    }
}