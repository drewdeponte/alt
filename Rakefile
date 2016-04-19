# load rspec rake tasks if rspec is available
begin
  require 'rspec/core/rake_task'
  RSpec::Core::RakeTask.new(:spec)
  task default: :spec
rescue LoadError => e
  raise e unless e.message =~ /rspec\/core\/rake_task/
end
