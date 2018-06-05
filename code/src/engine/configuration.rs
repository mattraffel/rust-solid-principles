//!
#![allow(non_snake_case)]
use preferences::{AppInfo, PreferencesMap, Preferences, prefs_base_dir};

pub struct Configuration {
    data_source : String,
    computation : String,
    pay_this_amount : i32
}

impl Configuration {
    pub fn new() -> Self {
        return Configuration {
            data_source : "memory".to_string(),
            computation : "max".to_string(),
            pay_this_amount : 13
        };
    }

    pub fn from_file() -> Self {

        let APPINFO = AppInfo{name: "rust_solid", author: "matt"};
        let config_location = "v1";
        let load_result = PreferencesMap::<String>::load(&APPINFO, config_location);

        if load_result.is_err() {
            println!("loading default configuration");
            return Configuration::new();
        }

        let config_data = load_result.unwrap();
        let data_source = config_data.get("data_source").unwrap().to_string();
        let computation = config_data.get("computation").unwrap().to_string();
        let pay_this_amount = config_data.get("pay_this_amount").unwrap().parse::<i32>().unwrap();;

        return Configuration {
            data_source,
            computation,
            pay_this_amount
        };
    }

    // #[cfg(Debug)]
    pub fn save_config_file() {
        let APPINFO = AppInfo{name: "~/rust_solid", author: "matt"};
        let config_location = "v1";

        let mut data: PreferencesMap<String> = PreferencesMap::new();
        data.insert("data_source".to_string(), "memory".to_string());
        data.insert("computation".to_string(), "max".to_string());
        data.insert("pay_this_amount".to_string(), "13".to_string());

        let save_result = data.save(&APPINFO, config_location);
        println!("result of save is: {:?}", save_result);
        assert!(save_result.is_ok());

        println!("config saved to {:?}", prefs_base_dir().unwrap());
    }

    pub fn get_data_source_name(&self) -> String {
        return self.data_source;
    }

    pub fn get_computation_name(&self) -> String {
        return self.computation;
    }

    pub fn get_payment_amount(&self) -> i32 {
        return self.pay_this_amount;
    }
}

#[cfg(test)]
mod configuration_tests{

    use preferences::{AppInfo, PreferencesMap, Preferences};

    #[test]
    fn save_config_file() {
        let APPINFO = AppInfo{name: "preferences", author: "raffel"};
        let config_location = "config";

        let mut data: PreferencesMap<String> = PreferencesMap::new();
        data.insert("data_source".to_string(), "memory".to_string());
        data.insert("computation".to_string(), "max".to_string());
        data.insert("pay_this_amount".to_string(), "13".to_string());

        let save_result = data.save(&APPINFO, config_location);
        println!("result of save is: {:?}", save_result);
        assert!(save_result.is_ok());
    }
}