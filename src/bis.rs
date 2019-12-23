use percent_encoding::{percent_encode, CONTROLS};

use select::predicate::{Attr, Name, Predicate};

use crate::util::AllaUtil;
use select::node::{Children, Node};
use std::borrow::BorrowMut;

static CUR_EXPANSION: &str = "The Shadows of Luclin";
static EQ_ITEMS_BASE: &str = "http://eqitems.com/?";

struct BisQueryMapper;

impl BisQueryMapper {
    fn get_query(race: &str, class: &str, expansion: Option<&str>) -> Result<String, String> {
        let race_code = BisQueryMapper::map_race(race)?;
        let class_code = BisQueryMapper::map_class(class)?;
        let expac_code = BisQueryMapper::map_expac(expansion)?;

        let class_query = format!("frmSearch[classId]={}", class_code);
        let race_query = format!("&frmSearch[raceId]={}", race_code);
        let expac_query = format!("&frmSearch[expansionId]={}", expac_code);

        let query_parts = vec![
            class_query.as_str(),
            race_query.as_str(),
            expac_query.as_str(),
            "&frmSearch[level]=60",
            "&frmSearch[deityId]=0",
            "&frmSearch[gearLevel]=99",
            "&frmSearch[NoDrop]=1",
            "&frmSearch[IgnoreEffects]=1",
            "&DoSearch=Search Best in Slot",
        ];

        let query: String = query_parts.join("");
        let encoded_query = percent_encode(query.as_bytes(), CONTROLS).to_string();

        Ok(format!("{}{}", EQ_ITEMS_BASE, encoded_query))
    }

    fn map_race(race: &str) -> Result<i32, String> {
        let search_race = race.to_ascii_lowercase();

        match search_race.as_str() {
            "barbarian" => Ok(1),
            "bar" => Ok(1),
            "dark-elf" => Ok(2),
            "darkelf" => Ok(2),
            "def" => Ok(2),
            "drakkin" => Ok(4),
            "dra" => Ok(4),
            "dwarf" => Ok(8),
            "dwf" => Ok(8),
            "erudite" => Ok(16),
            "eru" => Ok(16),
            "froglok" => Ok(32),
            "frg" => Ok(32),
            "gnome" => Ok(64),
            "gnm" => Ok(64),
            "half-elf" => Ok(128),
            "halfelf" => Ok(128),
            "haf" => Ok(128),
            "halfling" => Ok(256),
            "hlf" => Ok(256),
            "high-elf" => Ok(512),
            "highelf" => Ok(512),
            "hie" => Ok(512),
            "human" => Ok(1024),
            "hum" => Ok(1024),
            "iksar" => Ok(2048),
            "iks" => Ok(2048),
            "ogre" => Ok(4096),
            "ogr" => Ok(4096),
            "troll" => Ok(8192),
            "trl" => Ok(8192),
            "vah-shir" => Ok(16384),
            "vahshir" => Ok(16384),
            "vah" => Ok(16384),
            "wood-elf" => Ok(32768),
            "woodelf" => Ok(32768),
            "elf" => Ok(32768),
            _ => Err(format!("Unknown race: {}", race)),
        }
    }

    fn map_class(class: &str) -> Result<i32, String> {
        let search_class = class.to_ascii_lowercase();

        match search_class.as_str() {
            "bard" => Ok(1),
            "brd" => Ok(1),
            "beastlord" => Ok(2),
            "bst" => Ok(2),
            "berserker" => Ok(4),
            "ber" => Ok(4),
            "cleric" => Ok(8),
            "clr" => Ok(8),
            "druid" => Ok(16),
            "dru" => Ok(16),
            "enchanter" => Ok(32),
            "enc" => Ok(32),
            "magician" => Ok(64),
            "mag" => Ok(64),
            "monk" => Ok(128),
            "mnk" => Ok(128),
            "necromancer" => Ok(256),
            "nec" => Ok(256),
            "paladin" => Ok(512),
            "pal" => Ok(512),
            "ranger" => Ok(1024),
            "rng" => Ok(1024),
            "rogue" => Ok(2048),
            "rog" => Ok(2048),
            "shadow-knight" => Ok(4096),
            "shadowknight" => Ok(4096),
            "shd" => Ok(4096),
            "shaman" => Ok(8192),
            "shm" => Ok(8192),
            "warrior" => Ok(16384),
            "war" => Ok(16384),
            "wizard" => Ok(32768),
            "wiz" => Ok(32768),
            _ => Err(format!("Unknown class: {}", class)),
        }
    }

