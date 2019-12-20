use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use select::document::Document;
use select::predicate::Name;

use reqwest::get;

static CUR_EXPANSION: Int = 3;

struct BisQueryMapper;

impl BisQueryMapper {
    fn get_query(
        race: &str,
        class: &str,
        slot: &str,
        expansion: Option<&str>,
    ) -> Result<String, String> {
        let race_code = BisQueryMapper::map_race(race)?;
        let class_code = BisQueryMapper::map_class(class)?;
        let expac_code = BisQueryMapper::map_expac(expansion)?;

        let query_parts = vec![
            "http://eqitems.com/?",
            format!("frmSearch[classId]={}", class_code).as_str(),
            format!("&frmSearch[raceId]={}", race_code).as_str(),
            format!("&frmSearch[expansionId]={}", expac_code).as_str(),
            "&frmSearch[level]=60",
            "&frmSearch[deityId]=0",
            "&frmSearch[gearLevel]=99",
            "&frmSearch[NoDrop]=1",
            "&frmSearch[IgnoreEffects]=0",
            "&DoSearch=Search Best in Slot",
        ];

        let query = query_parts.iter().collect();

        Ok(percent_encode(query, NON_ALPHANUMERIC).to_string())
    }

    fn map_race(race: &str) -> Result<i32, String> {
        let search_race = race.to_ascii_lowercase().as_str();

        match search_race {
            "barbarian" => Ok(1),
            "dark-elf" => Ok(2),
            "darkelf" => Ok(2),
            "drakkin" => Ok(4),
            "dwarf" => Ok(8),
            "erudite" => Ok(16),
            "froglok" => Ok(32),
            "gnome" => Ok(64),
            "half-elf" => Ok(128),
            "halfelf" => Ok(128),
            "halfling" => Ok(256),
            "high-elf" => Ok(512),
            "highelf" => Ok(512),
            "human" => Ok(1024),
            "iksar" => Ok(2048),
            "ogre" => Ok(4096),
            "troll" => Ok(8192),
            "vah-shir" => Ok(16384),
            "vahshir" => Ok(16384),
            "wood-elf" => Ok(32768),
            "woodelf" => Ok(32768),
            _ => Err(format!("Unknown race: {}", race)),
        }
    }

    fn map_class(class: &str) -> Result<i32, String> {
        let search_class = class.to_ascii_lowercase().as_str();

        match search_class {
            "bard" => Ok(1),
            "beastlord" => Ok(2),
            "berserker" => Ok(4),
            "cleric" => Ok(8),
            "druid" => Ok(16),
            "enchanter" => Ok(32),
            "magician" => Ok(64),
            "monk" => Ok(128),
            "necromancer" => Ok(256),
            "paladin" => Ok(512),
            "ranger" => Ok(1024),
            "rogue" => Ok(2048),
            "shadow-knight" => Ok(4096),
            "shadowknight" => Ok(4096),
            "shaman" => Ok(8192),
            "warrior" => Ok(16384),
            "wizard" => Ok(32768),
            _ => Err(format!("Unknown class: {}", class)),
        }
    }

    fn map_expac(expac: Option<&str>) -> Result<i32, String> {
        let search_expac = match expac {
            Some(e) => e.to_ascii_lowercase().as_str(),
            None => return Ok(CUR_EXPANSION),
        };

        match search_expac {
            "classic" => Ok(0),
            "classic everquest" => Ok(0),
            "the ruins of kunark" => Ok(1),
            "the scars of velious" => Ok(2),
            "the shadows of luclin" => Ok(3),
            "the planes of power" => Ok(4),
            "the legacy of ykesha" => Ok(5),
            "lost dungeons of norrath" => Ok(6),
            "gates of discord" => Ok(7),
            "omens of war" => Ok(8),
            "dragons of norrath" => Ok(9),
            "depths of darkhollow" => Ok(10),
            "prophecy of ro" => Ok(11),
            "the serpent's spine" => Ok(12),
            "the buried sea" => Ok(13),
            "secrets of faydwer" => Ok(14),
            "seeds of destruction" => Ok(15),
            "underfoot" => Ok(16),
            "house of thule" => Ok(17),
            "veil of alaris" => Ok(18),
            "rain of fear" => Ok(19),
            "call of the forsaken" => Ok(20),
            "the darkened sea" => Ok(21),
            "the broken mirror" => Ok(22),
            "empires of kunark" => Ok(23),
            "ring of scale" => Ok(24),
            "the burning lands" => Ok(25),
            _ => Err(format!("Unknown expansion: {}", expac)),
        }
    }
}

pub struct Bis;

impl Bis {
    pub fn accept_raw(msg_parts: Vec<&str>) -> String {}

    fn do_search(
        race: &str,
        class: &str,
        expansion: Option<&str>,
    ) -> Vec<(String, String, Integer)> {
    }
}
