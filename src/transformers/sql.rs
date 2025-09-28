use crate::{card::CardTransformer, constants::Type};

#[cfg(not(target_arch = "wasm32"))]
use crate::transformers::CDB;

pub struct SQL;

pub const STR_FIELD_NAMES: [&str; 16] = ["str1","str2","str3","str4","str5","str6","str7","str8","str9","str10","str11","str12","str13","str14","str15","str16"];
pub const CREATE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS datas(id integer primary key,ot integer,alias integer,setcode integer,type integer,atk integer,def integer,level integer,race integer,attribute integer,category integer);
CREATE TABLE IF NOT EXISTS texts(id integer primary key,name text,desc text,str1 text,str2 text,str3 text,str4 text,str5 text,str6 text,str7 text,str8 text,str9 text,str10 text,str11 text,str12 text,str13 text,str14 text,str15 text,str16 text);
";

impl CardTransformer for SQL {
    fn to_string(card: &crate::card::Card) -> String {
        let level = card.level + (if card._type.contains(Type::Pendulum) { (card.lscale<<16) + (card.rscale<<24) } else {0});
        let defense = if card._type.contains(Type::Link) { card.link_marker.bits() } else { card.defense };
        let text_keys = STR_FIELD_NAMES[0..16].into_iter().map(|s| format!(",{}",s)).collect::<Vec<_>>().join("");
        let mut text_descs = card.texts.iter().map(|s| format!(",'{}'", s.replace("'", "''"))).collect::<Vec<_>>();
        while text_descs.len() < 16 { text_descs.push(",''".to_string()) }
        format!("INSERT OR REPLACE INTO datas(id, ot,alias,setcode,type,atk,def,level,race,attribute,category) values({},{},{},{},{},{},{},{},{},{},{});\n",
                card.code,card.ot.bits(),card.alias,card.setcode,card._type.bits(),card.attack,defense,level,card.race.bits(),card.attribute.bits(),0)+
        &format!("INSERT OR REPLACE INTO texts(id,name,desc{}) values({},'{}','{}'{});",
                text_keys,card.code,card.name.replace("'", "''"),card.desc.replace("'", "''"),
                text_descs.join(""))
    }

    fn from_string(str: &str) -> Vec<crate::card::Card> {
        #[cfg(target_arch = "wasm32")]
        unimplemented!("Sqlite is disabled.");
        #[cfg(not(target_arch = "wasm32"))]
        {
            let connection = sqlite::open(":memory:").expect("Cannot open sqlite memory instance");
            connection.execute(CREATE_TABLE_SQL).expect("create table failed");
            // for (n,line) in str.split("\n").into_iter().enumerate() {
            //     if let Err(e) = connection.execute(line) {
            //         println!("Failed to execute on line {} '{}': {}", n, line, e)
            //     }
            // }
            connection.execute(str).expect("execute sql failed");
            CDB::from_connection(connection)
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::path::Path;

    use crate::card::CardTransformer;
    use crate::transformers::*;

    #[test]
    fn test_format() {
        let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/transformers/test_data/xyyz.txt");
        let text = std::fs::read_to_string(file).expect("Failed to read test file");
        let cards = Xyyz::from_string(&text);
        for card in cards {
            println!("{:?}", SQL::to_string(&card))
        }
    }

    #[test]
    fn test_parse() {
        let path_sql = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/transformers/test_data/xyyz.txt");
        let sql = read_to_string(path_sql).expect("Failed to read text sql file");
        let cards = SQL::from_string(&sql);
        for card in cards {
            println!("{:?}", Xyyz::to_string(&card))
        }
    }
}