    fn map_expac(expac: Option<&str>) -> Result<i32, String> {
        let search_expac = match expac {
            Some(e) => e.to_ascii_lowercase(),
            None => CUR_EXPANSION.to_ascii_lowercase(),
        };

        match search_expac.as_str() {
            "classic" => Ok(0),
            "classic everquest" => Ok(0),
            "the ruins of kunark" => Ok(1),
            "kunark" => Ok(1),
            "the scars of velious" => Ok(2),
            "velious" => Ok(2),
            "the shadows of luclin" => Ok(3),
            "luclin" => Ok(3),
            "the planes of power" => Ok(4),
            "pop" => Ok(4),
            "the legacy of ykesha" => Ok(5),
            "ykesha" => Ok(5),
            "lost dungeons of norrath" => Ok(6),
            "ldon" => Ok(6),
            "gates of discord" => Ok(7),
            "gates" => Ok(7),
            "god" => Ok(7),
            "omens of war" => Ok(8),
            "omens" => Ok(8),
            "oow" => Ok(8),
            "dragons of norrath" => Ok(9),
            "don" => Ok(9),
            "depths of darkhollow" => Ok(10),
            "depths" => Ok(10),
            "dod" => Ok(10),
            "prophecy of ro" => Ok(11),
            "prophecy" => Ok(11),
            "por" => Ok(11),
            "the serpent's spine" => Ok(12),
            "serp" => Ok(12),
            "tss" => Ok(12),
            "the buried sea" => Ok(13),
            "tbs" => Ok(13),
            "secrets of faydwer" => Ok(14),
            "secrets" => Ok(14),
            "sof" => Ok(14),
            "seeds of destruction" => Ok(15),
            "sod" => Ok(15),
            "underfoot" => Ok(16),
            "house of thule" => Ok(17),
            "hot" => Ok(17),
            "veil of alaris" => Ok(18),
            "veil" => Ok(18),
            "voa" => Ok(18),
            "rain of fear" => Ok(19),
            "rain" => Ok(19),
            "rof" => Ok(19),
            "call of the forsaken" => Ok(20),
            "call" => Ok(20),
            "cotf" => Ok(20),
            "the darkened sea" => Ok(21),
            "darkened sea" => Ok(21),
            "tds" => Ok(21),
            "the broken mirror" => Ok(22),
            "broken mirror" => Ok(22),
            "mirror" => Ok(22),
            "tbm" => Ok(22),
            "empires of kunark" => Ok(23),
            "empires" => Ok(23),
            "eok" => Ok(23),
            "ring of scale" => Ok(24),
            "ring" => Ok(24),
            "ros" => Ok(24),
            "the burning lands" => Ok(25),
            "burning lands" => Ok(25),
            "tbl" => Ok(25),
            _ => Err(format!(
                "Unknown expansion: {}",
                expac.unwrap_or(CUR_EXPANSION)
            )),
        }
    }
}

pub struct Bis;

impl Bis {
    pub fn accept_raw(msg_parts: Vec<&str>) -> String {
        let result = if msg_parts.len() < 3 {
            return format!(
                "Usage: !bis <race> <class> <slot> [expansion] (Default expansion: {})",
                CUR_EXPANSION
            );
        } else if msg_parts.len() == 3 {
            Bis::do_search(msg_parts[0], msg_parts[1], msg_parts[2], None)
        } else if msg_parts.len() > 3 {
            let expac = msg_parts[2..].join(" ");
            Bis::do_search(
                msg_parts[0],
                msg_parts[1],
                msg_parts[2],
                Some(expac.as_str()),
            )
        } else {
            panic!("Math is broken")
        };

        match result {
            Ok(items) => Bis::format_result(items),
            Err(err) => err,
        }
    }

