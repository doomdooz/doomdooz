use regex::Regex;

use crate::cop;
use crate::source;
use crate::types;
use crate::CONFIG;

// https://github.com/rubocop/rubocop/blob/master/lib/rubocop/cop/layout/leading_comment_space.rb
static MSG: &str = "Missing space after `#`.";
static COP_NAME: &str = "Layout/LeadingCommentSpace";

lazy_static! {
    static ref RE_COMMENT: Regex = Regex::new(r"\A([#]+)[^#\s=+-]").unwrap();
}

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    let path = &file.filepath;

    for comment in &file.parser_result.comments {
        let loc = comment.location;
        let line_col = file.line_col(loc.begin).unwrap();
        let text = file.source(&loc);
        let mut postfix_offset = 0;

        if !is_comment(&mut postfix_offset, text) {
            continue;
        }

        if line_col.0 == 1 && is_allowed_on_first_line(&mut postfix_offset, text, path) {
            continue;
        }
        if is_doxygen_comment_style(&mut postfix_offset, text) {
            continue;
        }
        if is_gemfile_ruby_comment(&mut postfix_offset, text, path) {
            continue;
        }

        file.add_offense(COP_NAME, loc, MSG);

        let mut new_text = text.to_string();
        new_text.replace_range(postfix_offset + 1..postfix_offset + 1, " ");

        let correction = types::Correction::replace(loc, &new_text);
        file.add_correction(correction);
    }
}

fn is_comment(offset: &mut usize, text: &str) -> bool {
    if let Some(g) = RE_COMMENT.captures(text) {
        let mark = g.get(1).map_or("", |m| m.as_str());

        *offset = mark.len() - 1;
        return true;
    }

    false
}

fn is_allowed_on_first_line(offset: &mut usize, text: &str, path: &str) -> bool {
    if text.starts_with("#!") {
        *offset = 1;
        return true;
    }

    if path.ends_with("config.ru") && text.starts_with(r#"#\"#) {
        *offset = 2;
        return true;
    }

    false
}

fn is_doxygen_comment_style(offset: &mut usize, text: &str) -> bool {
    if is_enabled("AllowDoxygenCommentStyle") && text.starts_with("#*") {
        *offset = 2;
        return true;
    }

    false
}

fn is_gemfile_ruby_comment(offset: &mut usize, text: &str, path: &str) -> bool {
    if is_enabled("AllowGemfileRubyComment")
        && path.ends_with("Gemfile")
        && text.starts_with("#ruby")
    {
        *offset = 5;
        return true;
    }

    false
}

fn is_enabled(key: &str) -> bool {
    CONFIG.get_bool(COP_NAME, key)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        expect_offense! {"
        test
        #missing space
        ^^^^^^^^^^^^^^ Missing space after `#`.
        "};
        expect_correction!("test #missing space", "test # missing space");
        expect_correction!("test ##missing space", "test ## missing space");

        // does not register an offense
        expect_no_offense!("#");
        expect_no_offense!("#   heavily indented");
        expect_no_offense!("###### heavily indented");
        expect_no_offense!("######");
        expect_no_offense! {"
        #!/usr/bin/ruby
        test
        "};

        // registers an offense and corrects #! after the first line
        expect_offense! {"
        test
        #!/usr/bin/ruby
        ^^^^^^^^^^^^^^^ Missing space after `#`.
        "};

        expect_correction! {
        "
        test
        #!/usr/bin/ruby
        ",
        "
        test
        # !/usr/bin/ruby
        "
        };
    }

    #[test]
    fn test_named_config_ru() {
        // does not register an offense for #\ on first line
        expect_no_offense! {"config.ru",
        r#"
        #\ -w -p 8765
        test
        "#
        };

        // registers an offense and corrects for #\ after the first line
        expect_offense! {"config.ru", r#"
        test
        #\ -w -p 8765
        ^^^^^^^^^^^^^ Missing space after `#`.
        "#}

        expect_correction! {"config.ru", r#"
        test
        #\ -w -p 8765
        "#, r#"
        test
        # \ -w -p 8765
        "#}
    }

    #[test]
    fn test_not_named_config_ru() {
        // registers an offense and corrects #\ on first line
        expect_no_offense! {"config.ru",
        r#"
        #\ -w -p 8765
        test
        "#
        };

        expect_correction! {r#"
        #\ -w -p 8765
        test
        "#, r#"
        # \ -w -p 8765
        test
        "#}

        // registers an offense and corrects #\ after the first line
        expect_offense! {r#"
        test
        #\ -w -p 8765
        ^^^^^^^^^^^^^ Missing space after `#`.
        "#}

        expect_correction! {r#"
        test
        #\ -w -p 8765
        "#, r#"
        test
        # \ -w -p 8765
        "#}
    }

    #[test]
    fn test_doxygen_style() {
        // does not register offense when using Doxygen style
        expect_no_offense! {"
        #**
        # Some comment
        # Another comment on a second line
        #*
        "};
    }

    #[test]
    fn test_rdoc_syntax() {
        // does not register an offense when using `#++` or `#--`
        expect_no_offense! {"
        #++
        #--
        test
        "};

        // registers an offense when starting `:`
        expect_offense! {"
        test
        #:nodoc:
        ^^^^^^^^ Missing space after `#`.
        "};

        // accepts sprockets directives
        expect_no_offense! {"#= require_tree ."}

        // accepts =begin/=end comments
        expect_no_offense! {"
        =begin
        #blahblah
        =end
        "};
    }

    #[test]
    fn test_gemfile_ruby_commment() {
        expect_correction! {"path/to/Gemfile", "
        # Specific version (comment) will be used by RVM
        #ruby=2.7.0
        #ruby-gemset=myproject
        ruby '~> 2.7.0'
        ", "
        # Specific version (comment) will be used by RVM
        # ruby=2.7.0
        # ruby-gemset=myproject
        ruby '~> 2.7.0'
        "}

        // TODO: Mock AllowGemfileRubyComment to test that it is enabled
    }
}
