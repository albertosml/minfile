use regex::Regex;

/// Minify the content obtained from a markup language file like XML or HTML.
/// 
/// # Arguments
/// 
/// * `content` - A string that contains the content of a markup language file.
/// 
/// # Example
/// 
/// ```
/// use minfile::minify_markup_language::minify;
/// let content_minified: String = minify("<class><element   /> </class>");
/// assert_eq!(content_minified, "<class><element/></class>");
/// ```
pub fn minify(content: &str) -> String {
    // Set the regular expression that takes all kind of
    // whitespaces and all non word boundaries.
    let regular_expression: &str = r"\s\B";

    // Create the regex object with the previous regular expression.
    let re: Regex = Regex::new(regular_expression).unwrap();

    // To minify an HTML or XML file, we will need to remove all the
    // whitespaces and non word boundaries.
    return re.replace_all(content, "").to_string();
}

// Unit tests for minify_markup_language function.
mod tests {
    // Import minify function for the tests.
    use super::minify;

    #[test]
    fn test_basic_xml() {
        // Simulate file content.
        let content: &str = "<xml> <element/></xml>";

        // Run function for the content.
        let result: String = minify(content);

        // Set desired output.
        let expected_output: &str = "<xml><element/></xml>";

        // Assert result.
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_basic_html() {
        // Simulate file content.
        let content: &str = "
            <html>
                <head>
                    <style>
                        <meta charset=\"UTF-8\">
                    </style>
                </head>
                <body/>
            </html>
        ";

        // Run function for the content.
        let result: String = minify(content);

        // Set desired output.
        let expected_output: &str = "<html><head><style><meta charset=\"UTF-8\"></style></head><body/></html>";

        // Assert result.
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_complex_xml() {
        // Simulate file content.
        let content: &str = "
            <?xml version=\"1.0\"        encoding=\"UTF-8\" ?>
            <!---An XML comment---->

            <comments>
                <comment>
                    <description>The best comment</description>
                        </comment>
                
                <comment><description>A comment</description></comment>
                    <comment>
                            <description/>
                </comment>
            </comments>
        ";

        // Run function for the content.
        let result: String = minify(content);

        // Set desired output.
        let expected_output: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><!---An XML comment----><comments><comment><description>The best comment</description></comment><comment><description>A comment</description></comment><comment><description/></comment></comments>";

        // Assert result.
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_complex_html() {
        // Simulate file content.
        let content: &str = "  <html>

            <head>
                <style>a { pointer-events: none; }</style>
                </head>
                    <body>
                        <div>    <a href=\"www.as.com\" style=\"background-color: red;\" id=\"link-1\" /></div>
                            </body>
                </head>
            </html>
        ";

        // Run function for the content.
        let result: String = minify(content);

        // Set desired output.
        let expected_output: &str = "<html><head><style>a{ pointer-events: none;}</style></head><body><div><a href=\"www.as.com\" style=\"background-color: red;\" id=\"link-1\"/></div></body></head></html>";

        // Assert result.
        assert_eq!(result, expected_output);
    }
}