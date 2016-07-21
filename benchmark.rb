require 'benchmark'

repeat = 1_000
# repeat = 1

puts "\nRuby Classic Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt -- lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt -- spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRuby Scatter Gather Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt_sg -- lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./alt_sg -- spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRust Classic Implementation"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -- lib/rubyfile_qq.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/aa_zz_possibles.txt | ./target/release/alt -- spec/lib/rubyfile_qr_spec.rb` }
  end
end

puts "\nRuby Classic Implementation - Discourse"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./alt -- app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./alt -- spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\nRuby Scatter Gather Implementation - Discourse"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./alt_sg -- app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./alt_sg -- spec/controllers/admin/groups_controller.rb` }
  end
end

puts "\nRust Classic Implementation - Discourse"

Benchmark.bm(15) do |x|
  x.report('For impl. file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -- app/controllers/admin/groups_controller.rb` }
  end

  x.report('For test file:') do
    repeat.times { `cat spec/fixtures/ruby_on_rails_discourse_possibles.txt | ./target/release/alt -- spec/controllers/admin/groups_controller.rb` }
  end
end

