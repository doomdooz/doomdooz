use crate::types;

lazy_static! {
    pub static ref COP: types::Cop<'static> = types::Cop {
        cop_name: "Default",
        enabled: false,
        description: "Default Cop used for Settings",
        style_guide: "",
        supported_styles: None,
        include: Some(vec![
            String::from("**/*.rb"),
            String::from("**/*.arb"),
            String::from("**/*.axlsx"),
            String::from("**/*.builder"),
            String::from("**/*.fcgi"),
            String::from("**/*.gemfile"),
            String::from("**/*.gemspec"),
            String::from("**/*.god"),
            String::from("**/*.jb"),
            String::from("**/*.jbuilder"),
            String::from("**/*.mspec"),
            String::from("**/*.opal"),
            String::from("**/*.pluginspec"),
            String::from("**/*.podspec"),
            String::from("**/*.rabl"),
            String::from("**/*.rake"),
            String::from("**/*.rbuild"),
            String::from("**/*.rbw"),
            String::from("**/*.rbx"),
            String::from("**/*.ru"),
            String::from("**/*.ruby"),
            String::from("**/*.spec"),
            String::from("**/*.thor"),
            String::from("**/*.watchr"),
            String::from("**/.irbrc"),
            String::from("**/.pryrc"),
            String::from("**/.simplecov"),
            String::from("**/buildfile"),
            String::from("**/Appraisals"),
            String::from("**/Berksfile"),
            String::from("**/Brewfile"),
            String::from("**/Buildfile"),
            String::from("**/Capfile"),
            String::from("**/Cheffile"),
            String::from("**/Dangerfile"),
            String::from("**/Deliverfile"),
            String::from("**/Fastfile"),
            String::from("**/*Fastfile"),
            String::from("**/Gemfile"),
            String::from("**/Guardfile"),
            String::from("**/Jarfile"),
            String::from("**/Mavenfile"),
            String::from("**/Podfile"),
            String::from("**/Puppetfile"),
            String::from("**/Rakefile"),
            String::from("**/rakefile"),
            String::from("**/Snapfile"),
            String::from("**/Steepfile"),
            String::from("**/Thorfile"),
            String::from("**/Vagabondfile"),
            String::from("**/Vagrantfile"),
        ]),
        exclude: Some(vec![
            String::from("node_modules/**/*"),
            String::from("tmp/**/*"),
            String::from("vendor/**/*"),
            String::from(".git/**/*"),
        ]),
        parent_config: None,
    };
}
