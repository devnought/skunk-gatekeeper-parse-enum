#![allow(dead_code)]

use serde::Deserialize;
use time::OffsetDateTime;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Citizen {
    moniker: String,
    handle: String,
    title: Title,
    #[serde(with = "time::serde::iso8601")]
    enlisted: OffsetDateTime,
    citizen_record_number: Option<u64>,
    avatar: Url,
    #[serde(default)]
    location: Vec<String>,
    #[serde(default)]
    fluency: Vec<String>,
    website: Option<String>,
    #[serde(default)]
    bio: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Title {
    icon: Url,
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Organization {
    Visible {
        logo: Box<Url>,
        name: String,
        sid: String,
        url: Box<Url>,
        rank: OrganizationRank,
    },
    Redacted,
}

#[derive(Debug, Deserialize)]
struct OrganizationRank {
    name: String,
    value: u8,
}

#[derive(Debug, Deserialize)]
struct Organizations {
    main: Option<Organization>,
    #[serde(default)]
    affiliates: Vec<Organization>,
}

#[derive(Debug, Deserialize)]
struct CompositRecord {
    citizen: Citizen,
    organizations: Option<Organizations>,
}

fn main() {
    const INPUT: &str = r###"
    {
        "citizen": {
            "moniker": "BlurtedNonsense",
            "handle": "BlurtedNonsense",
            "title": {
                "icon": "https://robertsspaceindustries.com/media/ucku5oae1s5z1r/heap_thumb/Freelancer.png",
                "value": "Freelancer"
            },
            "enlisted": "+002014-02-14T00:00:00.000000000Z",
            "citizenRecordNumber": 389589,
            "avatar": "https://robertsspaceindustries.com/media/770mxs40v4crdr/heap_infobox/IMG_0657.jpg",
            "fluency": [
                "English"
            ],
            "website": "https://robertsspaceindustries.com/citizens/BlurtedNonsense",
            "bio": [
                "#4290"
            ]
        },
        "organizations": {
            "main": {
                "visible": {
                    "logo": "https://robertsspaceindustries.com/media/ec28s9hd9b3msr/heap_infobox/7HILLS-Logo.png",
                    "name": "7Hills Industries",
                    "sid": "7HILLS",
                    "url": "https://robertsspaceindustries.com/orgs/7HILLS",
                    "rank": {
                        "name": "MIA",
                        "value": 0
                    }
                }
            },
            "affiliates": [
                {
                    "visible": {
                        "logo": "https://robertsspaceindustries.com/media/sk91buzpgv0npr/heap_infobox/MOONDUST-Logo.png",
                        "name": "Moondust Logistics",
                        "sid": "MOONDUST",
                        "url": "https://robertsspaceindustries.com/orgs/MOONDUST",
                        "rank": {
                            "name": "Junior",
                            "value": 0
                        }
                    }
                },
                {
                    "visible": {
                        "logo": "https://robertsspaceindustries.com/media/27hn6v23ow35sr/heap_infobox/BONDSMANS-Logo.png",
                        "name": "Bounty Hunters Guild",
                        "sid": "BONDSMANS",
                        "url": "https://robertsspaceindustries.com/orgs/BONDSMANS",
                        "rank": {
                            "name": "Beroya ┃Bounty Hunter",
                            "value": 1
                        }
                    }
                },
                "redacted",
                {
                    "visible": {
                        "logo": "https://robertsspaceindustries.com/media/2br0pyrcdlplxr/heap_infobox/LAMP-Logo.png",
                        "name": "L.A.M.P.",
                        "sid": "LAMP",
                        "url": "https://robertsspaceindustries.com/orgs/LAMP",
                        "rank": {
                            "name": "Fleet Member",
                            "value": 3
                        }
                    }
                }
            ]
        }
    }
    "###;

    let record: CompositRecord = serde_json::from_str(INPUT).unwrap();

    println!("{record:#?}");
}
