require 'benchmark'

repeat = 1_000

puts "\n/opt/homebrew/bin/alt - aa_zz_possibiles.txt fixture"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | /opt/homebrew/bin/alt -f - lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | /opt/homebrew/bin/alt -f - spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\ntarget/release/alt - aa_zz_possibles.txt fixture"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -f - lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -f - spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\n/opt/homebrew/bin/alt - ruby_on_rails_discourse_possibles.txt fixture"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | /opt/homebrew/bin/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | /opt/homebrew/bin/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\ntarget/release/alt - ruby_on_rails_discourse_possibles.txt fixture"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end

Dir.chdir('spec/fixtures/discourse')

puts "\n/opt/homebrew/bin/alt - With cd spec/fixtures/discourse using Glob/WalkDir"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `/opt/homebrew/bin/alt app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `/opt/homebrew/bin/alt spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\ntarget/release/alt - With cd spec/fixtures/discourse using Glob/WalkDir"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `../../../target/release/alt app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `../../../target/release/alt spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\n/opt/homebrew/bin/alt - With cd spec/fixtures/discourse using find"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `find . -not -path '*/\.*' -type f | /opt/homebrew/bin/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `find . -not -path '*/\.*' -type f  | /opt/homebrew/bin/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\ntarget/release/alt - With cd spec/fixtures/discourse using find"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `find . -not -path '*/\.*' -type f | ../../../target/release/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `find . -not -path '*/\.*' -type f  | ../../../target/release/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end

Dir.chdir('../../../')
