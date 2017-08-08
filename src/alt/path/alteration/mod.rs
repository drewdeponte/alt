extern crate regex;

use self::regex::Regex;

pub fn strip_test_words(filename: &String) -> String {
    let re = Regex::new(r"(test_)?(?P<p>\w+?)(_rake_spec|_spec|_rake_test|_test|_steps|Tests|UITests|Specs|UISpecs|Test|Spec|Suite)?(\.\w+)?$").unwrap();
    re.replace_all(filename.as_str(), "$p")
}

#[cfg(test)]
mod tests {
    use super::strip_test_words;

    // Groovy

    #[test]
    fn strip_test_words_returns_filename_with_groovy_test_words_stripped() {
        let s = String::from("VehicleAttributesVehicleSpec.groovy");
        assert_eq!(strip_test_words(&s), "VehicleAttributesVehicle");
    }

    // Swift XCTest

    #[test]
    fn strip_test_words_returns_filename_with_swift_xctest_test_words_stripped() {
        let s = String::from("VehicleAttributesVehicleTests.swift");
        assert_eq!(strip_test_words(&s), "VehicleAttributesVehicle");
    }

    #[test]
    fn strip_test_words_returns_filename_with_swift_xcui_test_words_stripped() {
        let s = String::from("VehicleAttributesVehicleUITests.swift");
        assert_eq!(strip_test_words(&s), "VehicleAttributesVehicle");
    }

    // Swift Quick

    #[test]
    fn strip_test_words_returns_filename_with_swift_quick_test_words_stripped() {
        let s = String::from("VehicleAttributesVehicleSpecs.swift");
        assert_eq!(strip_test_words(&s), "VehicleAttributesVehicle");
    }

    #[test]
    fn strip_test_words_returns_filename_with_swift_quickui_test_words_stripped() {
        let s = String::from("VehicleAttributesVehicleUISpecs.swift");
        assert_eq!(strip_test_words(&s), "VehicleAttributesVehicle");
    }

    // Ruby RSpec (including Rails & Hanami)

    #[test]
    fn strip_test_words_returns_filename_with_ruby_rspec_test_words_stripped() {
        let s = String::from("create_spec.rb");
        assert_eq!(strip_test_words(&s), "create");
    }

    // Ruby Rake RSpec

    #[test]
    fn strip_test_words_returns_filename_with_rake_rspec_test_words_stripped() {
        let s = String::from("foo_rake_spec.rb");
        assert_eq!(strip_test_words(&s), "foo");
    }

    // Ruby Cucumber

    #[test]
    fn strip_test_words_returns_filename_with_ruby_cucumber_test_words_stripped() {
        let s = String::from("project_management_steps.rb");
        assert_eq!(strip_test_words(&s), "project_management");
    }

    // Ruby Minitest

    #[test]
    fn strip_test_words_returns_filename_with_ruby_minitest_test_words_stripped() {
        let s = String::from("tasks_controller_test.rb");
        assert_eq!(strip_test_words(&s), "tasks_controller");
    }

    // Ruby Rake Minitest

    #[test]
    fn strip_test_words_returns_filename_with_rake_minitest_test_words_stripped() {
        let s = String::from("foo_rake_test.rb");
        assert_eq!(strip_test_words(&s), "foo");
    }

    // Elixer ExUnit

    #[test]
    fn strip_test_words_returns_filename_with_elixer_exunit_test_words_stripped() {
        let s = String::from("supervisor_test.exs");
        assert_eq!(strip_test_words(&s), "supervisor");
    }

    // JavaScript Jasmine

    #[test]
    fn strip_test_words_returns_filename_with_jasmine_test_words_stripped() {
        let s = String::from("jacked_spec.js");
        assert_eq!(strip_test_words(&s), "jacked");
    }

    // Python

    #[test]
    fn strip_test_words_returns_filename_with_python_test_words_stripped() {
        let s = String::from("test_toaster.py");
        assert_eq!(strip_test_words(&s), "toaster");
    }

    // Java Maven JUnit

    #[test]
    fn strip_test_words_returns_filename_with_junit_test_words_stripped() {
        let s = String::from("SomethingTest.java");
        assert_eq!(strip_test_words(&s), "Something");
    }

    // Scala ScalaTest

    #[test]
    fn strip_test_words_returns_filename_with_scalatest_test_word_stripped() {
        let s = String::from("SomethingTest.scala");
        assert_eq!(strip_test_words(&s), "Something");
    }

    #[test]
    fn strip_test_words_returns_filename_with_scalatest_spec_word_stripped() {
        let s = String::from("SomethingSpec.scala");
        assert_eq!(strip_test_words(&s), "Something");
    }

    #[test]
    fn strip_test_words_returns_filename_with_scalatest_suite_word_stripped() {
        let s = String::from("SomethingSuite.scala");
        assert_eq!(strip_test_words(&s), "Something");
    }
}
