pub fn mean(data: &Vec<u64>) -> f64 {
    data.iter().sum::<u64>() as f64 / data.len() as f64
}

pub fn median(data: &mut Vec<u64>) -> f64 {
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] as f64 + data[mid] as f64) / 2.0
    } else {
        data[mid] as f64
    }
}

pub fn std_deviation(data: &Vec<u64>, mean: f64) -> f64 {
    let variance = data
        .iter()
        .map(|&value| {
            let diff = mean - (value as f64);
            diff * diff
        })
        .sum::<f64>()
        / data.len() as f64;

    variance.sqrt()
}

pub fn collect_data<F: Fn() -> u64>(func: F, trails: usize) -> Vec<u64> {
    (0..trails).map(|_| func()).collect()
}

pub mod structs {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Person {
        name: String,
        age: u8,
        bank_account: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct ApacheBuilds {
        #[serde(rename = "numExecutors")]
        num_executors: u32,
        description: String,
        jobs: Vec<Job>,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Job {
        name: String,
        url: String,
        color: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Log {
        jk_host: String,
        class_name: String,
        logger_name: String,
        cgi_tte_ms: String,
        start_timestamp: u64,
        user_agent_device: String,
        slush: String,
        and_an_ip4: String,
        #[serde(rename = "@version")]
        at_version: String,
        error_url_path: String,
        logstash: String,
        #[serde(rename = "uuids->")]
        uuids_arrow: String,
        anotherfilename: String,
        environment: String,
        floatasstr: String,
        #[serde(rename = "there_string:")]
        there_string_colon: String,
        arry: Vec<String>,
        message: String,
        argh: String,
        oh_my_files: String,
        user_agent_os: String,
        error_host: String,
        application: String,
        yam_message: String,
        user_agent_browser: String,
        error_url: String,
        short_message: String,
        action: String,
        #[serde(rename = "cakes!")]
        cakes_exclamation: String,
        #[serde(rename = "type")]
        type_field: String,
        log_level: String,
        too_many_ho: String,
        controller: String,
        #[serde(rename = "key_keykeykey")]
        key_keykeykey_field: String,
        #[serde(rename = "a proper_timestamp_ja")]
        a_proper_timestamp_ja: String,
        and_yet_another: String,
        #[serde(rename = "@timestamp")]
        at_timestamp: String,
        level: u8,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct GithubEvent {
        #[serde(rename = "type")]
        pub event_type: String,
        pub created_at: String,
        pub actor: Actor,
        pub repo: Repo,
        #[serde(rename = "public")]
        pub is_public: bool,
        pub payload: Payload,
        pub id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Actor {
        pub gravatar_id: String,
        pub login: String,
        pub avatar_url: String,
        pub url: String,
        pub id: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Repo {
        pub url: String,
        pub id: i32,
        pub name: String,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Payload {
        pub commits: Vec<Commit>,
        pub distinct_size: i32,
        #[serde(rename = "ref")]
        pub _ref: String,
        pub push_id: i32,
        pub head: String,
        pub before: String,
        pub size: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Commit {
        pub url: String,
        pub message: String,
        pub distinct: bool,
        pub sha: String,
        pub author: Author,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Author {
        pub email: String,
        pub name: String,
    }
}
