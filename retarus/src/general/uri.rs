
#[derive(Debug, Clone, PartialEq)]
pub enum Region {
    Europe,
    America,
    Switzerland,
    Singapore,
    Custom(String)
}

pub fn determine_region_uri(region: Region) -> RegionUri {
    match region {
        Region::Europe => {
            RegionUri::new(Region::Europe, "https://faxws-ha.de.retarus.com", vec!["https://faxws.de2.retarus.com", "https://faxws.de1.retarus.com"])
        }
        Region::America => {
            RegionUri::new(Region::America, "https://faxws-ha.us.retarus.com", vec!["https://faxws.us2.retarus.com", "https://faxws.us1.retarus.com"])
        }
        Region::Switzerland => {
            RegionUri::new(Region::Switzerland, "https://faxws-ha.ch.retarus.com", vec!["https://faxws.ch1.retarus.com"])
        }
        Region::Singapore => {
            RegionUri::new(Region::Singapore, "https://faxws.sg1.retarus.com", vec!["https://faxws.sg1.retarus.com"])
        }
        Region::Custom(data) => {
            RegionUri::new(Region::Custom(String::new()), "", vec![data.as_str()])
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegionUri {
    pub region: Region,
    pub ha_addr: String,
    pub servers: Vec<String>
}
impl RegionUri {
    pub fn new(region: Region, ha_addr: &str, servers: Vec<&str>) -> RegionUri {
        let mut c = vec![];
        for x in servers.iter(){
            c.push(x.to_owned().to_string());
        }
        RegionUri {
            region,
            ha_addr: String::from(ha_addr),
            servers:c
        }
    }
}