    fn format_result(result: Vec<(String, String, String)>) -> String {
        result
            .iter()
            .map(|(name, link, detail)| format!("{} - <{}> {}\n", name, link, detail))
            .collect()
    }

    fn do_search(
        race: &str,
        class: &str,
        slot: &str,
        expansion: Option<&str>,
    ) -> Result<Vec<(String, String, String)>, String> {
        let query = BisQueryMapper::get_query(race, class, expansion)?;

        let document = match AllaUtil::fetch_url(query.as_str()) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let search_slot = slot.to_ascii_uppercase();

        let slot_nodes: Vec<Node> = document
            .find(Name("h3"))
            .filter(|n| n.text().as_str() == search_slot.as_str())
            .collect();

        if slot_nodes.len() == 0 {
            return Err(format!("Unknown slot: {}", slot));
        } else if slot_nodes.len() > 1 {
            return Err(format!("Found multiple listings for slot: {}", slot));
        }

        let item_list = match slot_nodes[0].next() {
            Some(node) => node,
            None => return Err(format!("No item list found for slot: {}", slot)),
        };

        Ok(item_list
            .children()
            .take(3)
            .filter_map(|li| li.first_child())
            .filter_map(|div| Bis::extract_name_and_link(div.children().borrow_mut()))
            .map(|(name, link)| {
                let detail = Bis::fetch_detail(link.as_str());
                (name, link, detail)
            })
            .collect())
    }

    fn extract_name_and_link(div_children: &mut Children) -> Option<(String, String)> {
        let pairs: Vec<(String, String)> = div_children
            .filter(|n| Name("a").and(Attr("target", "_blank")).matches(n))
            .map(|n| (n.text(), n.attr("href")))
            .filter_map(|(name, maybe_link)| match maybe_link {
                None => None,
                Some(link) => Some((name, String::from(link))),
            })
            .collect();

        match pairs.get(0) {
            None => None,
            Some((name, link)) => Some((String::from(name), String::from(link))),
        }
    }

    fn fetch_detail(link: &str) -> String {
        let document = match AllaUtil::fetch_url(link) {
            Ok(d) => d,
            Err(e) => return e,
        };

        let quest_node: Vec<(String, String)> = document
            .find(Name("a"))
            .filter(|n| n.text().contains("/db/quest.html"))
            .filter(|n| n.attr("href").is_some())
            .map(|n| (n.text(), String::from(n.attr("href").unwrap())))
            .collect();

        match quest_node.get(0) {
            Some((_quest, link)) => return format!("quested: <{}>", link),
            None => {}
        };

        let drop_nodes: Option<Node> = document.find(Name("div").and(Attr("id", "drops"))).next();

        let drop_mob_div_children = match drop_nodes {
            None => return String::from("Couldn't determine source"),
            Some(n) => n.children(),
        };

        let mobs: Vec<String> = drop_mob_div_children
            .filter(|n| Name("ul").matches(n))
            .map(|n| n.children())
            .map(|c| c.filter(|n| Name("li").matches(n)))
            .flat_map(|f| f.map(|c| c.children()))
            .filter_map(|mut c| c.find(|n| Name("a").matches(n)))
            .map(|n| n.text())
            .filter(|t| !t.contains("The Fabled"))
            .collect();

        println!("Droppers: {}", mobs.join(", "));

        let found_mob = match mobs.len() {
            0 => String::from("unknown mob"),
            1 => String::from(mobs.get(0).unwrap()),
            _ => String::from("multiple mobs"),
        };

        let drop_zone_div_children = drop_nodes.unwrap().children();

        let zones: Vec<String> = drop_zone_div_children
            .filter(|n| Name("b").matches(n))
            .map(|n| n.text())
            .collect();

        let found_zone = match zones.len() {
            0 => String::from("unknown zone"),
            1 => String::from(zones.get(0).unwrap()),
            _ => String::from("multiple zones"),
        };

        return format!("drops from {} in {}", found_mob, found_zone);
    }
}
