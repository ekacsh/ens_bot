use crate::domain::rank::Rank;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FalconRank {
    Admin,
    NewMember,
    GuildMember,
    CopperFalcon,
    IronFalcon,
    GoldFalcon,
    PlatinumFalcon,
    DementiaFalcon,
    VoidFalcon,
    LustreFalcon,
    StarfireFalcon,
    DreadloFalcon,
    GodshardFalcon,
    OnBreak,
    UnderGp,
    Kick,
}

static CODE_TO_RANK: Lazy<HashMap<&'static str, FalconRank>> = Lazy::new(|| {
    use FalconRank::*;
    HashMap::from([
        ("NM", NewMember),
        ("M", GuildMember),
        ("C", CopperFalcon),
        ("I", IronFalcon),
        ("G", GoldFalcon),
        ("P", PlatinumFalcon),
        ("D", DementiaFalcon),
        ("V", VoidFalcon),
        ("L", LustreFalcon),
        ("SF", StarfireFalcon),
        ("DL", DreadloFalcon),
        ("GS", GodshardFalcon),
        ("BREAK", OnBreak),
        ("U", UnderGp),
        ("KICK", Kick),
    ])
});

impl FalconRank {
    pub fn as_rank(&self) -> Rank {
        match self {
            FalconRank::Admin => Rank {
                name: "Admin",
                rank_id: 898340252492660776,
                code: "-",
            },
            FalconRank::NewMember => Rank {
                name: "New Member",
                rank_id: 899349090431807498,
                code: "NM",
            },
            FalconRank::GuildMember => Rank {
                name: "Guild Member",
                rank_id: 898340609239166996,
                code: "M",
            },
            FalconRank::CopperFalcon => Rank {
                name: "Copper Falcon",
                rank_id: 899346957343019029,
                code: "C",
            },
            FalconRank::IronFalcon => Rank {
                name: "Iron Falcon",
                rank_id: 899346990108934214,
                code: "I",
            },
            FalconRank::GoldFalcon => Rank {
                name: "Gold Falcon",
                rank_id: 899347063576342620,
                code: "G",
            },
            FalconRank::PlatinumFalcon => Rank {
                name: "Platinum Falcon",
                rank_id: 903288583404806164,
                code: "P",
            },
            FalconRank::DementiaFalcon => Rank {
                name: "Dementia Falcon",
                rank_id: 903289983639621672,
                code: "D",
            },
            FalconRank::VoidFalcon => Rank {
                name: "Void Falcon",
                rank_id: 903290650953392198,
                code: "V",
            },
            FalconRank::LustreFalcon => Rank {
                name: "Lustre Falcon",
                rank_id: 916719322100015184,
                code: "L",
            },
            FalconRank::StarfireFalcon => Rank {
                name: "Starfire Falcon",
                rank_id: 1110972496515186688,
                code: "SF",
            },
            FalconRank::DreadloFalcon => Rank {
                name: "Dreadlo Falcon",
                rank_id: 1110973154253352992,
                code: "DL",
            },
            FalconRank::GodshardFalcon => Rank {
                name: "Godshard Falcon",
                rank_id: 1110975157020917810,
                code: "GS",
            },
            FalconRank::UnderGp => Rank {
                name: "Under GP",
                rank_id: 904308472080904193,
                code: "U",
            },
            FalconRank::OnBreak => Rank {
                name: "On Break",
                rank_id: 898340609239166996,
                code: "BREAK",
            },
            FalconRank::Kick => Rank {
                name: "Kick",
                rank_id: 904308472080904193,
                code: "KICK",
            },
        }
    }

    pub const RANK_ROLES: [FalconRank; 14] = [
        FalconRank::NewMember,
        FalconRank::CopperFalcon,
        FalconRank::IronFalcon,
        FalconRank::GoldFalcon,
        FalconRank::PlatinumFalcon,
        FalconRank::DementiaFalcon,
        FalconRank::VoidFalcon,
        FalconRank::LustreFalcon,
        FalconRank::StarfireFalcon,
        FalconRank::DreadloFalcon,
        FalconRank::GodshardFalcon,
        FalconRank::OnBreak,
        FalconRank::UnderGp,
        FalconRank::Kick,
    ];

    pub fn from_code(code: &str) -> Option<Self> {
        CODE_TO_RANK.get(code).copied()
    }
}
