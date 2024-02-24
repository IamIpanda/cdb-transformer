use crate::{card::CardTransformer, constants::Type};

use super::{CDB, STR_FIELD_NAMES};

pub struct SQL;

pub const CREATE_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS datas(id integer primary key,ot integer,alias integer,setcode integer,type integer,atk integer,def integer,level integer,race integer,attribute integer,category integer);
CREATE TABLE IF NOT EXISTS texts(id integer primary key,name text,desc text,str1 text,str2 text,str3 text,str4 text,str5 text,str6 text,str7 text,str8 text,str9 text,str10 text,str11 text,str12 text,str13 text,str14 text,str15 text,str16 text);
";



impl CardTransformer for SQL {
    fn to_string(card: &crate::card::Card) -> String {
        let level = card.level + if card._type.contains(Type::Pendulum) { card.lscale<<16 + card.rscale<<24 } else {0};
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


#[test]
fn test_format() {
	let cards = super::CDB::from_string("/Users/iami/Workshop/code/mycard/ygopro-database/locales/zh-CN/cards.cdb");
	let s = cards.into_iter().map(|c| SQL::to_string(&c)).collect::<Vec<_>>().join("\n\n");
	std::fs::write("/Users/iami/Workshop/code/mycard/cdb-transformer/test2.log", s).unwrap();
}

#[test]
fn test_parse() {
	let cards = std::fs::read_to_string("/Users/iami/Workshop/code/mycard/cdb-transformer/test2.log").unwrap();
	let cc = SQL::from_string(&cards);
	println!("{:?}", cc.len());
}
