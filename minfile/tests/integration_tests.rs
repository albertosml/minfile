use std::fs::read_to_string;

use minfile::minify_markup_language::minify;

/// Given a file path, open it and obtain its content.
/// 
/// # Arguments
/// 
/// * `file_path` - The path of the file to obtain the content.
/// 
/// # Example
/// 
/// ```
/// obtain_file_content("my_file.rs");
/// ```
fn obtain_file_content(file_path: &str) -> String {
    // Read file content in a string.
    return read_to_string(file_path).expect("Unable to read the file.");
}

/// Prepare the filename given the directory, the file type and an attribute
/// to indicate if we need to set the input or the output file.
/// 
/// # Arguments
/// 
/// * `directory` - The directory where the file is ubicated.
/// * `file_type` - The file extension that the new filename will have.
/// * `is_input_file` - A boolean that indicate if we need to prepare a input or output file.
/// 
/// # Examples
/// 
/// ```
/// use minify::integration_tests::set_filename;
/// 
/// let file_name: String = set_filename("tests", "xml", true);
/// assert_eq!(file_name, "tests/input.xml");
/// 
/// let file_name: String = set_filename("tests", "xml", false);
/// assert_eq!(file_name, "tests/output.xml");
/// ```
fn set_filename(directory: &str, file_type: &str, is_input_file: bool) -> String {
    let mut file_name: String = String::new();

    // Add directory to the file name.
    file_name.push_str(directory);

    // Add the name of the file depending if it is an input or output file.
    if is_input_file {
        file_name.push_str("/input.");
    } else {
        file_name.push_str("/output.");
    }

    // Add the file extension.
    file_name.push_str(file_type);

    // Return the file name.
    return file_name;
}

// Set macro rule for the integration test for the minification.
macro_rules! run_integration_test {
    ($test_name: ident, $directory: expr, $file_type: expr) => {
        #[test]
        fn $test_name() {
            // Obtain directory.
            let directory: String = $directory.to_owned();
            
            // Get file type.
            let file_type: String = $file_type.to_owned();
            
            // Set the input filename for the given directory. Note that the input file 
            // will be always called "input.html".
            let input_file_name: String = set_filename(&directory, &file_type, true);
            
            // Set the output filename for the given directory. Note that the output file 
            // will be always called "output.html".
            let output_file_name: String = set_filename(&directory, &file_type, false);

            // Receive content from the input file.
            let input_content: String = obtain_file_content(&input_file_name);

            // Receive content from the output file.
            let expected_output_content: String = obtain_file_content(&output_file_name);

            // Minify input file.
            let result: String = minify(&input_content);

            // Compare the obtained and the expected result.
            assert_eq!(result, expected_output_content);
        }
    };
}

// Run the test for the HTML case.
run_integration_test!(test_integration_html, "tests/html", "html");

// Run the test for the XML case.
run_integration_test!(test_integration_xml, "tests/xml", "xml");