use merge_yaml_hash::MergeYamlHash;
use yaml_rust::yaml::Hash;
use yaml_rust::yaml::Yaml;

pub struct Config(Hash);

pub fn load() -> Config {
    let mut hash = MergeYamlHash::new();

    hash.merge(include_str!("config/default.yml"));
    // hash.merge("cherry:\n  sweet: 1\n  tart: 2");

    Config(hash.data)
}

impl Config {
    pub fn get_array(&self, cop: &str, key: &str) -> Vec<String> {
        let cop_config = &self.0[&Yaml::String(cop.to_string())];
        let mut output: Vec<String> = vec![];

        match &cop_config[key] {
            Yaml::Array(array) => {
                for item in array {
                    if let Yaml::String(string) = item {
                        output.push(string.clone());
                    } else {
                        panic!("item has to be string");
                    }
                }
            }
            Yaml::BadValue => {
                return self.get_array("AllCops", key);
            }
            _ => (),
        }

        output
    }

    pub fn is_enabled(&self, cop: &str) -> bool {
        let cop_config = &self.0[&Yaml::String(cop.to_string())];
        if let Yaml::Boolean(enabled) = &cop_config["Enabled"] {
            return *enabled;
        } else {
            panic!("Enabled field not found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn test_get_array() {
        let config = load();

        assert_eq!(
            config.get_array("Bundler/DuplicatedGem", "Include"),
            vec![
                String::from("**/*.gemfile"),
                String::from("**/Gemfile"),
                String::from("**/gems.rb")
            ]
        );
    }

    #[test]
    fn test_is_enabled() {
        let config = load();

        assert!(config.is_enabled("Bundler/DuplicatedGem"));
        assert!(!config.is_enabled("Bundler/GemComment"));
    }
}
