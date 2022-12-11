use crate::cop;
use crate::types;

lazy_static! {
    static ref COP: types::Cop<'static> = types::Cop {
        cop_name: "Bundler/DuplicatedGem",
        enabled: true,
        description: "Checks for duplicate gem entries in Gemfile.",
        style_guide: "",
        supported_styles: Some(vec!["compact".into(), "expanded".into()]),
        include: Some(vec![
            String::from("**/*.gemfile"),
            String::from("**/Gemfile"),
            String::from("**/gems.rb")
        ]),
        exclude: None,
        parent_config: Some(&cop::default::COP),
    };
}

pub fn init() {
    cop::register(&COP);
}
