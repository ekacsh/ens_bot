use crate::domain::rank::Rank;

pub enum Mee6Ranks {
    Level55Plus,
    Level50Plus,
    Level45Plus,
    Level40Plus,
    Level35Plus,
    Level30Plus,
    Level25Plus,
    Level20Plus,
    Level15Plus,
    Level10Plus,
    Level5Plus,
    LowLevel,
}

impl Mee6Ranks {
    pub fn as_rank(&self) -> Rank {
        match self {
            Mee6Ranks::Level55Plus => Rank {
                name: "Mee6 Level 55+",
                rank_id: 1146929830319886427,
                code: "-",
            },
            Mee6Ranks::Level50Plus => Rank {
                name: "Mee6 Level 50+",
                rank_id: 1121872671811059712,
                code: "-",
            },
            Mee6Ranks::Level45Plus => Rank {
                name: "Mee6 Level 45+",
                rank_id: 1121872431364190308,
                code: "-",
            },
            Mee6Ranks::Level40Plus => Rank {
                name: "Mee6 Level 40+",
                rank_id: 930847990602674236,
                code: "-",
            },
            Mee6Ranks::Level35Plus => Rank {
                name: "Mee6 Level 35+",
                rank_id: 935574768869081088,
                code: "-",
            },
            Mee6Ranks::Level30Plus => Rank {
                name: "Mee6 Level 30+",
                rank_id: 925506600876253224,
                code: "-",
            },
            Mee6Ranks::Level25Plus => Rank {
                name: "Mee6 Level 25+",
                rank_id: 898343022109032469,
                code: "-",
            },
            Mee6Ranks::Level20Plus => Rank {
                name: "Mee6 Level 20+",
                rank_id: 898342984427393034,
                code: "-",
            },
            Mee6Ranks::Level15Plus => Rank {
                name: "Mee6 Level 15+",
                rank_id: 898342951032340490,
                code: "-",
            },
            Mee6Ranks::Level10Plus => Rank {
                name: "Mee6 Level 10+",
                rank_id: 898342910079172650,
                code: "-",
            },
            Mee6Ranks::Level5Plus => Rank {
                name: "Mee6 Level 5+",
                rank_id: 898342739001884732,
                code: "-",
            },
            Mee6Ranks::LowLevel => Rank {
                name: "Mee6 Low Level",
                rank_id: 0,
                code: "-",
            },
        }
    }

    pub fn from_level(level: i32) -> Self {
        match level {
            55.. => Mee6Ranks::Level55Plus,
            50.. => Mee6Ranks::Level50Plus,
            45.. => Mee6Ranks::Level45Plus,
            40.. => Mee6Ranks::Level40Plus,
            35.. => Mee6Ranks::Level35Plus,
            30.. => Mee6Ranks::Level30Plus,
            25.. => Mee6Ranks::Level25Plus,
            20.. => Mee6Ranks::Level20Plus,
            15.. => Mee6Ranks::Level15Plus,
            10.. => Mee6Ranks::Level10Plus,
            5.. => Mee6Ranks::Level5Plus,
            _ => Mee6Ranks::LowLevel,
        }
    }

    const ALL: [Mee6Ranks; 11] = [
        Mee6Ranks::Level55Plus,
        Mee6Ranks::Level50Plus,
        Mee6Ranks::Level45Plus,
        Mee6Ranks::Level40Plus,
        Mee6Ranks::Level35Plus,
        Mee6Ranks::Level30Plus,
        Mee6Ranks::Level25Plus,
        Mee6Ranks::Level20Plus,
        Mee6Ranks::Level15Plus,
        Mee6Ranks::Level10Plus,
        Mee6Ranks::Level5Plus,
    ];

    pub fn is_mee6_rank(&self, rank_id: u64) -> bool {
        Mee6Ranks::ALL
            .iter()
            .any(|r| r.as_rank().rank_id == rank_id)
    }
}
