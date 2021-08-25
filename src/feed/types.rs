use serde::{Deserialize, Deserializer};

fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(d)?;
    let val = opt.unwrap_or_else(T::default);
    Ok(val)
}

pub struct FeedEvent {
    pub id: Uuid,
    #[serde(deserialize_with = "null_to_default")]
    pub player_tags: Vec<Uuid>,
    #[serde(deserialize_with = "null_to_default")]
    pub team_tags: Vec<Uuid>,
    #[serde(deserialize_with = "null_to_default")]
    pub game_tags: Vec<Uuid>,
    pub metadata: JSONValue,
    pub created: DateTime<Utc>,
    pub season: i32,
    pub tournament: i32,
    pub r#type: i32,
    pub day: i32,
    pub phase: i32,
    pub category: i32,
    pub nuts: i32,
    pub description: String
}

impl FeedEvent {
    /// Returns an enum describing the being which said this message, or None if there isn't one.
    pub fn being(&self) -> Option<i64> {
        use EventBeing::*;
        self.metadata.get("being").and_then(|v| v.as_i64()).and_then(|v| match v {
            -1 => EmergencyAlert,
            0 => ShelledOne,
            1 => Monitor,
            2 => Coin,
            3 => Reader,
            4 => Microphone,
            5 => Lootcrates,
            6 => Namerifeht,
            _ => Other(v as i32)
        })
    }

    /// Returns an enum describing the phase, or in-game period, in which this event occurred.
    pub fn phase(&self) -> EventPhase {
        use EventPhase::*;
        match self.phase {
            0 => GodsDay,
            1 => PreSeason,
            2 => EarlSeason,
            3 => EarlSiesta,
            4 => MidSeason,
            5 => LateSiesta,
            6 => LateSeason,
            7 => EndSeason,
            8 => PrePostSeason,
            9 => EarlPostSeason,
            10 => EarlPostSeasonEnd,
            11 => LatePostSeason,
            12 => PostSeasonEnd,
            13 => Election,
            14 => SpecialEvent,
            15 | 16 | 17 | 18 => Tournament(self.phase),
            _ => Other(self.phase)
        }
    }

    /// Returns an enum describing the category of this event.
    pub fn category(&self) -> EventCategory {
        use EventCategory::*;
        match self.category {
            0 => Plays,
            1 => Changes,
            2 => Special,
            3 => Outcomes,
            4 => Narrative,
            _ => Other(self.category)
        }
    }
}

/// EventPhase enumerates the in-game period in which an event ocurred, as determined by blaseball's calendar.
pub enum EventPhase {
    GodsDay,
    PreSeason,
    EarlSeason,
    EarlSiesta,
    MidSeason,
    LateSiesta,
    LateSeason,
    EndSeason,
    PrePostSeason,
    EarlPostSeason
    EarlPostSeasonEnd,
    LatePostSeason,
    PostSeasonEnd,
    Election,
    SpecialEvent,
    /// Tournament phases are unknown and span about four phases, so this is a catch-all for them.
    Tournament(i32),
    Other(i32)
}

/// Enumerates the event category, which is used by the blaseball website to filter down the feed.
pub enum EventCategory {
    Plays,
    Changes,
    Special,
    Outcomes,
    Narrative,
    Other(i32)
}

/// The being who said this message.
pub enum EventBeing {
    EmergencyAlert,
    ShelledOne,
    Monitor,
    Coin,
    Reader,
    Microphone,
    Lootcrates,
    Namerifeht,
    Other(i32)
}
