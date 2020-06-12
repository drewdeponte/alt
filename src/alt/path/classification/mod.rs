extern crate regex;

use self::regex::Regex;

pub fn is_test_file(path: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:^(features/step_definitions/|test/|spec/|tests/|src/test/|\w*Tests/))|(?:^((\w+/)+spec/|(\w+/)+test/))|(?:^(.+\.(?:spec|test)\.\w+)$)").unwrap();
    }
    RE.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::is_test_file;

    // Swift/XCTest/XCUI/Quick

    #[test]
    fn is_test_file_detects_swift_xctest_files() {
        let s = String::from("AutomotiveTests/Vehicles/VehicleAttributesVehicleTests.swift");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_swift_xcui_files() {
        let s = String::from("AutomotiveUITests/Vehicles/VehicleAttributesVehicleUITests.swift");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_swift_quick_files() {
        let s = String::from("AutomotiveTests/Vehicles/VehicleAttributesVehicleSpec.swift");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_swift_quick_ui_files() {
        let s = String::from("AutomotiveUITests/Vehicles/VehicleAttributesVehicleUISpec.swift");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_swift_implementation_files() {
        let s = String::from("Automotive/Vehicles/VehicleAttributesVehicle.swift");
        assert_eq!(is_test_file(&s), false);
    }

    // Ruby Gem

    #[test]
    fn is_test_file_detects_ruby_gem_test_files() {
        let s = String::from("spec/foo_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_ruby_gem_test_files_under_lib() {
        let s = String::from("spec/lib/foo_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_ruby_gem_implementation_files() {
        let s = String::from("lib/foo.rb");
        assert_eq!(is_test_file(&s), false);
    }

    // Rake Tasks

    #[test]
    fn is_test_file_detects_rake_test_files() {
        let s = String::from("spec/lib/tasks/bar/foo_rake_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_rake_implementation_files() {
        let s = String::from("lib/tasks/bar/foo.rake");
        assert_eq!(is_test_file(&s), false);
    }

    // Rails

    #[test]
    fn is_test_file_does_not_detect_rails_controller_implementation_files() {
        let s = String::from("app/controllers/tasks_controller.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_rails_model_implementation_files() {
        let s = String::from("app/models/task.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_rails_helper_implementation_files() {
        let s = String::from("app/helpers/hoopty.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_rails_mailer_implementation_files() {
        let s = String::from("app/mailers/hoopty_mailer.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_rails_lib_implementation_files() {
        let s = String::from("lib/foo.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_rails_rake_implementation_files() {
        let s = String::from("bar/foo.rake");
        assert_eq!(is_test_file(&s), false);
    }

    // Rails Minitest

    #[test]
    fn is_test_file_detects_minitest_controller_test_files() {
        let s = String::from("test/controllers/tasks_controller_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_minitest_model_test_files() {
        let s = String::from("test/models/task_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_minitest_helper_test_files() {
        let s = String::from("test/helpers/hoopty_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_minitest_mailer_test_files() {
        let s = String::from("test/mailers/hoopty_mailer_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_minitest_lib_test_files() {
        let s = String::from("test/foo_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_minitest_rake_test_files() {
        let s = String::from("test/bar/foo_rake_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rails_monorepo_test_files() {
        let s = String::from("components/module1/test/app/controllers/file_controller_test.rb");
        assert_eq!(is_test_file(&s), true);
    }

    // Rails RSpec

    #[test]
    fn is_test_file_detects_rspec_controller_test_files() {
        let s = String::from("spec/controllers/tasks_controller_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rspec_model_test_files() {
        let s = String::from("spec/models/task_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rspec_helper_test_files() {
        let s = String::from("spec/helpers/hoopty_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rspec_mailer_test_files() {
        let s = String::from("spec/mailers/hoopty_mailer_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rspec_lib_test_files() {
        let s = String::from("spec/foo_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rspec_rake_test_files() {
        let s = String::from("spec/bar/foo_rake_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rails_monorepo_spec_files() {
        let s = String::from("components/module1/spec/app/controllers/file_controller_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_rails_monorepo_implementation_files() {
        let s = String::from("components/module1/app/controller/file_controller.rb");
        assert_eq!(is_test_file(&s), false);
    }

    // Hanami App Arch

    #[test]
    fn is_test_file_does_not_detect_hanami_app_controller_implementation_files() {
        let s = String::from("app/controllers/users/create.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_app_view_implementation_files() {
        let s = String::from("app/views/users/create.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_app_lib_implementation_files() {
        let s = String::from("lib/foo/bar/car/my_lib.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_app_general_implementation_files() {
        let s = String::from("app/fulfiller.rb");
        assert_eq!(is_test_file(&s), false);
    }

    // Hanami App Arch RSpec

    #[test]
    fn is_test_file_detects_hanami_app_rspec_controller_test_files() {
        let s = String::from("spec/controllers/users/create_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_app_rspec_view_test_files() {
        let s = String::from("spec/views/users/create_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_app_rspec_lib_test_files() {
        let s = String::from("spec/foo/bar/car/my_lib_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_app_rspec_general_test_files() {
        let s = String::from("spec/fulfiller_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    // Hanami Container Arch

    #[test]
    fn is_test_file_does_not_detect_hanami_container_controller_implementation_files() {
        let s = String::from("apps/web/controllers/users/create.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_container_view_implementation_files() {
        let s = String::from("apps/web/views/users/create.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_container_lib_implementation_files() {
        let s = String::from("lib/foo/bar/car/my_lib.rb");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_does_not_detect_hanami_container_general_implementation_files() {
        let s = String::from("apps/offer_service/fulfiller.rb");
        assert_eq!(is_test_file(&s), false);
    }

    // Hanami Container Arch RSpec

    #[test]
    fn is_test_file_detects_hanami_container_rspec_controller_test_files() {
        let s = String::from("spec/web/controllers/users/create_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_container_rspec_view_test_files() {
        let s = String::from("spec/web/views/users/create_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_container_rspec_lib_test_files() {
        let s = String::from("spec/foo/bar/car/my_lib_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_hanami_container_rspec_general_test_files() {
        let s = String::from("spec/offer_service/fulfiller_spec.rb");
        assert_eq!(is_test_file(&s), true);
    }

    // Elixer ExUnit

    #[test]
    fn is_test_file_detects_elixer_exunit_files() {
        let s = String::from("test/lib/my_awesome_app/supervisor_test.exs");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_elixer_implementation_files() {
        let s = String::from("lib/my_awesome_app/supervisor.ex");
        assert_eq!(is_test_file(&s), false);
    }

    // Jasmine JavaScript

    #[test]
    fn is_test_file_detects_js_jasmine_test_files() {
        let s = String::from("spec/foo/bar/jacked_spec.js");
        assert_eq!(is_test_file(&s), true);
    }

    // Mocha JavaScript

    #[test]
    fn is_test_file_detects_js_mocha_test_files() {
        let s = String::from("test/foo/bar/jacked.test.js");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_js_implementation_files() {
        let s = String::from("foo/bar/jacked.js");
        assert_eq!(is_test_file(&s), false);
    }

    #[test]
    fn is_test_file_detects_js_mocha_test_files_in_src_directory() {
        let s = String::from("src/foo/bar/jacked.test.js");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_js_mocha_spec_files_in_src_directory() {
        let s = String::from("src/foo/bar/jacked.spec.js");
        assert_eq!(is_test_file(&s), true);
    }
    // Python

    #[test]
    fn is_test_file_detects_python_test_files() {
        let s = String::from("test/test_toaster.py");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_python_implementation_files() {
        let s = String::from("toaster.py");
        assert_eq!(is_test_file(&s), false);
    }

    // Cucumber Ruby

    #[test]
    fn is_test_file_detects_cucumber_step_definition_files() {
        let s = String::from("features/step_definitions/project_management_steps.rb");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_cucumber_feature_files() {
        let s = String::from("features/project_management.feature");
        assert_eq!(is_test_file(&s), false);
    }

    // Java Maven JUnit

    #[test]
    fn is_test_file_detects_java_maven_junit_test_files() {
        let s = String::from("src/test/java/com/example/SomethingTest.java");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_java_implementation_files() {
        let s = String::from("src/main/java/com/example/Something.java");
        assert_eq!(is_test_file(&s), false);
    }

    // Scala ScalaTest

    #[test]
    fn is_test_file_detects_scala_scalatest_test_files() {
        let s = String::from("src/test/scala/com/example/SomethingTest.scala");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_scala_scalatest_spec_files() {
        let s = String::from("src/test/scala/com/example/SomethingSpec.scala");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_detects_scala_scalatest_suite_files() {
        let s = String::from("src/test/scala/com/example/SomethingSuite.scala");
        assert_eq!(is_test_file(&s), true);
    }

    #[test]
    fn is_test_file_does_not_detect_scala_implementation_files() {
        let s = String::from("src/main/scala/com/example/Something.scala");
        assert_eq!(is_test_file(&s), false);
    }
}
