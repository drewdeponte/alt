require 'benchmark'

repeat = 1_000

puts "\nRust Classic Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | /usr/local/bin/alt -f - lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | /usr/local/bin/alt -f - spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRust Classic Implementation - Discourse"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | /usr/local/bin/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | /usr/local/bin/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\nRust Latest Build Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -f - lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -f - spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRust Latest Build Implementation - Discourse"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -f - app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -f - spec/controllers/admin/groups_controller.rb` }
  end
end
